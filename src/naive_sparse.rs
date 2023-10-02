//! A simple sparse set based on an indexed sorted vector.

use crate::{
    persist::{load_usize, load_vec_u64, save_vec, Persistent, load_vec_usize},
    rank::Rank,
    select::Select,
    set::ImpliedSet,
};

/// A simple index sparse set representation.
/// 
/// This representation is not succinct - it stores the elements of the set
/// in a sorted vector, and computes a table-of-contents to accelerate the
/// binary search based implementation of `rank`.
/// 
/// It uses a 1024 entry table of contents.
/// 
pub struct NaiveSparse {
    b: usize,
    elements: Vec<u64>,
    toc: Vec<usize>,
}

static B: usize = 10;

impl NaiveSparse {
    /// Create a naive sparse set for values with `b` bits.
    pub fn new(b: usize, elements: &[u64]) -> NaiveSparse {
        assert!(b > B);
        let m = 1 << B;
        let s = if b > B { b - B } else { 0 };
        let mut toc = Vec::new();
        toc.resize(m + 1, 0);
        for i in 0..elements.len() {
            let x = elements[i];
            let v = (x >> s) as usize;
            toc[v] += 1;
        }
        let mut count = 0;
        for i in 0..toc.len() {
            let c = toc[i];
            toc[i] = count;
            count += c;
        }
        NaiveSparse {
            b,
            elements: Vec::from(elements),
            toc,
        }
    }
}

impl ImpliedSet for NaiveSparse {
    fn count(&self) -> usize {
        self.elements.len()
    }

    fn size(&self) -> u64 {
        1 << self.b
    }
}

impl Rank for NaiveSparse {
    fn rank(&self, value: u64) -> usize {
        if value >= (1 << self.b) {
            return self.count();
        }
        let s = self.b - B;
        let i = (value >> s) as usize;
        let mut first = self.toc[i];
        let last = self.toc[i + 1];
        let mut count = last - first;
        while count > 0 {
            let step = count / 2;
            let i = first + step;
            if self.elements[i] < value {
                first = i + 1;
                count -= step + 1;
            } else {
                count = step;
            }
        }
        first
    }
}

impl Select for NaiveSparse {
    fn select(&self, index: usize) -> u64 {
        self.elements[index]
    }
}

impl Persistent for NaiveSparse {
    fn save<Sink>(&self, sink: &mut Sink) -> std::io::Result<()>
    where
        Sink: std::io::Write,
    {
        sink.write_all(&self.b.to_ne_bytes())?;
        save_vec(sink, &self.elements)?;
        save_vec(sink, &self.toc)?;
        Ok(())
    }

    fn load<Source>(source: &mut Source) -> std::io::Result<Box<Self>>
    where
        Source: std::io::Read,
    {
        let b: usize = load_usize(source)?;
        let elements = load_vec_u64(source)?;
        let toc = load_vec_usize(source)?;
        Ok(Box::new(NaiveSparse { b, elements, toc }))
    }
}

#[cfg(test)]
mod tests {
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
        let b: usize = 20;
        let m: u64 = (1 << b) - 1;
        let k: usize = 1024;
        let mut xs: Vec<u64> = Vec::new();
        let mut rng = MiniRng::new(0xfbdb8b2bcc6674b9u64);
        for _i in 0..k {
            let x: u64 = (rng.rnd() ^ (rng.rnd() << 32) ^ (rng.rnd() >> 32)) & m;
            xs.push(x);
        }
        xs.sort();
        xs.dedup();
        println!("xs={:?}", xs);

        let r = NaiveSparse::new(b, &xs);
        for i in 0..xs.len() {
            let x = xs[i];
            println!("xs[i]={}", x);
            let j = r.rank(x);
            assert_eq!(j, i);
            assert!(r.contains(x));
            assert_eq!(r.access_and_rank(x), (i, true));
        }
    }
}
