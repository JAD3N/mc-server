// use crate::register;

#[derive(Debug, PartialEq)]
pub struct DimensionType {
    id: i32,
    name: &'static str,
    suffix: &'static str,
    folder: &'static str,
    has_sky_light: bool,
}

impl DimensionType {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn suffix(&self) -> &str {
        self.suffix
    }

    pub fn folder(&self) -> &str {
        self.folder
    }

    pub fn has_sky_light(&self) -> bool {
        self.has_sky_light
    }
}

macro_rules! add_dimension_type {
    ($key:ident, $id:expr, $name:expr, $suffix:expr, $folder:expr) => {
        add_dimension_type!($key, $id, $name, $suffix, $folder, false);
    };
    ($key:ident, $id:expr, $name:expr, $suffix:expr, $folder:expr, $has_sky_light:expr) => {
        impl DimensionType {
            pub const $key: DimensionType = DimensionType {
                id: $id,
                name: $name,
                suffix: $suffix,
                folder: $folder,
                has_sky_light: $has_sky_light,
            };
        }

        // register!($name, DimensionType::<$key>);
    };
}

add_dimension_type!(OVERWORLD, 1, "overworld", "", "", true);
add_dimension_type!(NETHER, 0, "the_nether", "_nether", "DIM-1");
add_dimension_type!(END, 2, "the_end", "_end", "DIM1");

pub struct Dimension {
    type_: DimensionType,
}