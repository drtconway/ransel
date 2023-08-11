use std::io::{Read, Write};

use num_traits::{FromBytes, ToBytes};

pub trait Persistent {
    fn save<Sink>(&self, sink: &mut Sink) -> std::io::Result<()>
    where
        Sink: Write;

    fn load<Source>(source: &mut Source) -> std::io::Result<Box<Self>>
    where
        Source: Read;
}

pub fn save_vec<Sink, T>(sink: &mut Sink, xs: &[T]) -> std::io::Result<()>
where
    Sink: Write,
    T: ToBytes,
{
    sink.write_all(&xs.len().to_ne_bytes())?;
    for i in 0..xs.len() {
        let bytes = ToBytes::to_ne_bytes(&xs[i]);
        sink.write_all(bytes.as_ref())?;
    }
    Ok(())
}

pub fn load_u64<Source>(source: &mut Source) -> std::io::Result<u64> where Source: Read {
    const X_SIZE: usize = std::mem::size_of::<u64>();
    let mut x_buf: [u8; X_SIZE] = [0; X_SIZE];
    source.read_exact(&mut x_buf)?;
    let x: u64 = FromBytes::from_ne_bytes(&x_buf);
    Ok(x)
}

pub fn load_vec_u8<Source>(source: &mut Source) -> std::io::Result<Vec<u8>>
where
    Source: Read,
{
    const N_SIZE: usize = std::mem::size_of::<usize>();
    let mut n_buf: [u8; N_SIZE] = [0; N_SIZE];
    source.read_exact(&mut n_buf)?;
    let n: usize = FromBytes::from_ne_bytes(&n_buf);

    let mut res: Vec<u8> = Vec::new();
    res.reserve(n);

    const X_SIZE: usize = std::mem::size_of::<u8>();
    let mut x_buf: [u8; X_SIZE] = [0; X_SIZE];

    for _i in 0..n {
        source.read_exact(&mut x_buf)?;
        let x: u8 = FromBytes::from_ne_bytes(&x_buf);
        res.push(x);
    }
    Ok(res)
}

pub fn load_vec_u16<Source>(source: &mut Source) -> std::io::Result<Vec<u16>>
where
    Source: Read,
{
    const N_SIZE: usize = std::mem::size_of::<usize>();
    let mut n_buf: [u8; N_SIZE] = [0; N_SIZE];
    source.read_exact(&mut n_buf)?;
    let n: usize = FromBytes::from_ne_bytes(&n_buf);

    let mut res: Vec<u16> = Vec::new();
    res.reserve(n);

    const X_SIZE: usize = std::mem::size_of::<u16>();
    let mut x_buf: [u8; X_SIZE] = [0; X_SIZE];

    for _i in 0..n {
        source.read_exact(&mut x_buf)?;
        let x: u16 = FromBytes::from_ne_bytes(&x_buf);
        res.push(x);
    }
    Ok(res)
}

pub fn load_vec_u32<Source>(source: &mut Source) -> std::io::Result<Vec<u32>>
where
    Source: Read,
{
    const N_SIZE: usize = std::mem::size_of::<usize>();
    let mut n_buf: [u8; N_SIZE] = [0; N_SIZE];
    source.read_exact(&mut n_buf)?;
    let n: usize = FromBytes::from_ne_bytes(&n_buf);

    let mut res: Vec<u32> = Vec::new();
    res.reserve(n);

    const X_SIZE: usize = std::mem::size_of::<u32>();
    let mut x_buf: [u8; X_SIZE] = [0; X_SIZE];

    for _i in 0..n {
        source.read_exact(&mut x_buf)?;
        let x: u32 = FromBytes::from_ne_bytes(&x_buf);
        res.push(x);
    }
    Ok(res)
}

pub fn load_vec_u64<Source>(source: &mut Source) -> std::io::Result<Vec<u64>>
where
    Source: Read,
{
    const N_SIZE: usize = std::mem::size_of::<usize>();
    let mut n_buf: [u8; N_SIZE] = [0; N_SIZE];
    source.read_exact(&mut n_buf)?;
    let n: usize = FromBytes::from_ne_bytes(&n_buf);

    let mut res: Vec<u64> = Vec::new();
    res.reserve(n);

    const X_SIZE: usize = std::mem::size_of::<u64>();
    let mut x_buf: [u8; X_SIZE] = [0; X_SIZE];

    for _i in 0..n {
        source.read_exact(&mut x_buf)?;
        let x: u64 = FromBytes::from_ne_bytes(&x_buf);
        res.push(x);
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use std::io::{BufWriter, Cursor};

    use super::*;

    #[test]
    fn test_vec_u8() {
        let xs: Vec<u8> = vec![23, 56, 129, 230, 255];

        let mut buf = BufWriter::new(Vec::new());

        save_vec(&mut buf, &xs).expect("save_vec failed");

        let bytes = buf.into_inner().expect("failed to get bytes");

        let mut cursor = Cursor::new(bytes);

        let ys: Vec<u8> = load_vec_u8(&mut cursor).expect("load_vec failed");

        assert_eq!(xs, ys);
    }

    #[test]
    fn test_vec_u16() {
        let xs: Vec<u16> = vec![
            0xb3c4, 0x008a, 0x9b9f, 0x0a73, 0x26e1, 0x1b9c, 0x0c88, 0x8871, 0xdb2d, 0x8f4b,
        ];

        let mut buf = BufWriter::new(Vec::new());

        save_vec(&mut buf, &xs).expect("save_vec failed");

        let bytes = buf.into_inner().expect("failed to get bytes");

        let mut cursor = Cursor::new(bytes);

        let ys: Vec<u16> = load_vec_u16(&mut cursor).expect("load_vec failed");

        assert_eq!(xs, ys);
    }
}
