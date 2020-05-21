use super::ProtocolError;
use bytes::{Buf, BufMut};

pub trait ProtocolData {
    fn len(&self) -> usize;

    fn write<T: BufMut>(&self, dst: &mut T) -> Result<(), ProtocolError> where Self: Sized {
        unimplemented!("protocol write not implemented");
    }

    fn read<T: Buf>(src: &mut T) -> Result<Self, ProtocolError> where Self: Sized {
        unimplemented!("protocol read not implemented");
    }
}

#[macro_export]
macro_rules! protocol_data_struct {
    ($name:ident { $($fname:ident: $fty:ty),* $(,)? }) => {
        #[derive(Debug)]
        pub struct $name {
            $(pub $fname: $fty),*
        }

        impl $crate::network::protocol::ProtocolData for $name {
            fn len(&self) -> usize {
                0 $(+ (self.$fname).len())*
            }

            fn write<T: bytes::BufMut>(&self, _dst: &mut T) -> Result<(), $crate::network::protocol::ProtocolError> {
                $(self.$fname.write(_dst)?;)*
                Ok(())
            }

            fn read<T: bytes::Buf>(_src: &mut T) -> Result<Self, $crate::network::protocol::ProtocolError> {
                Ok($name { $($fname: <$fty>::read(_src)?,)* })
            }
        }
    };
}

macro_rules! protocol_data_scalar {
    ($t:ty, $len:expr, $w:ident, $r:ident) => {
        impl ProtocolData for $t {
            fn len(&self) -> usize { $len }

            fn write<T: BufMut>(&self, dst: &mut T) -> Result<(), ProtocolError> {
                if dst.remaining_mut() < $len {
                    Err(ProtocolError::NotEnoughBytes)
                } else {
                    Ok(dst.$w(*self))
                }
            }

            fn read<T: Buf>(src: &mut T) -> Result<Self, ProtocolError> {
                if src.remaining() < $len {
                    Err(ProtocolError::NotEnoughBytes)
                } else {
                    Ok(src.$r())
                }
            }
        }
    };
}

protocol_data_scalar!(u8, 1, put_u8, get_u8);
protocol_data_scalar!(u16, 2, put_u16, get_u16);
protocol_data_scalar!(u32, 4, put_u32, get_u32);
protocol_data_scalar!(u64, 8, put_u64, get_u64);

protocol_data_scalar!(i8, 1, put_i8, get_i8);
protocol_data_scalar!(i16, 2, put_i16, get_i16);
protocol_data_scalar!(i32, 4, put_i32, get_i32);
protocol_data_scalar!(i64, 8, put_i64, get_i64);

protocol_data_scalar!(f32, 4, put_f32, get_f32);
protocol_data_scalar!(f64, 8, put_f64, get_f64);

impl ProtocolData for bool {
    fn len(&self) -> usize {
        1
    }

    fn write<T: BufMut>(&self, dst: &mut T) -> Result<(), ProtocolError> {
        u8::write(&(*self as u8), dst)
    }

    fn read<T: Buf>(src: &mut T) -> Result<Self, ProtocolError> {
        let value = u8::read(src)?;

        if value > 1 {
            Err(ProtocolError::Invalid)
        } else {
            Ok(value == 1)
        }
    }
}

#[derive(Debug)]
pub struct Var<T>(pub T);
pub type VarInt = Var<i32>;
pub type VarLong = Var<i64>;

impl ProtocolData for Var<i32> {
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

    fn write<T: BufMut>(&self, dst: &mut T) -> Result<(), ProtocolError> {
        // do remaining check beforehand
        if dst.remaining_mut() < self.len() {
            Err(ProtocolError::NotEnoughBytes)
        } else {
            let mut value = self.0;

            loop {
                let mut byte = (value & 0b01111111) as u8;

                value >>= 7;

                if value != 0 {
                    byte |= 0b10000000;
                }

                dst.put_u8(byte);

                if value == 0 {
                    break;
                }
            }

            Ok(())
        }
    }

    fn read<T: Buf>(src: &mut T) -> Result<Self, ProtocolError> {
        let mut value = 0i32;
        let mut reads = 0usize;

        loop {
            // do remaining check in u8
            let byte = u8::read(src)? as i32;

            value |= (byte & 0b01111111) << (7 * reads);
            reads += 1;

            if reads > 5 {
                // VarInt is bigger than 5 bytes
                return Err(ProtocolError::TooLarge);
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

impl ProtocolData for Var<i64> {
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

    fn write<T: BufMut>(&self, dst: &mut T) -> Result<(), ProtocolError> {
        // do remaining check beforehand
        if dst.remaining_mut() < self.len() {
            Err(ProtocolError::NotEnoughBytes)
        } else {
            let mut value = self.0;

            loop {
                let mut byte = (value & 0b01111111) as u8;

                value >>= 7;

                if value != 0 {
                    byte |= 0b10000000;
                }

                dst.put_u8(byte);

                if value == 0 {
                    break;
                }
            }

            Ok(())
        }
    }

    fn read<T: Buf>(src: &mut T) -> Result<Self, ProtocolError> {
        let mut value = 0i64;
        let mut reads = 0usize;

        loop {
            // do remaining check in u8
            let byte = u8::read(src)? as i64;

            value |= (byte & 0b01111111) << (7 * reads);
            reads += 1;

            if reads > 10 {
                // VarInt is bigger than 10 bytes
                return Err(ProtocolError::TooLarge);
            }

            if byte & 0b10000000 == 0 {
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


impl ProtocolData for String {
    fn len(&self) -> usize {
        let len = self.len();
        let len_var: Var<i32> = (len as i32).into();

        len_var.len() + len
    }

    fn write<T: BufMut>(&self, dst: &mut T) -> Result<(), ProtocolError> {
        let len = self.len();
        let len_var: Var<i32> = (len as i32).into();

        if len > 32767 {
            Err(ProtocolError::TooLarge)
        } else {
            len_var.write(dst)?;

            for &byte in self.as_bytes() {
                dst.put_u8(byte);
            }

            Ok(())
        }
    }

    fn read<T: Buf>(src: &mut T) -> Result<Self, ProtocolError> {
        let len_var = Var::<i32>::read(src)?;
        let len = len_var.0 as usize;

        if len > 32767 {
            Err(ProtocolError::TooLarge)
        } else {
            let mut bytes = vec![0u8; len];

            // copy bytes to vec
            src.copy_to_slice(&mut bytes);

            // convert bytes to string
            String::from_utf8(bytes)
                .map_err(|_| ProtocolError::Invalid)
        }
    }
}