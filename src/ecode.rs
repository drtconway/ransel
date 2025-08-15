pub struct EnumCodeSet {
    n: u64,
    k: u64,
}

impl EnumCodeSet {
    /// Returns the number of bits required to store the maximum rank.
    pub fn bits(&self) -> u32 {
        let total = binom(self.n, self.k);
        if total == 0 {
            0
        } else {
            64 - total.saturating_sub(1).leading_zeros()
        }
    }
    
    pub fn new(n: u64, k: u64) -> Self {
        EnumCodeSet { n, k }
    }

    pub fn rank(&self, x: u64) -> u64 {
        assert_eq!((x & ((1 << self.n) - 1)).count_ones() as u64, self.k);
        let mut r = 0;
        let mut k = self.k;
        for i in (0..self.n).rev() {
            if (x & (1 << i)) != 0 {
                r += binom(i, k);
                k -= 1;
                if k == 0 {
                    break;
                }
            }
        }
        r
    }

    /// Returns the i-th smallest n-bit number with k one bits.
    pub fn select(&self, i: u64) -> u64 {
        let mut result = 0u64;
        let mut k = self.k;
        let mut i = i;
        for j in (0..self.n).rev() {
            if k == 0 {
                break;
            }
            let b = binom(j, k);
            if i < b {
                // Place a 0 at this position
                continue;
            } else {
                // Place a 1 at this position
                result |= 1 << j;
                i -= b;
                k -= 1;
            }
        }
        result
    }
}

fn binom(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    let mut res = 1u64;
    for i in 0..k {
        res = res * (n - i) as u64 / (i + 1) as u64;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binom_1() {
        assert_eq!(binom(5, 2), 10);
        assert_eq!(binom(10, 7), 120);
        assert_eq!(binom(3, 0), 1);
        assert_eq!(binom(6, 6), 1);
    }

    #[test]
    fn rank_1() {
        let s = EnumCodeSet::new(20, 1);
        assert_eq!(s.rank(1 << 0), 0);
        assert_eq!(s.rank(1 << 1), 1);
        assert_eq!(s.rank(1 << 2), 2);
        assert_eq!(s.rank(1 << 3), 3);
        assert_eq!(s.rank(1 << 18), 18);
        assert_eq!(s.rank(1 << 19), 19);
    }

    #[test]
    fn select_1() {
        let s = EnumCodeSet::new(5, 2);
        // All 5-bit numbers with 2 ones, in order:
        // 0b00011, 0b00101, 0b00110, 0b01001, 0b01010,
        // 0b01100, 0b10001, 0b10010, 0b10100, 0b11000
        let expected = [
            0b00011, 0b00101, 0b00110, 0b01001, 0b01010,
            0b01100, 0b10001, 0b10010, 0b10100, 0b11000,
        ];
        for (i, &val) in expected.iter().enumerate() {
            let i = i as u64;
            assert_eq!(s.select(i), val);
            assert_eq!(s.rank(val), i);
        }
    }
}

