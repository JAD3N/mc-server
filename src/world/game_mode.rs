use std::slice::Iter;

const UNSET: (i32, &str) = (-1, "");
const SURVIVAL: (i32, &str) = (0, "survival");
const CREATIVE: (i32, &str) = (1, "creative");
const ADVENTURE: (i32, &str) = (2, "adventure");
const SPECTATOR: (i32, &str) = (3, "spectator");

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameMode {
    Unset,
    Survival,
    Creative,
    Adventure,
    Spectator,
}

const ALL: [GameMode; 5] = [
    GameMode::Unset,
    GameMode::Survival,
    GameMode::Creative,
    GameMode::Adventure,
    GameMode::Spectator,
];

impl GameMode {
    pub fn id(&self) -> i32 {
        match self {
            GameMode::Unset => UNSET.0,
            GameMode::Survival => SURVIVAL.0,
            GameMode::Creative => CREATIVE.0,
            GameMode::Adventure => ADVENTURE.0,
            GameMode::Spectator => SPECTATOR.0,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            GameMode::Unset => UNSET.1,
            GameMode::Survival => SURVIVAL.1,
            GameMode::Creative => CREATIVE.1,
            GameMode::Adventure => ADVENTURE.1,
            GameMode::Spectator => SPECTATOR.1,
        }
    }

    pub fn iter() -> Iter<'static, GameMode> {
        ALL.iter()
    }

    pub fn from_id(id: i32) -> Option<GameMode> {
        for mode in Self::iter() {
            if mode.id() == id {
                return Some(*mode);
            }
        }

        None
    }

    pub fn from_name(name: &str) -> Option<GameMode> {
        for mode in Self::iter() {
            if mode.name() == name {
                return Some(*mode);
            }
        }

        None
    }
}