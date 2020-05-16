use std::slice::Iter;

// id, name, serialization, version
const DEFAULT: (i32, &str, &str, u8) = (0, "default", "default", 1);
const FLAT: (i32, &str, &str, u8) = (1, "flat", "flat", 0);
const LARGE_BIOMES: (i32, &str, &str, u8) = (2, "largeBiomes", "largeBiomes", 0);
const AMPLIFIED: (i32, &str, &str, u8) = (3, "amplified", "amplified", 0);
const CUSTOMIZED: (i32, &str, &str, u8) = (4, "customized", "normal", 0);
const BUFFET: (i32, &str, &str, u8) = (5, "buffet", "buffet", 0);
const DEBUG: (i32, &str, &str, u8) = (6, "debug_all_block_states", "debug_all_block_states", 0);
// const NORMAL_1_1: (i32, &str, &str, u8) = (8, "default_1_1", "default_1_1", 0);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LevelKind {
    Default,
    Flat,
    LargeBiomes,
    Amplified,
    Customized,
    Buffet,
    Debug,
    // Normal11,
}

const ALL: [LevelKind; 7] = [
    LevelKind::Default,
    LevelKind::Flat,
    LevelKind::LargeBiomes,
    LevelKind::Amplified,
    LevelKind::Customized,
    LevelKind::Buffet,
    LevelKind::Debug,
    // LevelKind::Normal11,
];

impl LevelKind {
    pub fn id(&self) -> i32 {
        match self {
            LevelKind::Default => DEFAULT.0,
            LevelKind::Flat => FLAT.0,
            LevelKind::LargeBiomes => LARGE_BIOMES.0,
            LevelKind::Amplified => AMPLIFIED.0,
            LevelKind::Customized => CUSTOMIZED.0,
            LevelKind::Buffet => BUFFET.0,
            LevelKind::Debug => DEBUG.0,
            // LevelKind::Normal11 => NORMAL_1_1.0,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            LevelKind::Default => DEFAULT.1,
            LevelKind::Flat => FLAT.1,
            LevelKind::LargeBiomes => LARGE_BIOMES.1,
            LevelKind::Amplified => AMPLIFIED.1,
            LevelKind::Customized => CUSTOMIZED.1,
            LevelKind::Buffet => BUFFET.1,
            LevelKind::Debug => DEBUG.1,
            // LevelKind::Normal11 => NORMAL_1_1.1,
        }
    }

    pub fn serialization(&self) -> &str {
        match self {
            LevelKind::Default => DEFAULT.2,
            LevelKind::Flat => FLAT.2,
            LevelKind::LargeBiomes => LARGE_BIOMES.2,
            LevelKind::Amplified => AMPLIFIED.2,
            LevelKind::Customized => CUSTOMIZED.2,
            LevelKind::Buffet => BUFFET.2,
            LevelKind::Debug => DEBUG.2,
            // LevelKind::Normal11 => NORMAL_1_1.2,
        }
    }

    pub fn version(&self) -> u8 {
        match self {
            LevelKind::Default => DEFAULT.3,
            LevelKind::Flat => FLAT.3,
            LevelKind::LargeBiomes => LARGE_BIOMES.3,
            LevelKind::Amplified => AMPLIFIED.3,
            LevelKind::Customized => CUSTOMIZED.3,
            LevelKind::Buffet => BUFFET.3,
            LevelKind::Debug => DEBUG.3,
            // LevelKind::Normal11 => NORMAL_1_1.3,
        }
    }

    pub fn iter() -> Iter<'static, LevelKind> {
        ALL.iter()
    }

    pub fn from_id(id: i32) -> Option<LevelKind> {
        for type_ in Self::iter() {
            if type_.id() == id {
                return Some(*type_);
            }
        }

        None
    }

    pub fn from_name(name: &str) -> Option<LevelKind> {
        for type_ in Self::iter() {
            if type_.name() == name {
                return Some(*type_);
            }
        }

        None
    }
}