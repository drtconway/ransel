use bitintr::Pdep;

pub fn rank64(x: u64, pos: u64) -> u64 {
    if pos < 64 {
        (x & ((1 << pos) - 1)).count_ones() as u64
    } else {
        x.count_ones() as u64
    }
}

pub fn select64(x: u64, idx: usize) -> u64 {
    (1 << idx).pdep(x).trailing_zeros() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank64_1() {
        let x: u64 = 0xdeadbeefdeadbeefu64;
        // 11011110_10101101_10111110_11101111_11011110_10101101_10111110_11101111
        assert_eq!(rank64(x, 0), 0);
        assert_eq!(rank64(x, 1), 1);
        assert_eq!(rank64(x, 4), 4);
        assert_eq!(rank64(x, 5), 4);
        assert_eq!(rank64(x, 8), 7);
        assert_eq!(rank64(x, 16), 13);
        assert_eq!(rank64(x, 32), 24);
        assert_eq!(rank64(x, 48), 37);
        assert_eq!(rank64(x, 63), 47);
        assert_eq!(rank64(x, 64), 48);
    }

    #[test]
    fn test_select64_1() {
        let x: u64 = 0xdeadbeefdeadbeefu64;
        // 11011110_10101101_10111110_11101111_11011110_10101101_10111110_11101111
        assert_eq!(select64(x, 0), 0);
        assert_eq!(select64(x, 1), 1);
        assert_eq!(select64(x, 2), 2);
        assert_eq!(select64(x, 3), 3);
        assert_eq!(select64(x, 4), 5);
        assert_eq!(select64(x, 5), 6);
        assert_eq!(select64(x, 47), 63);
    }
}
