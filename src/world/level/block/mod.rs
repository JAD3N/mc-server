mod air;

pub use air::*;

pub trait Block: mopa::Any + Send + Sync {}

mopafy!(Block);