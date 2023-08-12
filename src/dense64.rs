use crate::{
    persist::{load_u64, load_vec_u32, load_vec_u64, save_vec, Persistent},
    rank::Rank,
    select::{Select, Select0},
    set::ImpliedSet,
    words::{rank64, select64},
};

fn lower_bound(x: u32, xs: &[u32]) -> usize {
    let mut first = 0;
    let mut count = xs.len();
    while count > 0 {
        let step = count / 2;
        let i = first + step;
        if xs[i] < x {
            first = i + 1;
            count -= step + 1;
        } else {
            count = step;
        }
    }
    first
}

fn select_from_rank(randex: &[u32], words: &[u64], index: usize) -> u64 {
    let i0 = lower_bound(index as u32, randex);
    let mut i = if i0 > 0 { i0 - 1 } else { i0 };
    while i + 1 < randex.len() && randex[i + 1] <= index as u32 {
        i += 1;
    }
    let r0 = randex[i] as usize;
    println!("i={}, x={}, index={}, r0={}, index-r0={}", i, words[i], index, r0, index - r0);
    64 * i as u64 + select64(words[i], index - r0)
}

static BLOCK_BITS: usize = 10;
//static BLOCK_SIZE: usize = 1 << BLOCK_BITS;
//static BLOCK_MASK: usize = BLOCK_SIZE - 1;

#[derive(Debug)]
pub struct Dense64 {
    size_: u64,
    words: Vec<u64>,
    randex: Vec<u32>,
    seldex: Vec<u32>,
}

impl Dense64 {
    pub fn new(size_: u64, words: &[u64]) -> Dense64 {
        std::debug_assert!(size_ / 64 <= (words.len() as u64));
        let words: Vec<u64> = Vec::from(words);

        let mut randex: Vec<u32> = Vec::new();
        randex.resize(words.len() + 1, 0);
        let mut count: u32 = 0;
        for i in 0..words.len() {
            let x = words[i];
            let m = x.count_ones();
            randex[i] = count;
            count += m as u32;
        }
        randex[words.len()] = count;

        let mut seldex: Vec<u32> = Vec::new();
        let mut i: usize = 0;
        while i << BLOCK_BITS < count as usize {
            let index: usize = i << BLOCK_BITS;
            let value = (select_from_rank(&randex, &words, index) / 64) as u32;
            seldex.push(value);
            i += 1;
        }

        Dense64 {
            size_,
            words,
            randex,
            seldex,
        }
    }
}

impl ImpliedSet for Dense64 {
    fn count(&self) -> usize {
        self.randex[self.randex.len() - 1] as usize
    }

    fn size(&self) -> u64 {
        self.size_
    }
}

impl Rank for Dense64 {
    fn rank(&self, value: u64) -> usize {
        if value >= self.size_ {
            return self.randex[self.randex.len() - 1] as usize;
        }
        let w = (value / 64) as usize;
        let b = value & 63;
        self.randex[w] as usize + rank64(self.words[w], b) as usize
    }
}

impl Select for Dense64 {
    fn select(&self, index: usize) -> u64 {
        let mut i = self.seldex[index >> BLOCK_BITS] as usize;
        while i + 1 < self.randex.len() && self.randex[i + 1] <= index as u32 {
            i += 1;
        }
        let r0 = self.randex[i] as usize;
        64 * i as u64 + select64(self.words[i], index - r0)
    }
}

impl Select0 for Dense64 {}

impl Persistent for Dense64 {
    fn save<Sink>(&self, sink: &mut Sink) -> std::io::Result<()>
    where
        Sink: std::io::Write,
    {
        sink.write_all(&self.size_.to_ne_bytes())?;
        save_vec(sink, &self.words)?;
        save_vec(sink, &self.randex)?;
        save_vec(sink, &self.seldex)?;
        Ok(())
    }

