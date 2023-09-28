use crate::{
    rank::Rank,
    select::Select,
    set::ImpliedSet,
};

pub struct Sorted {
    elements: Vec<u64>,
}

impl Sorted {
    pub fn new(elements: &[u64]) -> Sorted {
        Sorted {
            elements: Vec::from(elements),
        }
    }
}

impl ImpliedSet for Sorted {
    fn count(&self) -> usize {
        self.elements.len()
    }

    fn size(&self) -> u64 {
        match self.elements.last() {
            None => 0,
            Some(x) => x + 1
        }
    }
}

impl Rank for Sorted {
    fn rank(&self, value: u64) -> usize {
        if value >= self.size() {
            return self.count();
        }
        let mut first = 0;
        let mut count = self.count();
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

impl Select for Sorted {
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

        let r = Sorted::new( &xs);
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
