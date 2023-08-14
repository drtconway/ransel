use crate::{
    bitvec::BitVec,
    dense64::Dense64,
    intvec::IntVec,
    persist::{load_usize, Persistent},
    rank::Rank,
    select::Select,
    select::Select0,
    set::ImpliedSet,
};

pub struct Sparse {
    b: usize,
    n: usize,
    d: usize,
    hi: Dense64,
    lo: IntVec,
}

impl Sparse {
    pub fn new(b: usize, n: usize, elements: &[u64]) -> Sparse {
        let d = ((1u64 << b) as f64 / (1.44 * n as f64)).log2() as usize;
        let m = (1u64 << d) - 1;
        let mut hi_cursor = 0;
        let mut hi_bits = BitVec::new();
        let mut low_bits = IntVec::new(d);
        for x in elements {
            let hi = *x >> d;
            let lo = *x & m;
            while hi_cursor <= hi {
                hi_bits.push(true);
                hi_cursor += 1;
            }
            hi_bits.push(false);
            low_bits.push(lo);
        }
        let j = 1u64 << (b - d);
        while hi_cursor < j {
            hi_bits.push(true);
            hi_cursor += 1;
        }
        hi_bits.push(true);
        Sparse {
            b,
            n,
            d,
            hi: Dense64::new(hi_bits.len() as u64, hi_bits.as_words()),
            lo: low_bits,
        }
    }
}

impl ImpliedSet for Sparse {
    fn size(&self) -> u64 {
        1u64 << self.b
    }

    fn count(&self) -> usize {
        self.n
    }
}

impl Rank for Sparse {
    fn rank(&self, value: u64) -> usize {
        if value >= (1u64 << self.b) {
            return self.count();
        }
        let hi = (value >> self.d) as usize;
        let lo = value & ((1u64 << self.d) - 1);
        let r0 = self.hi.select(hi) as usize - hi;
        let r1 = self.hi.select(hi + 1) as usize - (hi + 1);
        let mut r = r0;
        while r < r1 && self.lo.get(r) < lo {
            r += 1;
        }
        r as usize
    }

    fn rank_2(&self, value_1: u64, value_2: u64) -> (usize, usize) {
        let hi_1 = (value_1 >> self.d) as usize;
        let hi_2 = (value_2 >> self.d) as usize;
        if hi_1 != hi_2 || value_2 < value_1 || value_2 >= (1u64 << self.d) {
            return (self.rank(value_1), self.rank(value_2));
        }
        
        let hi = hi_1;
        let lo_1 = value_1 & ((1u64 << self.d) - 1);
        let lo_2 = value_2 & ((1u64 << self.d) - 1);
        let r0 = self.hi.select(hi) as usize - hi;
        let r1 = self.hi.select(hi + 1) as usize - (hi + 1);

        let mut r_a = r0;
        while r_a < r1 && self.lo.get(r_a) < lo_1 {
            r_a += 1;
        }
        let mut r_b = r_a;
        while r_b < r1 && self.lo.get(r_b) < lo_2 {
            r_b += 1;
        }
        (r_a, r_b)
    }
}

impl Select for Sparse {
    fn select(&self, index: usize) -> u64 {
        let z = self.hi.select_0(index);
        let hi = self.hi.rank(z) as u64 - 1;
        (hi << self.d) | self.lo.get(index)
    }
}

impl Persistent for Sparse {
    fn save<Sink>(&self, sink: &mut Sink) -> std::io::Result<()>
    where
        Sink: std::io::Write,
    {
        sink.write_all(&self.b.to_ne_bytes())?;
        sink.write_all(&self.n.to_ne_bytes())?;
        sink.write_all(&self.d.to_ne_bytes())?;
        self.hi.save(sink)?;
        self.lo.save(sink)?;
        Ok(())
    }

