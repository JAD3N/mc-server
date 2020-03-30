use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
enum DimensionTrait {
    HasSkyLight,
}

traitable!(DimensionTrait, Dimension {
    id: i32,
    name: &'static str,
    suffix: &'static str,
    folder: &'static str,
});

macro_rules! dimension_type {
    ($id:expr, $name:expr, $suffix:expr, $folder:expr) => {
        dimension_type!($id, $name, $suffix, $folder, []);
    };
    ($id:expr, $name:expr, $suffix:expr, $folder:expr, [ $( $trait:ident ),* ]) => {
        Dimension {
            id: $id,
            name: $name,
            suffix: $suffix,
            folder: $folder,
            traits: vec![
                $({ DimensionTrait::$trait }),*
            ],
        }
    };
}

lazy_static! {
    static ref DIMENSIONS: HashMap<&'static str, Dimension> = {
        let mut m = HashMap::new();

        m.insert("overworld", dimension_type!(1, "overworld", "", "", [HasSkyLight]));
        m.insert("the_nether", dimension_type!(0, "the_nether", "_nether", "DIM-1"));
        m.insert("the_end", dimension_type!(2, "the_nether", "_end", "DIM1"));

        m
    };
}

impl Dimension {
    pub fn from_name(name: &str) -> Option<&Dimension> {
        DIMENSIONS.get(name)
    }
}