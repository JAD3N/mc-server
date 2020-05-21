#[macro_use]
mod packet;

pub use packet::*;

use bytes::{Buf, BufMut, BytesMut};
use std::io;
use crate::server;

#[async_trait]
pub trait Protocol {
    fn len(&self) -> usize;

    async fn write(&self, dst: &mut BytesMut) -> io::Result<()>  where Self: Sized;
    async fn read(src: &mut BytesMut) -> io::Result<Self> where Self: Sized;
}

#[async_trait]
impl Protocol for bool {
    fn len(&self) -> usize {
        1
    }

    async fn write(&self, dst: &mut BytesMut) -> io::Result<()> {
        dst.put_u8(if *self { 1 } else { 0 });
        Ok(())
    }


    async fn read(src: &mut BytesMut) -> io::Result<Self> {
        let value = src.get_i8();

        if value > 1 {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                &(format!("Invalid bool value, expected 0 or 1, got {}", value)[..])
            ))
        } else {
            Ok(value == 1)
        }
    }
}
/*
pub struct Var<T>(T);

impl Protocol for Var<i32> {
    fn len(&self) -> usize {
        let mut value = self.0;

        for i in 1..5 {
            value >>= 7;

            if value == 0 {
                return i;
            }
        }

        5
    }

    fn write<T: Write>(&self, dst: &mut T) -> io::Result<()> {
        let mut value = self.0;
        let mut tmp: u8;

        loop {
            tmp = (value & 0b01111111) as u8;
            value >>= 7;

            if value != 0 {
                tmp |= 0b10000000;
            }

            dst.write_u8(tmp)?;

            if value == 0 {
                break;
            }
        }

        Ok(())
    }

    fn read<T: Read>(src: &mut T) -> io::Result<Self> {
        let mut value = 0i32;
        let mut reads = 0usize;

        loop {
            let byte = src.read_u8()? as i32;

            value |= (byte & 0b01111111) << (7 * reads);
            reads += 1;

            if reads > 5 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "VarInt is bigger than 5 bytes",
                ));
            }

            if byte & 0b10000000 == 0 {
                break;
            }
        }

        Ok(Self(value))
    }
}

impl<T: Into<i32>> From<T> for Var<i32> {
    fn from(value: T) -> Self {
        Var(value.into())
    }
}

impl Protocol for Var<i64> {
    fn len(&self) -> usize {
        let mut value = self.0;

        for i in 1..10 {
            value >>= 7;

            if value == 0 {
                return i;
            }
        }

        10
    }

    fn write<T: Write>(&self, dst: &mut T) -> io::Result<()> {
        let mut value = self.0;
        let mut tmp: u8;

        loop {
            tmp = (value & 0b01111111) as u8;
            value >>= 7;

            if value != 0 {
                tmp |= 0b10000000;
            }

            dst.write_u8(tmp)?;

            if value == 0 {
                break;
            }
        }

        Ok(())
    }

    fn read<T: Read>(src: &mut T) -> io::Result<Self> {
        let mut value = 0i64;
        let mut reads = 0usize;

        loop {
            let byte = src.read_u8()? as i64;

            value |= (byte & 0b0111_1111) << (7 * reads);
            reads += 1;

            if reads > 10 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "VarLong is bigger than 10 bytes",
                ));
            }

            if byte & 0b1000_0000 == 0 {
                break;
            }
        }

        Ok(Self(value))
    }
}

impl<T: Into<i64>> From<T> for Var<i64> {
    fn from(value: T) -> Self {
        Var(value.into())
    }
}

impl Protocol for String {
    fn len(&self) -> usize {
        let len = self.len();
        let len_var: Var<i32> = (len as i32).into();

        len_var.len() + len
    }

    fn write<T: Write>(&self, dst: &mut T) -> io::Result<()> {
        let len_var: Var<i32> = (self.len() as i32).into();

        len_var.write(dst)?;
        dst.write_all(self.as_bytes())
    }

    fn read<T: Read>(src: &mut T) -> io::Result<Self> {
        let len_var = Var::<i32>::read(src)?;
        let mut bytes = vec![0u8; len_var.0 as usize];

        src.read_exact(&mut bytes)?;

        String::from_utf8(bytes).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Error reading string!",
            )
        })
    }
}

macro_rules! scalar_data_type {
    ($t:ty, 1, $w:ident, $r:ident) => {
        impl Protocol for $t {
            fn len(&self) -> usize { 1 }

            fn write<T: Write>(&self, dst: &mut T) -> io::Result<()> {
                dst.$w(*self)?;
                Ok(())
            }

            fn read<T: Read>(src: &mut T) -> io::Result<Self> {
                src.$r().map_err(|err| io::Error::from(err))
            }
        }
    };
    ($t:ty, $len:expr, $w:ident, $r:ident) => {
        impl Protocol for $t {
            fn len(&self) -> usize { $len }

            fn write<T: Write>(&self, dst: &mut T) -> io::Result<()> {
                dst.$w::<BigEndian>(*self)?;
                Ok(())
            }

            fn read<T: Read>(src: &mut T) -> io::Result<Self> {
                src.$r::<BigEndian>().map_err(|err| io::Error::from(err))
            }
        }
    };
}

scalar_data_type!(u8, 1, write_u8, read_u8);
scalar_data_type!(u16, 2, write_u16, read_u16);
scalar_data_type!(u32, 4, write_u32, read_u32);
scalar_data_type!(u64, 8, write_u64, read_u64);

scalar_data_type!(i8, 1, write_i8, read_i8);
scalar_data_type!(i16, 2, write_i16, read_i16);
scalar_data_type!(i32, 4, write_i32, read_i32);
scalar_data_type!(i64, 8, write_i64, read_i64);

scalar_data_type!(f32, 4, write_f32, read_f32);
scalar_data_type!(f64, 8, write_f64, read_f64);*/