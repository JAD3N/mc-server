use super::{ResourceLocation, ResourceLocatable};

#[derive(Debug)]
pub struct Sound {
    resource_location: ResourceLocation,
}

impl Sound {
    pub fn new<T: Into<ResourceLocation>>(location: T) -> Self {
        Self { resource_location: location.into() }
    }
}

impl ResourceLocatable for Sound {
    fn resource_location(&self) -> &ResourceLocation {
        &self.resource_location
    }
}