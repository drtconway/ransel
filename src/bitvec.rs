//! A bit vector represented as a vector of 64 bit words.

/// A bit vector represented by a vector of 64 bit words.
#[derive(Clone)]
pub struct BitVec {
    size: usize,
    words: Vec<u64>
}

impl BitVec {
    /// Return a new bit vector.
    pub fn new() -> BitVec {
        BitVec {size: 0, words: Vec::new() }
    }

    /// Return the length of the bit vector.
    pub fn len(&self) -> usize {
        self.size
    }

    /// Retrieve a bit from the bit vector.
    pub fn get(&self, index: usize) -> bool {
        let w = index >> 6;
        let b = index & 63;
        (self.words[w] >> b) & 1 == 1
    }

    /// Set a bit in the bit vector.
    pub fn set(&mut self, index: usize, bit: bool) {
        let w = index >> 6;
        let b = index & 63;
        let m: u64 = 0xffffffff_ffffffffu64 ^ (1 << b);
        self.words[w] &= m;
        self.words[w] |= (bit as u64) << b;
    }

    /// Add a bit to the end of the bit vector.
    pub fn push(&mut self, bit: bool) {
        let index = self.size;
        self.size += 1;
        let w = index >> 6;
        while self.words.len() <= w {
            self.words.push(0);
        }
        self.set(index, bit);
    }

    /// Return the underlying vector of words.
    pub fn as_words(&self) -> &[u64] {
        &self.words
    }
}