    fn load<Source>(source: &mut Source) -> std::io::Result<Box<Self>>
    where
        Source: std::io::Read,
    {
        let size_: u64 = load_u64(source)?;
        let words: Vec<u64> = load_vec_u64(source)?;
        let randex: Vec<u32> = load_vec_u32(source)?;
        let seldex: Vec<u32> = load_vec_u32(source)?;
        Ok(Box::new(Dense64 {
            size_,
            words,
            randex,
            seldex,
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufWriter;
    use std::io::Cursor;

    #[allow(unused_imports)]
    use num_traits::WrappingAdd;
    #[allow(unused_imports)]
    use num_traits::WrappingMul;

    use super::*;

    struct MiniRng {
        x: u64,
    }

    impl MiniRng {
        fn new(seed: u64) -> MiniRng {
            MiniRng { x: seed }
        }

        fn rnd(&mut self) -> u64 {
            self.x = self.x.wrapping_mul(2862933555777941757u64);
            self.x = self.x.wrapping_add(3037000493u64);
            self.x
        }
    }

    #[test]
    fn test_rank_1() {
        let n = 1024;
        let m = n * 64;
        let k = 100;
        let mut bits = Vec::new();
        let mut words = Vec::new();
        words.resize(n, 0);
        let mut rng = MiniRng::new(0xfbdb8b2bcc6674b8u64);
        for _i in 0..k {
            let x: u64 = (rng.rnd() ^ (rng.rnd() << 32) ^ (rng.rnd() >> 32)) % (m as u64);
            bits.push(x);
            let w = x / 64;
            let b = x & 63;
            words[w as usize] |= 1 << b;
        }
        bits.sort();
        println!("{:?}", bits);

        let r = Dense64::new(m as u64, &words);
        for i in 0..k {
            let x = bits[i];
            println!("{} {}", i, x);
            let j = r.rank(x);
            assert_eq!(j, i);
            assert!(r.contains(x));
            assert_eq!(r.access_and_rank(x), (i, true));
            assert_eq!(r.access_and_rank(x + 1), (i + 1, false));
        }
        assert_eq!(r.rank(m as u64), k);
    }

    #[test]
    fn test_select_1() {
        let n = 1024;
        let m = n * 64;
        let k = 100;
        let mut bits = Vec::new();
        let mut words = Vec::new();
        words.resize(n, 0);
        let mut rng = MiniRng::new(0xfbdb8b2bcc6674b8u64);
        for _i in 0..k {
            let x: u64 = (rng.rnd() ^ (rng.rnd() << 32) ^ (rng.rnd() >> 32)) % (m as u64);
            bits.push(x);
            let w = x / 64;
            let b = x & 63;
            words[w as usize] |= 1 << b;
        }
        bits.sort();
        println!("{:?}", bits);

        let r = Dense64::new(m as u64, &words);
        for i in 0..k {
            assert_eq!(r.select(i), bits[i]);
        }
    }

    #[test]
    fn test_select_2() {
        let m = 1024 * 1024;
        let n = m / 64;
        let k = 65536;
        let mut bits = Vec::new();
        let mut words = Vec::new();
        words.resize(n, 0);
        let mut rng = MiniRng::new(0xfbdb8b2bcc6674b8u64);
        for _i in 0..k {
            let x: u64 = (rng.rnd() ^ (rng.rnd() << 32) ^ (rng.rnd() >> 32)) % (m as u64);
            bits.push(x);
            let w = x / 64;
            let b = x & 63;
            words[w as usize] |= 1 << b;
        }
        bits.sort();
        bits.dedup();

        let r = Dense64::new(m as u64, &words);
        for i in 0..bits.len() {
            assert_eq!(r.select(i), bits[i]);
        }
    }

    #[test]
    fn test_select_3() {
        let m = 1024 * 1024;
        let n = m / 64;
        let k = 1024;
        let mut bits = Vec::new();
        let mut words = Vec::new();
        words.resize(n, 0);
        let mut rng = MiniRng::new(0xfbdb8b2bcc6674b8u64);
        for _i in 0..k {
            let x: u64 = (rng.rnd() ^ (rng.rnd() << 32) ^ (rng.rnd() >> 32)) % (m as u64);
            bits.push(x);
            let w = x / 64;
            let b = x & 63;
            words[w as usize] |= 1 << b;
        }
        bits.sort();
        bits.dedup();
        //println!("{:?}", bits);

        let r = Dense64::new(m as u64, &words);
        for i in 0..bits.len() {
            //println!("i={}, x={}", i, bits[i]);
            assert_eq!(r.select(i), bits[i]);
        }
    }

    #[test]
    fn test_select0_1() {
        let words: Vec<u64> = vec![0b100111u64];
        let r = Dense64::new(6, &words);
        assert_eq!(r.rank_0(0), 0);
        assert_eq!(r.rank_0(1), 0);
        assert_eq!(r.rank_0(2), 0);
        assert_eq!(r.rank_0(3), 0);
        assert_eq!(r.rank_0(4), 1);
        assert_eq!(r.rank_0(5), 2);
        assert_eq!(r.rank_0(6), 2);
        assert_eq!(r.select(0), 0);
        assert_eq!(r.select(1), 1);
        assert_eq!(r.select(2), 2);
        assert_eq!(r.select(3), 5);
        assert_eq!(r.select_0(0), 3);
        assert_eq!(r.select_0(1), 4);
    }

    #[test]
    fn test_load_and_save_1() {
        let m = 1024 * 1024;
        let n = m / 64;
        let k = 1024;
        let mut bits = Vec::new();
        let mut words = Vec::new();
        words.resize(n, 0);
        let mut rng = MiniRng::new(0xfbdb8b2bcc6674b8u64);
        for _i in 0..k {
            let x: u64 = (rng.rnd() ^ (rng.rnd() << 32) ^ (rng.rnd() >> 32)) % (m as u64);
            bits.push(x);
            let w = x / 64;
            let b = x & 63;
            words[w as usize] |= 1 << b;
        }
        bits.sort();
        bits.dedup();
        //println!("{:?}", bits);

        let r = Dense64::new(m as u64, &words);

        let mut buf = BufWriter::new(Vec::new());
        r.save(&mut buf).expect("save failed");
        let bytes = buf.into_inner().expect("failed to get bytes");
        let mut cursor = Cursor::new(bytes);
        let s = Dense64::load(&mut cursor).expect("load failed");

        for i in 0..k {
            let x = bits[i];
            println!("{} {}", i, x);
            let j = s.rank(x);
            assert_eq!(j, i);
            let y = s.select(i);
            assert_eq!(y, x);
        }
    }
}
