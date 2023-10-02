//! A module for storing unsigned integers of different widths.

use std::io::{Error, ErrorKind};

use crate::persist::{
    load_usize, load_vec_u16, load_vec_u32, load_vec_u64, load_vec_u8, save_vec, Persistent,
};

enum Integers {
    U8(Vec<u8>),
    U16(Vec<u16>),
    U24(Vec<u8>, Vec<u16>),
    U32(Vec<u32>),
    U48(Vec<u16>, Vec<u32>),
    U64(Vec<u64>),
}

/// A vector of unsigned integers.
/// 
/// Values are stored in a vector of an appropriate width, or
/// in some cases 2 vectors, if required.
/// 
pub struct IntVec {
    items: Integers,
}

impl IntVec {
    /// Create an empty vector for integers of the requested width.
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

    fn new8() -> IntVec {
        IntVec {
            items: Integers::U8(Vec::new()),
        }
    }

    fn new16() -> IntVec {
        IntVec {
            items: Integers::U16(Vec::new()),
        }
    }

    fn new24() -> IntVec {
        IntVec {
            items: Integers::U24(Vec::new(), Vec::new()),
        }
    }

    fn new32() -> IntVec {
        IntVec {
            items: Integers::U32(Vec::new()),
        }
    }

    fn new48() -> IntVec {
        IntVec {
            items: Integers::U48(Vec::new(), Vec::new()),
        }
    }

    fn new64() -> IntVec {
        IntVec {
            items: Integers::U64(Vec::new()),
        }
    }

    /// Return the length of the vector.
    pub fn len(&self) -> usize {
        match &self.items {
            Integers::U8(vec) => vec.len(),
            Integers::U16(vec) => vec.len(),
            Integers::U24(hi, _low) => hi.len(),
            Integers::U32(vec) => vec.len(),
            Integers::U48(hi, _low) => hi.len(),
            Integers::U64(vec) => vec.len(),
        }
    }

    /// Push a new value on to the end of the vector.
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

    /// Reserve enough space for the given number of elements.
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

    /// Get an element from the vector.
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

    /// Update an element of the vector.
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

impl Persistent for IntVec {
    fn save<Sink>(&self, sink: &mut Sink) -> std::io::Result<()>
    where
        Sink: std::io::Write,
    {
        match &self.items {
            Integers::U8(vec) => {
                let fmt: usize = 8;
                sink.write_all(&fmt.to_ne_bytes())?;
                save_vec(sink, vec)?;
            }
            Integers::U16(vec) => {
                let fmt: usize = 16;
                sink.write_all(&fmt.to_ne_bytes())?;
                save_vec(sink, vec)?;
            }
            Integers::U24(hi, low) => {
                let fmt: usize = 24;
                sink.write_all(&fmt.to_ne_bytes())?;
                save_vec(sink, hi)?;
                save_vec(sink, low)?;
            }
            Integers::U32(vec) => {
                let fmt: usize = 32;
                sink.write_all(&fmt.to_ne_bytes())?;
                save_vec(sink, vec)?;
            }
            Integers::U48(hi, low) => {
                let fmt: usize = 48;
                sink.write_all(&fmt.to_ne_bytes())?;
                save_vec(sink, hi)?;
                save_vec(sink, low)?;
            }
            Integers::U64(vec) => {
                let fmt: usize = 64;
                sink.write_all(&fmt.to_ne_bytes())?;
                save_vec(sink, vec)?;
            }
        }
        Ok(())
    }

    fn load<Source>(source: &mut Source) -> std::io::Result<Box<Self>>
    where
        Source: std::io::Read,
    {
        let z = load_usize(source)?;
        match z {
            8 => {
                let vec: Vec<u8> = load_vec_u8(source)?;
                Ok(Box::new(IntVec {
                    items: Integers::U8(vec),
                }))
            }
            16 => {
                let vec: Vec<u16> = load_vec_u16(source)?;
                Ok(Box::new(IntVec {
                    items: Integers::U16(vec),
                }))
            }
            24 => {
                let hi: Vec<u8> = load_vec_u8(source)?;
                let low: Vec<u16> = load_vec_u16(source)?;
                Ok(Box::new(IntVec {
                    items: Integers::U24(hi, low),
                }))
            }
            32 => {
                let vec: Vec<u32> = load_vec_u32(source)?;
                Ok(Box::new(IntVec {
                    items: Integers::U32(vec),
                }))
            }
            48 => {
                let hi: Vec<u16> = load_vec_u16(source)?;
                let low: Vec<u32> = load_vec_u32(source)?;
                Ok(Box::new(IntVec {
                    items: Integers::U48(hi, low),
                }))
            }
            64 => {
                let vec: Vec<u64> = load_vec_u64(source)?;
                Ok(Box::new(IntVec {
                    items: Integers::U64(vec),
                }))
            }
            _ => Err(Error::new(
                ErrorKind::Other,
                format!("unknown IntVec size: {}", z),
            )),
        }
    }
}
