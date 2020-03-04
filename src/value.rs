use std::{collections::HashSet, fmt};

use crossterm::style::Color;
use lazy_static::lazy_static;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

lazy_static! {
    pub static ref ALL_VALUES: HashSet<Value> = Value::iter().collect();
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, EnumIter)]
pub enum Value {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Value {
    pub fn color(self) -> Color {
        match self {
            Self::One => Color::Rgb { r: 255, g: 0, b: 0 },
            Self::Two => Color::Rgb {
                r: 255,
                g: 120,
                b: 0,
            },
            Self::Three => Color::Rgb {
                r: 255,
                g: 255,
                b: 0,
            },
            Self::Four => Color::Rgb {
                r: 120,
                g: 255,
                b: 0,
            },
            Self::Five => Color::Rgb {
                r: 0,
                g: 180,
                b: 80,
            },
            Self::Six => Color::Rgb {
                r: 0,
                g: 150,
                b: 255,
            },
            Self::Seven => Color::Rgb {
                r: 60,
                g: 80,
                b: 220,
            },
            Self::Eight => Color::Rgb {
                r: 140,
                g: 40,
                b: 255,
            },
            Self::Nine => Color::Rgb {
                r: 240,
                g: 20,
                b: 255,
            },
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            Self::One => '1',
            Self::Two => '2',
            Self::Three => '3',
            Self::Four => '4',
            Self::Five => '5',
            Self::Six => '6',
            Self::Seven => '7',
            Self::Eight => '8',
            Self::Nine => '9',
        };
        write!(f, "{}", val)
    }
}

impl From<u8> for Value {
    fn from(number: u8) -> Self {
        match number {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            _ => panic!("value cannot be less than 1, or greater than 9"),
        }
    }
}
