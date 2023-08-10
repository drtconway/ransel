use crate::{set::ImpliedSet, rank::Rank, select::Select};


pub struct NaiveSparse {
    b: usize,
    elements: Vec<u64>,
    toc: Vec<usize>
}

static B: usize = 10;

impl NaiveSparse {
    pub fn new(b: usize, elements: &[u64]) -> NaiveSparse {
        assert!(b > B);
        let n = 1 << b;
        let m = n >> B;
        let s = b - B;
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
        NaiveSparse { b, elements: Vec::from(elements), toc }
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
            let x= xs[i];
            println!("xs[i]={}", x);
            let j = r.rank(x);
            assert_eq!(j, i);
            assert!(r.contains(x));
            assert_eq!(r.access_and_rank(x), (i, true));
        }
    }
}