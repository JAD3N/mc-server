use super::Dimension;

struct Level {
    dimension: Dimension,
}

impl Level {
    pub fn new(dimension_name: &str) -> Level {
        // TODO: Create dimension from dimension type
        let dimension = Dimension::from_name(dimension_name)
            .expect(&format!("Invalid dimension: {}", dimension_name))
            .clone();

        Level { dimension }
    }
}