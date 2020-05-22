use super::ProtocolError;
use bytes::{Buf, BufMut, BytesMut};
use std::marker::PhantomData;

pub struct ProtocolData<T> {
    _marker: PhantomData<T>,
}

pub trait ProtocolLength<T> {
    fn len(value: &T) -> usize;
}

pub trait ProtocolRead<T> {
    fn read<U: Buf>(src: &mut U) -> Result<T, ProtocolError> {
        unimplemented!("protocol read not implemented");
    }
}

pub trait ProtocolWrite<T> {
    fn write<U: BufMut>(value: &T, dst: &mut U) -> Result<(), ProtocolError> {
        unimplemented!("protocol write not implemented");
    }
}

#[macro_export]
macro_rules! protocol_data_struct {
    ($name:ident { $($fname:ident: $fty:ty),* $(,)? }) => {
        #[derive(Debug)]
        pub struct $name {
            $(pub $fname: $fty),*
        }

        impl $crate::network::protocol::ProtocolLength<$name> for $crate::network::protocol::ProtocolData<$name> {
            fn len(_value: &$name) -> usize {
                0 $(+ $crate::network::protocol::ProtocolData::<$fty>::len(&_value.$fname))*
            }
        }

        impl $crate::network::protocol::ProtocolRead<$name> for $crate::network::protocol::ProtocolData<$name> {
            fn read<U: bytes::Buf>(_src: &mut U) -> Result<$name, $crate::network::protocol::ProtocolError> {
                Ok($name { $($fname: $crate::network::protocol::ProtocolData::<$fty>::read(_src)?,)* })
            }
        }

        impl $crate::network::protocol::ProtocolWrite<$name> for $crate::network::protocol::ProtocolData<$name> {
            fn write<U: bytes::BufMut>(_value: &$name, _dst: &mut U) -> Result<(), $crate::network::protocol::ProtocolError> {
                $($crate::network::protocol::ProtocolData::<$fty>::write(&_value.$fname, _dst)?;)*
                Ok(())
            }
        }
    };
}

macro_rules! protocol_data_scalar {
    ($t:ty, $len:expr, $w:ident, $r:ident) => {
        impl ProtocolLength<$t> for ProtocolData<$t> {
            fn len(value: &$t) -> usize { $len }
        }

        impl ProtocolRead<$t> for ProtocolData<$t> {
            fn read<U: Buf>(src: &mut U) -> Result<$t, ProtocolError> {
                if src.remaining() < $len {
                    Err(ProtocolError::NotEnoughBytes)
                } else {
                    Ok(src.$r())
                }
            }
        }

        impl ProtocolWrite<$t> for ProtocolData<$t> {
            fn write<U: BufMut>(value: &$t, dst: &mut U) -> Result<(), ProtocolError> {
                if dst.remaining_mut() < $len {
                    Err(ProtocolError::NotEnoughBytes)
                } else {
                    dst.$w(*value);
                    Ok(())
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

impl ProtocolLength<bool>for ProtocolData<bool> {
    fn len(value: &bool) -> usize { 1 }
}

impl ProtocolRead<bool> for ProtocolData<bool> {
    fn read<U: Buf>(src: &mut U) -> Result<bool, ProtocolError> {
        let value = ProtocolData::<u8>::read(src)?;

        if value > 1 {
            Err(ProtocolError::Invalid)
        } else {
            Ok(value == 1)
        }
    }
}

impl ProtocolWrite<bool> for ProtocolData<bool> {
    fn write<U: BufMut>(value: &bool, dst: &mut U) -> Result<(), ProtocolError> {
        ProtocolData::<u8>::write(&(*value as u8), dst)
    }
}

#[derive(Debug)]
pub struct Var<T>(pub T);
pub type VarInt = Var<i32>;
pub type VarLong = Var<i64>;

impl<T: Into<i32>> From<T> for Var<i32> {
    fn from(value: T) -> Self {
        Var(value.into())
    }
}

impl<T: Into<i64>> From<T> for Var<i64> {
    fn from(value: T) -> Self {
        Var(value.into())
    }
}

impl ProtocolLength<Var<i32>> for ProtocolData<Var<i32>> {
    fn len(value: &Var<i32>) -> usize {
        let mut value = value.0;

        for i in 1..5 {
            value >>= 7;

            if value == 0 {
                return i;
            }
        }

        5
    }
}

impl ProtocolRead<Var<i32>> for ProtocolData<Var<i32>> {
    fn read<U: Buf>(src: &mut U) -> Result<Var<i32>, ProtocolError> {
        let mut value = 0i32;
        let mut reads = 0usize;

        loop {
            // do remaining check in u8
            let byte = ProtocolData::<u8>::read(src)? as i32;

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

        Ok(Var(value))
    }
}

impl ProtocolWrite<Var<i32>> for ProtocolData<Var<i32>> {
    fn write<U: BufMut>(value: &Var<i32>, dst: &mut U) -> Result<(), ProtocolError> {
        // do remaining check beforehand
        if dst.remaining_mut() < ProtocolData::<Var<i32>>::len(&value) {
            Err(ProtocolError::NotEnoughBytes)
        } else {
            let mut value = value.0;

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
}

impl ProtocolLength<Var<i64>> for ProtocolData<Var<i64>> {
    fn len(value: &Var<i64>) -> usize {
        let mut value = value.0;

        for i in 1..10 {
            value >>= 7;

            if value == 0 {
                return i;
            }
        }

        5
    }
}

impl ProtocolRead<Var<i64>> for ProtocolData<Var<i64>> {
    fn read<U: Buf>(src: &mut U) -> Result<Var<i64>, ProtocolError> {
        let mut value = 0i64;
        let mut reads = 0usize;

        loop {
            // do remaining check in u8
            let byte = ProtocolData::<u8>::read(src)? as i64;

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

        Ok(Var(value))
    }
}

impl ProtocolWrite<Var<i64>> for ProtocolData<Var<i64>> {
    fn write<U: BufMut>(value: &Var<i64>, dst: &mut U) -> Result<(), ProtocolError> {
        // do remaining check beforehand
        if dst.remaining_mut() < ProtocolData::<Var<i64>>::len(&value) {
            Err(ProtocolError::NotEnoughBytes)
        } else {
            let mut value = value.0;

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
}

impl ProtocolLength<String> for ProtocolData<String> {
    fn len(value: &String) -> usize {
        let len = value.len();
        let len_var: Var<i32> = (len as i32).into();

        ProtocolData::<Var<i32>>::len(&len_var) + len
    }
}

impl ProtocolRead<String> for ProtocolData<String> {
    fn read<U: Buf>(src: &mut U) -> Result<String, ProtocolError> {
        let len_var = ProtocolData::<Var<i32>>::read(src)?;
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

impl ProtocolWrite<String> for ProtocolData<String> {
    fn write<U: BufMut>(value: &String, dst: &mut U) -> Result<(), ProtocolError> {
        let len = value.len();
        let len_var: Var<i32> = (len as i32).into();

        if len > 32767 {
            Err(ProtocolError::TooLarge)
        } else {
            ProtocolData::<Var<i32>>::write(&len_var, dst)?;

            for &byte in value.as_bytes() {
                dst.put_u8(byte);
            }

            Ok(())
        }
    }
}