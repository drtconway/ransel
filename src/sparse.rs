use crate::{bitvec::BitVec, intvec::IntVec, dense64::Dense64, set::ImpliedSet, rank::Rank, select::Select, select::Select0};

pub struct Sparse {
    b: usize,
    n: usize,
    d: usize,
    hi: Dense64,
    lo: IntVec
}

impl Sparse {
    pub fn new(b: usize, n: usize, elements: &[u64]) -> Sparse {
        let d = ((1 << b) as f64 / (1.44 * n as f64)).log2() as usize;
        let m = (1 << d) - 1;
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
        hi_bits.push(true);
        Sparse { b, n, d, hi: Dense64::new(hi_bits.len() as u64, hi_bits.as_words()), lo: low_bits }
    }
}

impl ImpliedSet for Sparse {
    fn size(&self) -> u64 {
        1 << self.b
    }

    fn count(&self) -> usize {
        self.n
    }
}

impl Rank for Sparse {
    fn rank(&self, value: u64) -> usize {
        if value >= (1 << self.b) {
            return self.count();
        }
        let hi = (value >> self.d) as usize;
        let lo = value & ((1 << self.d) - 1);
        let r0 = self.hi.select(hi) as usize - hi;
        let r1 = self.hi.select(hi + 1) as usize - (hi + 1);
        let mut r = r0;
        while r < r1 && self.lo.get(r) < lo {
            r += 1;
        }
        r as usize
    }
}

impl Select for Sparse {
    fn select(&self, index: usize) -> u64 {
        let z = self.hi.select_0(index);
        let hi = self.hi.rank(z) as u64 - 1;
        println!("index={}, z={}, hi={}, lo={}", index, z, hi, self.lo.get(index));
        (hi << self.d) | self.lo.get(index)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

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
        let m = (1 << b) - 1;
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
        assert_eq!(s.b, b);
        assert_eq!(s.n, n);
        assert_eq!(s.d, 9);
        assert_eq!(s.hi.count(), 2048);
        assert_eq!(s.hi.size(), 2048 + n as u64);
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
        assert_eq!(s.hi.count(), 2048);
        assert_eq!(s.hi.size(), 2048 + n as u64);
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
        assert_eq!(s.hi.count(), 2048);
        assert_eq!(s.hi.size(), 2048 + n as u64);
        assert_eq!(s.lo.len(), n);
        for i in 0..xs.len() {
            let x = xs[i];
            let y = s.select(i);
            println!("i={}, x={}, y={}", i, x, y);
            assert_eq!(y, x);
        }
    }
}
