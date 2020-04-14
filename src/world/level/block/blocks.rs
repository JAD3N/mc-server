// TODO: This will have lazy static references of all known blocks.

use crate::core::registry;
use super::{Block, BaseBlock};

pub struct Blocks;

impl registry::Registerable for Blocks {
    fn register() {
        let block = Box::new(BaseBlock { test: 123 });
        register!(BLOCKS, "block", block);

        let reg_ref = registry!(BLOCKS).read().unwrap();
        let block_ref = reg_ref.get_value("block").unwrap();

        info!("{:?}", block_ref.downcast_ref::<BaseBlock>());
    }
}