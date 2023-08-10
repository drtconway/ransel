
pub struct BitVec {
    size: usize,
    words: Vec<u64>
}

impl BitVec {
    pub fn new() -> BitVec {
        BitVec {size: 0, words: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn get(&self, index: usize) -> bool {
        let w = index >> 6;
        let b = index & 63;
        (self.words[w] >> b) & 1 == 1
    }

    pub fn set(&mut self, index: usize, bit: bool) {
        let w = index >> 6;
        let b = index & 63;
        let m: u64 = 0xffffffff_ffffffffu64 ^ (1 << b);
        self.words[w] &= m;
        self.words[w] |= (bit as u64) << b;
    }

    pub fn push(&mut self, bit: bool) {
        let index = self.size;
        self.size += 1;
        let w = index >> 6;
        while self.words.len() <= w {
            self.words.push(0);
        }
        self.set(index, bit);
    }

    pub fn as_words(&self) -> &[u64] {
        &self.words
    }
}