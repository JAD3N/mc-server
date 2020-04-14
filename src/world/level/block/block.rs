pub trait Block: mopa::Any + Send + Sync + std::fmt::Debug {}

mopafy!(Block);

#[derive(Debug)]
pub struct BaseBlock {
    pub test: i32,
}

impl Block for BaseBlock {}