    fn load<Source>(source: &mut Source) -> std::io::Result<Box<Self>>
    where
        Source: std::io::Read,
    {
        let b: usize = load_usize(source)?;
        let n: usize = load_usize(source)?;
        let d: usize = load_usize(source)?;
        let hi: Dense64 = *(Dense64::load(source)?);
        let lo: IntVec = *(IntVec::load(source)?);
        Ok(Box::new(Sparse { b, n, d, hi, lo }))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use flate2;

    #[allow(unused_imports)]
    use num_traits::WrappingAdd;
    #[allow(unused_imports)]
    use num_traits::WrappingMul;

    use crate::set::ImpliedSet;

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

    fn make_set(b: usize, n: usize) -> Vec<u64> {
        let m = (1u64 << b) - 1;
        let mut rng = MiniRng::new(0xfbdb8b2bcc6674b8u64);
        let mut s = HashSet::new();
        while s.len() < n {
            let x: u64 = (rng.rnd() ^ (rng.rnd() << 32) ^ (rng.rnd() >> 32)) & m;
            s.insert(x);
        }
        let mut res = Vec::from_iter(s.iter().map(|v| *v));
        res.sort();
        res
    }

    #[test]
    fn test_sparse_build_1() {
        let b: usize = 20;
        let n: usize = 1024;
        let xs = make_set(b, n);
        let s = Sparse::new(b, n, &xs);
        assert_eq!(s.size(), 1u64 << b);
        assert_eq!(s.count(), n);
        assert_eq!(s.b, b);
        assert_eq!(s.n, n);
        assert_eq!(s.d, 9);
        assert_eq!(s.hi.count(), 2049);
        assert_eq!(s.hi.size(), 2049 + n as u64);
        assert_eq!(s.lo.len(), n);
    }

    #[test]
    fn test_sparse_rank_1() {
        let b: usize = 20;
        let n: usize = 1024;
        let xs = make_set(b, n);
        let s = Sparse::new(b, n, &xs);
        assert_eq!(s.b, b);
        assert_eq!(s.n, n);
        assert_eq!(s.d, 9);
        assert_eq!(s.hi.count(), 2049);
        assert_eq!(s.hi.size(), 2049 + n as u64);
        assert_eq!(s.lo.len(), n);
        for i in 0..xs.len() {
            let x = xs[i];
            let r = s.rank(x);
            assert_eq!(r, i);
        }
    }

    #[test]
    fn test_sparse_select_1() {
        let b: usize = 20;
        let n: usize = 1024;
        let xs = make_set(b, n);
        let s = Sparse::new(b, n, &xs);
        assert_eq!(s.b, b);
        assert_eq!(s.n, n);
        assert_eq!(s.d, 9);
        assert_eq!(s.hi.count(), 2049);
        assert_eq!(s.hi.size(), 2049 + n as u64);
        assert_eq!(s.lo.len(), n);
        for i in 0..xs.len() {
            let x = xs[i];
            let y = s.select(i);
            assert_eq!(y, x);
        }
    }

    #[test]
    fn test_big_sparse() {
        let b: usize = 50;
        let mut xs: Vec<u64> = Vec::new();
        {
            let file = File::open("./test_data/b_50_integers.txt.gz").expect("failed to open file");
            let buffer = BufReader::new(file);
            let gzip = flate2::bufread::GzDecoder::new(buffer);
            let reader = BufReader::new(gzip);
            for line in reader.lines() {
                let txt = line.expect("failed to read line");
                let x:u64 = txt.parse().expect("not an integer");
                xs.push(x);
            }
        }
        let n = xs.len();
        let s = Sparse::new(b, n, &xs);
        for i in 0..xs.len() {
            assert_eq!(s.rank(xs[i]), i);
            assert_eq!(s.select(i), xs[i]);
        }
        {
            let (r,c) = s.access_and_rank(0x3FFBC2C2BC000u64);
            assert_eq!(c, true);
            assert_eq!(r, s.count() - 1);
        }
        {
            let (r,c) = s.access_and_rank(0x3FFC9480BC000u64);
            assert_eq!(c, false);
            assert_eq!(r, s.count());
        }
        }
}
