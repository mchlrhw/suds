use std::{
    collections::HashSet,
    convert::TryInto,
    fmt,
    fs::File,
    io::{prelude::*, BufReader},
    num::ParseIntError,
    ops::Not,
    str::FromStr,
};

use fehler::throws;

use crate::{space::Space, value::{ALL_VALUES, Value}};

mod styled;

#[derive(Clone, Copy)]
pub struct Grid {
    spaces: [Space; 81],
}

impl Grid {
    pub fn empty() -> Self {
        Self {
            spaces: [Space::Empty; 81],
        }
    }

    fn get(&self, x: u8, y: u8) -> Space {
        let i: usize = (x + (y * 9)).try_into().unwrap();

        self.spaces[i]
    }

    fn set(&mut self, x: u8, y: u8, v: Value) {
        let i: usize = (x + (y * 9)).try_into().unwrap();

        self.spaces[i] = Space::Occupied(v);
    }

    fn set_empty(&mut self, x: u8, y: u8) {
        let i: usize = (x + (y * 9)).try_into().unwrap();

        self.spaces[i] = Space::Empty;
    }

    fn column_constraints(&self, x: u8) -> Vec<Value> {
        let mut c = vec![];
        for y in 0..=8 {
            let val = match self.get(x, y) {
                Space::Occupied(v) => v,
                _ => continue,
            };
            c.push(val);
        }

        c
    }

    fn row_constraints(&self, y: u8) -> Vec<Value> {
        let mut r = vec![];
        for x in 0..=8 {
            let val = match self.get(x, y) {
                Space::Occupied(v) => v,
                _ => continue,
            };
            r.push(val);
        }

        r
    }

    fn square_constraints(&self, x: u8, y: u8) -> Vec<Value> {
        let square_x = x / 3;
        let square_y = y / 3;

        let mut s = vec![];
        for local_y in 0..=2 {
            let grid_y = local_y + (square_y * 3);
            for local_x in 0..=2 {
                let grid_x = local_x + (square_x * 3);
                let val = match self.get(grid_x, grid_y) {
                    Space::Occupied(v) => v,
                    _ => continue,
                };
                s.push(val);
            }
        }

        s
    }

    pub fn solve(mut self) -> Option<Self> {
        for y in 0..=8 {
            for x in 0..=8 {
                let space = self.get(x, y);
                if space.is_occupied() {
                    continue;
                }

                let mut constraints = HashSet::new();
                constraints.extend(self.column_constraints(x));
                constraints.extend(self.row_constraints(y));
                constraints.extend(self.square_constraints(x, y));
                constraints = constraints;

                for value in ALL_VALUES.difference(&constraints) {
                    self.set(x, y, *value);
                    match self.solve() {
                        Some(solution) => return Some(solution),
                        None => self.set_empty(x, y),
                    }
                }

                return None;
            }
        }

        Some(self)
    }
}

impl PartialEq for Grid {
    fn eq(&self, other: &Self) -> bool {
        self.spaces.iter().eq(other.spaces.iter())
    }
}

impl Eq for Grid {}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.spaces.to_vec())
    }
}

impl fmt::Display for Grid {
    #[throws(fmt::Error)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) {
        for val in self.to_styled() {
            val.content().fmt(f)?;
        }
    }
}

impl From<Vec<u8>> for Grid {
    fn from(numbers: Vec<u8>) -> Self {
        let mut spaces = [Space::Empty; 81];
        for (i, number) in numbers.iter().enumerate() {
            spaces[i] = Space::from(*number);
        }

        Self { spaces }
    }
}

impl FromStr for Grid {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = vec![];
        for c in s.chars() {
            let c = if c == '.' { '0' } else { c };
            if c.is_ascii_digit().not() {
                continue;
            }
            numbers.push(c.to_string().parse()?);
        }

        Ok(Self::from(numbers))
    }
}

impl Grid {
    #[throws(Box<dyn std::error::Error>)]
    pub fn from_file(path: &str) -> Self {
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        Self::from_str(&contents)?
    }
}

#[cfg(test)]
mod tests {
    use fehler::throws;

    use super::*;

    #[test]
    #[throws(ParseIntError)]
    fn round_trip() {
        #[rustfmt::skip]
        let expected =
r",-----------------------------,
| 4  8  3 | 9  2  1 | 6  5  7 |
| 9  6  7 | 3  4  5 | 8  2  1 |
| 2  5  1 | 8  7  6 | 4  9  3 |
|---------+---------+---------|
| 5  4  8 | 1  3  2 | 9  7  6 |
| 7  2  9 | 5  6  4 | 1  3  8 |
| 1  3  6 | 7  9  8 | 2  4  5 |
|---------+---------+---------|
| 3  7  2 | 6  8  9 | 5  1  4 |
| 8  1  4 | 2  5  3 | 7  6  9 |
| 6  9  5 | 4  1  7 | 3  8  2 |
'-----------------------------'
";
        let grid: Grid = expected.parse()?;
        assert_eq!(format!("{}", grid), expected);
    }

    #[test]
    #[throws(ParseIntError)]
    fn round_trip_empty() {
        #[rustfmt::skip]
        let expected =
r",-----------------------------,
| .  .  . | .  .  . | .  .  . |
| .  .  . | .  .  . | .  .  . |
| .  .  . | .  .  . | .  .  . |
|---------+---------+---------|
| .  .  . | .  .  . | .  .  . |
| .  .  . | .  .  . | .  .  . |
| .  .  . | .  .  . | .  .  . |
|---------+---------+---------|
| .  .  . | .  .  . | .  .  . |
| .  .  . | .  .  . | .  .  . |
| .  .  . | .  .  . | .  .  . |
'-----------------------------'
";
        let grid: Grid = expected.parse()?;
        assert_eq!(grid, Grid::empty());
        assert_eq!(format!("{}", grid), expected);
    }

    #[test]
    #[throws(ParseIntError)]
    fn zeros_as_empty() {
        #[rustfmt::skip]
        let expected =
r",-----------------------------,
| 0  0  0 | 0  0  0 | 0  0  0 |
| 0  0  0 | 0  0  0 | 0  0  0 |
| 0  0  0 | 0  0  0 | 0  0  0 |
|---------+---------+---------|
| 0  0  0 | 0  0  0 | 0  0  0 |
| 0  0  0 | 0  0  0 | 0  0  0 |
| 0  0  0 | 0  0  0 | 0  0  0 |
|---------+---------+---------|
| 0  0  0 | 0  0  0 | 0  0  0 |
| 0  0  0 | 0  0  0 | 0  0  0 |
| 0  0  0 | 0  0  0 | 0  0  0 |
'-----------------------------'
";
        let grid: Grid = expected.parse()?;
        assert_eq!(grid, Grid::empty());
    }
}
