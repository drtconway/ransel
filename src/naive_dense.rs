//! A simple dense set based on an un-indexed bit vector.

use crate::{bitvec::BitVec, rank::Rank, set::ImpliedSet, select::Select, words::select64};

/// A sparse set based on an un-indexed bit vector.
pub struct NaiveDense {
    bit_count: usize,
    bits: BitVec,
}

impl NaiveDense {
    /// Create a new set from a bit vector.
    pub fn new(bits: BitVec) -> NaiveDense {
        let mut bit_count: usize = 0;
        for w in bits.as_words() {
            bit_count += w.count_ones() as usize;
        }
        NaiveDense { bit_count, bits }
    }
}

impl ImpliedSet for NaiveDense {
    fn size(&self) -> u64 {
        self.bits.len() as u64
    }

    fn count(&self) -> usize {
        self.bit_count
    }
}

impl Rank for NaiveDense {
    fn rank(&self, value: u64) -> usize {
        let n = self.bits.as_words().len();
        let w = (value / 64) as usize;
        let m = (1u64 << (value & 63)) - 1;
        if w >= n {
            return self.count();
        }
        let mut cumulative: usize = 0;
        for i in 0..w {
            cumulative += self.bits.as_words()[i].count_ones() as usize;
        }
        cumulative + (self.bits.as_words()[w] & m).count_ones() as usize
    }

    fn rank_2(&self, value_1: u64, value_2: u64) -> (usize, usize) {
        assert!(value_1 < value_2);
        let n = self.bits.as_words().len();
        let w1 = (value_1 / 64) as usize;
        let m1 = (1u64 << (value_2 & 63)) - 1;
        let w2 = (value_1 / 64) as usize;
        let m2 = (1u64 << (value_2 & 63)) - 1;
        if w1 >= n {
            let c = self.count();
            return (c, c);
        }
        let mut cumulative: usize = 0;
        for i in 0..w1 {
            cumulative += self.bits.as_words()[i].count_ones() as usize;
        }
        let r1 = cumulative + (self.bits.as_words()[w1] & m1).count_ones() as usize;
        for i in w1..w2 {
            cumulative += self.bits.as_words()[i].count_ones() as usize;
        }
        let r2 = cumulative + (self.bits.as_words()[w2] & m2).count_ones() as usize;
        (r1, r2)
    }
}

impl Select for NaiveDense {
    fn select(&self, index: usize) -> u64 {
        assert!(index < self.count());
        let words = self.bits.as_words();
        let mut cumulative = 0;
        for i in 0..words.len() {
            let c = words[i].count_ones() as usize;
            if cumulative + c > index {
                let j = index - cumulative;
                let p = select64(words[i], j);
                return 64 * i as u64 + p;
            }
            cumulative += c;
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex_bits(xs: &[u64]) -> BitVec {
        let mut res = BitVec::new();
        for x in xs.iter() {
            let mut x = *x;
            for _ in 0..64 {
                res.push((x & 1) == 1);
                x >>= 1;
            }
        }
        res
    }

    #[test]
    fn test_rank_1() {
        let dat = [0x634b9340deec8469, 0x84eb72e372e6a42f, 0x887223eead889e46, 0x60e42e378e9549c8, 0x86aaf4f00e8c7b16, 0x0ece3ae2b0fc440c, 0xcb1e4df954f381be, 0xb90b639ce82a8329];
        let bits = hex_bits(&dat);
        let nd = NaiveDense::new(bits.clone());
        assert_eq!(nd.count(), 248);
        assert_eq!(nd.size(), 512);

        let mut t = 0;
        let mut ones = Vec::new();
        for i in 0..512 {
            assert_eq!(nd.rank(i), t);
            if bits.get(i as usize) {
                t += 1;
                ones.push(i);
            }
        }
        assert_eq!(ones.len(), 248);
        for i in 0..248 {
            assert_eq!(nd.select(i), ones[i]);
        }
    }
}