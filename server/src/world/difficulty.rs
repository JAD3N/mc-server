use std::slice::Iter;

const PEACEFUL: (i32, &str) = (0, "peaceful");
const EASY: (i32, &str) = (1, "easy");
const NORMAL: (i32, &str) = (2, "normal");
const HARD: (i32, &str) = (3, "hard");

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}

const ALL: [Difficulty; 4] = [
    Difficulty::Peaceful,
    Difficulty::Easy,
    Difficulty::Normal,
    Difficulty::Hard,
];

impl Difficulty {
    pub fn id(&self) -> i32 {
        match self {
            Difficulty::Peaceful => PEACEFUL.0,
            Difficulty::Easy => EASY.0,
            Difficulty::Normal => NORMAL.0,
            Difficulty::Hard => HARD.0,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Difficulty::Peaceful => PEACEFUL.1,
            Difficulty::Easy => EASY.1,
            Difficulty::Normal => NORMAL.1,
            Difficulty::Hard => HARD.1,
        }
    }

    pub fn iter() -> Iter<'static, Difficulty> {
        ALL.iter()
    }

    pub fn from_id(id: i32) -> Option<Difficulty> {
        for difficulty in Self::iter() {
            if difficulty.id() == id {
                return Some(*difficulty);
            }
        }

        None
    }

    pub fn from_name(name: &str) -> Option<Difficulty> {
        for difficulty in Self::iter() {
            if difficulty.name() == name {
                return Some(*difficulty);
            }
        }

        None
    }
}