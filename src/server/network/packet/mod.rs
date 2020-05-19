mod data_type;

pub use data_type::*;

use std::io::{self, Read, Write};

trait Packet: DataType {}

#[macro_export]
macro_rules! packet {
    ($name:ident { $($fname:ident: $fty:ty),* $(,)? }) => {
        pub struct $name {
            $(pub $fname: $fty),*
        }

        impl DataType for $name {
            fn len(&self) -> usize {
                0 $(+ (self.$fname).len())*
            }

            fn write<T: Write>(&self, dst: &mut T) -> io::Result<()> {
                $(self.$fname.write(dst)?;)*
                Ok(())
            }

            fn read<T: Read>(src: &mut T) -> io::Result<$name> {
                Ok($name { $($fname: <$fty>::read(src)?,)* })
            }
        }

        impl Packet for $name {}
    };
}

packet!(PlayerMovePacket {
    id: Var<i32>,
    id2: Var<i32>,
});