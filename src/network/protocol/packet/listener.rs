use std::sync::{Arc, Mutex};
use crate::network::Connection;
use std::any::Any;

pub trait PacketListener {
    //fn connection(&self) -> Arc<Mutex<Connection>>;
    fn as_any(self) -> Box<dyn Any> where Self: Sized + 'static {
        Box::new(self)
    }
}