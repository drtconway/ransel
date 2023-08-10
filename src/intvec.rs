enum Integers {
    U8(Vec<u8>),
    U16(Vec<u16>),
    U24(Vec<u8>, Vec<u16>),
    U32(Vec<u32>),
    U48(Vec<u16>, Vec<u32>),
    U64(Vec<u64>),
}

pub struct IntVec {
    items: Integers,
}

impl IntVec {
    pub fn new(b: usize) -> IntVec {
        if b <= 8 {
            IntVec::new8()
        } else if b <= 16 {
            IntVec::new16()
        } else if b <= 24 {
            IntVec::new24()
        } else if b <= 32 {
            IntVec::new32()
        } else if b <= 48 {
            IntVec::new48()
        } else {
            IntVec::new64()
        }
    }

    pub fn new8() -> IntVec {
        IntVec {
            items: Integers::U8(Vec::new()),
        }
    }

    pub fn new16() -> IntVec {
        IntVec {
            items: Integers::U16(Vec::new()),
        }
    }

    pub fn new24() -> IntVec {
        IntVec {
            items: Integers::U24(Vec::new(), Vec::new()),
        }
    }

    pub fn new32() -> IntVec {
        IntVec {
            items: Integers::U32(Vec::new()),
        }
    }

    pub fn new48() -> IntVec {
        IntVec {
            items: Integers::U48(Vec::new(), Vec::new()),
        }
    }

    pub fn new64() -> IntVec {
        IntVec {
            items: Integers::U64(Vec::new()),
        }
    }

    pub fn len(&self) -> usize {
        match &self.items {
            Integers::U8(vec) => {
                vec.len()
            }
            Integers::U16(vec) => {
                vec.len()
            }
            Integers::U24(hi, _low) => {
                hi.len()
            }
            Integers::U32(vec) => {
                vec.len()
            }
            Integers::U48(hi, _low) => {
                hi.len()
            }
            Integers::U64(vec) => {
                vec.len()
            }
        }
    }

    pub fn push(&mut self, x: u64) {
        match &mut self.items {
            Integers::U8(vec) => {
                vec.push(x as u8);
            }
            Integers::U16(vec) => {
                vec.push(x as u16);
            }
            Integers::U24(hi, low) => {
                hi.push((x >> 16) as u8);
                low.push(x as u16);
            }
            Integers::U32(vec) => {
                vec.push(x as u32);
            }
            Integers::U48(hi, low) => {
                hi.push((x >> 32) as u16);
                low.push(x as u32);
            }
            Integers::U64(vec) => {
                vec.push(x);
            }
        }
    }

    pub fn reserve(&mut self, n: usize) {
        match &mut self.items {
            Integers::U8(vec) => {
                vec.reserve(n);
            }
            Integers::U16(vec) => {
                vec.reserve(n);
            }
            Integers::U24(hi, low) => {
                hi.reserve(n);
                low.reserve(n);
            }
            Integers::U32(vec) => {
                vec.reserve(n);
            }
            Integers::U48(hi, low) => {
                hi.reserve(n);
                low.reserve(n);
            }
            Integers::U64(vec) => {
                vec.reserve(n);
            }
        }
    }

    pub fn get(&self, index: usize) -> u64 {
        match &self.items {
            Integers::U8(vec) => vec[index] as u64,
            Integers::U16(vec) => vec[index] as u64,
            Integers::U24(hi, low) => (hi[index] as u64) << 16 | low[index] as u64,
            Integers::U32(vec) => vec[index] as u64,
            Integers::U48(hi, low) => (hi[index] as u64) << 32 | low[index] as u64,
            Integers::U64(vec) => vec[index],
        }
    }

    pub fn set(&mut self, index: usize, x: u64) {
        match &mut self.items {
            Integers::U8(vec) => {
                vec[index] = x as u8;
            }
            Integers::U16(vec) => {
                vec[index] = x as u16;
            }
            Integers::U24(hi, low) => {
                hi[index] = (x >> 16) as u8;
                low[index] = x as u16;
            }
            Integers::U32(vec) => {
                vec[index] = x as u32;
            }
            Integers::U48(hi, low) => {
                hi[index] = (x >> 32) as u16;
                low[index] = x as u32;
            }
            Integers::U64(vec) => {
                vec[index] = x;
            }
        }
    }
}
