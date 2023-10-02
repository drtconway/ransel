//! A module for storing unsigned integers of different widths.

use crate::persist::{Persistent, save_vec, load_vec_u64, load_usize};

/// A vector of unsigned integers.
///
/// Values are stored in a vector of words.
///
pub struct IntVec {
    b: usize,
    n: usize,
    words: Vec<u64>,
}

impl IntVec {
    /// Create an empty vector for integers of the requested width.
    pub fn new(b: usize) -> IntVec {
        IntVec {
            b,
            n: 0,
            words: Vec::new(),
        }
    }

    /// Return the length of the vector.
    pub fn len(&self) -> usize {
        self.n
    }

    /// Append a value to the vector.
    pub fn push(&mut self, value: u64) {
        let idx = self.n;
        self.n += 1;
        let end_bit = (idx + 1) * self.b;
        while self.words.len() * 64 < end_bit {
            self.words.push(0);
        }
        self.set(idx, value)
    }

    /// Get an element from the vector
    pub fn get(&self, idx: usize) -> u64 {
        assert!(idx < self.len());
        let begin_bit = idx * self.b;
        let end_bit = (idx + 1) * self.b;

        let begin_word = begin_bit / 64;
        let begin_bit_in_word = begin_bit & 63;
        let end_word = end_bit / 64;
        let end_bit_in_word = end_bit & 63;

        if end_word != begin_word && end_bit_in_word > 0 {
            // Spanning 2 words
            let low_bits = self.words[begin_word] >> begin_bit_in_word;
            let high_bits = self.words[end_word] & ((1u64 << end_bit_in_word) - 1);
            low_bits | (high_bits << (self.b - end_bit_in_word))
        } else {
            // All the bits are in 1 word
            let w = self.words[begin_word];
            let mask = (1u64 << self.b) - 1;
            (w >> begin_bit_in_word) & mask
        }
    }

    /// Set an element in the vector.
    pub fn set(&mut self, idx: usize, value: u64) {
        assert!(idx < self.len());
        assert!(value < (1u64 << self.b));

        let begin_bit = idx * self.b;
        let end_bit = (idx + 1) * self.b;

        let begin_word = begin_bit / 64;
        let begin_bit_in_word = begin_bit & 63;
        let end_word = end_bit / 64;
        let end_bit_in_word = end_bit & 63;

        if end_word != begin_word && end_bit_in_word > 0 {
            // Spanning 2 words
            let low_word_bits = self.b - end_bit_in_word;
            let low_bits_mask = (1u64 << low_word_bits) - 1;
            let low_bits = value & low_bits_mask;
            let high_bits = value >> low_word_bits;
            let low_word_mask = !(low_bits_mask << begin_bit_in_word);
            let low_word = self.words[begin_word];
            self.words[begin_word] = (low_word & low_word_mask) | (low_bits << begin_bit_in_word);
            let high_word_mask = !((1u64 << end_bit_in_word) - 1);
            let high_word = self.words[end_word];
            self.words[end_word] = (high_word & high_word_mask) | high_bits;
        } else {
            // All the bits are in 1 word
            let w = self.words[begin_word];
            let mask = !(((1u64 << self.b) - 1) << begin_bit_in_word);
            self.words[begin_word] = (w & mask) | (value << begin_bit_in_word);
        }
    }
}

impl Persistent for IntVec {
    fn load<Source>(source: &mut Source) -> std::io::Result<Box<Self>>
    where
        Source: std::io::Read,
    {
        let b: usize = load_usize(source)?;
        let n: usize = load_usize(source)?;
        let words: Vec<u64> = load_vec_u64(source)?;
        Ok(Box::new(IntVec {b, n, words}))
    }

    fn save<Sink>(&self, sink: &mut Sink) -> std::io::Result<()>
    where
        Sink: std::io::Write,
    {
        sink.write_all(&self.b.to_ne_bytes())?;
        sink.write_all(&self.n.to_ne_bytes())?;
        save_vec(sink, &self.words)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use random::Source;

    #[test]
    fn test_intvec_1() {
        let mut xs: Vec<u64> = Vec::new();
        let b = 4;
        let mask = (1u64 << b) - 1;
        let n = 1000;
        let mut rng = random::default(19);
        for _i in 0..n {
            xs.push(rng.read_u64() & mask);
        }

        let mut v = IntVec::new(b);
        for x in xs.iter() {
            v.push(*x);
        }
        assert!(v.words.len() * 64 >= n * b);
        for i in 0..n {
            assert_eq!(v.get(i), xs[i]);
        }
    }

    #[test]
    fn test_intvec_2() {
        let mut xs: Vec<u64> = Vec::new();
        let b = 7;
        let mask = (1u64 << b) - 1;
        let n = 1000;
        let mut rng = random::default(19);
        for _i in 0..n {
            xs.push(rng.read_u64() & mask);
        }

        let mut v = IntVec::new(b);
        for x in xs.iter() {
            v.push(*x);
        }
        assert!(v.words.len() * 64 >= n * b);
        for i in 0..n {
            assert_eq!(v.get(i), xs[i]);
        }
    }

    #[test]
    fn test_intvec_3() {
        let mut xs: Vec<u64> = Vec::new();
        let b = 47;
        let mask = (1u64 << b) - 1;
        let n = 1000;
        let mut rng = random::default(19);
        for _i in 0..n {
            xs.push(rng.read_u64() & mask);
        }

        let mut v = IntVec::new(b);
        for x in xs.iter() {
            v.push(*x);
        }
        assert!(v.words.len() * 64 >= n * b);
        for i in 0..n {
            assert_eq!(v.get(i), xs[i]);
        }
    }

    #[test]
    fn test_intvec_4() {
        let mut xs: Vec<u64> = Vec::new();
        let b = 63;
        let mask = (1u64 << b) - 1;
        let n = 1000;
        let mut rng = random::default(19);
        for _i in 0..n {
            xs.push(rng.read_u64() & mask);
        }

        let mut v = IntVec::new(b);
        for x in xs.iter() {
            v.push(*x);
        }
        assert!(v.words.len() * 64 >= n * b);
        for i in 0..n {
            assert_eq!(v.get(i), xs[i]);
        }
    }
}
