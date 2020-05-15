/*
public static final LevelType[] LEVEL_TYPES = new LevelType[16];
   public static final LevelType NORMAL = (new LevelType(0, "default", 1)).setHasReplacement();
   public static final LevelType FLAT = (new LevelType(1, "flat")).setCustomOptions(true);
   public static final LevelType LARGE_BIOMES = new LevelType(2, "largeBiomes");
   public static final LevelType AMPLIFIED = (new LevelType(3, "amplified")).setHasHelpText();
   public static final LevelType CUSTOMIZED = (new LevelType(4, "customized", "normal", 0)).setCustomOptions(true).setSelectableByUser(false);
   public static final LevelType BUFFET = (new LevelType(5, "buffet")).setCustomOptions(true);
   public static final LevelType DEBUG_ALL_BLOCK_STATES = new LevelType(6, "debug_all_block_states");
   public static final LevelType NORMAL_1_1 = (new LevelType(8, "default_1_1", 0)).setSelectableByUser(false);
*/
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
pub enum LevelType {
    Default,
    Flat,
    LargeBiomes,
    Amplified,
    Customized,
    Buffet,
    Debug,
    // Normal11,
}

const ALL: [LevelType; 7] = [
    LevelType::Default,
    LevelType::Flat,
    LevelType::LargeBiomes,
    LevelType::Amplified,
    LevelType::Customized,
    LevelType::Buffet,
    LevelType::Debug,
    // LevelType::Normal11,
];

impl LevelType {
    pub fn id(&self) -> i32 {
        match self {
            LevelType::Default => DEFAULT.0,
            LevelType::Flat => FLAT.0,
            LevelType::LargeBiomes => LARGE_BIOMES.0,
            LevelType::Amplified => AMPLIFIED.0,
            LevelType::Customized => CUSTOMIZED.0,
            LevelType::Buffet => BUFFET.0,
            LevelType::Debug => DEBUG.0,
            // LevelType::Normal11 => NORMAL_1_1.0,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            LevelType::Default => DEFAULT.1,
            LevelType::Flat => FLAT.1,
            LevelType::LargeBiomes => LARGE_BIOMES.1,
            LevelType::Amplified => AMPLIFIED.1,
            LevelType::Customized => CUSTOMIZED.1,
            LevelType::Buffet => BUFFET.1,
            LevelType::Debug => DEBUG.1,
            // LevelType::Normal11 => NORMAL_1_1.1,
        }
    }

    pub fn serialization(&self) -> &str {
        match self {
            LevelType::Default => DEFAULT.2,
            LevelType::Flat => FLAT.2,
            LevelType::LargeBiomes => LARGE_BIOMES.2,
            LevelType::Amplified => AMPLIFIED.2,
            LevelType::Customized => CUSTOMIZED.2,
            LevelType::Buffet => BUFFET.2,
            LevelType::Debug => DEBUG.2,
            // LevelType::Normal11 => NORMAL_1_1.2,
        }
    }

    pub fn version(&self) -> u8 {
        match self {
            LevelType::Default => DEFAULT.3,
            LevelType::Flat => FLAT.3,
            LevelType::LargeBiomes => LARGE_BIOMES.3,
            LevelType::Amplified => AMPLIFIED.3,
            LevelType::Customized => CUSTOMIZED.3,
            LevelType::Buffet => BUFFET.3,
            LevelType::Debug => DEBUG.3,
            // LevelType::Normal11 => NORMAL_1_1.3,
        }
    }

    pub fn iter() -> Iter<'static, LevelType> {
        ALL.iter()
    }

    pub fn from_id(id: i32) -> Option<LevelType> {
        for type_ in Self::iter() {
            if type_.id() == id {
                return Some(*type_);
            }
        }

        None
    }

    pub fn from_name(name: &str) -> Option<LevelType> {
        for type_ in Self::iter() {
            if type_.name() == name {
                return Some(*type_);
            }
        }

        None
    }
}