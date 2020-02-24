use std::fmt;

use crate::value::Value;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Space {
    Occupied(Value),
    Empty,
}

impl Space {
    pub fn is_occupied(self) -> bool {
        match self {
            Self::Empty => false,
            _ => true,
        }
    }
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Self::Occupied(v) => v.to_string(),
            Self::Empty => ".".to_string(),
        };
        write!(f, "{}", repr)
    }
}

impl From<u8> for Space {
    fn from(number: u8) -> Self {
        match number {
            0 => Self::Empty,
            _ => Self::Occupied(Value::from(number)),
        }
    }
}
