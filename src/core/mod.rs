#[macro_use]
pub mod registry;

#[macro_use]
pub mod sound;
pub use sound::Sound;

#[macro_use]
pub mod events;

mod resource_location;
pub use resource_location::*;