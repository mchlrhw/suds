use std::{collections::HashSet, convert::TryInto, fmt, num::ParseIntError, str::FromStr};

use crate::{space::Space, value::Value};

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

                let all_possible = {
                    let mut _set = HashSet::new();
                    _set.insert(Value::One);
                    _set.insert(Value::Two);
                    _set.insert(Value::Three);
                    _set.insert(Value::Four);
                    _set.insert(Value::Five);
                    _set.insert(Value::Six);
                    _set.insert(Value::Seven);
                    _set.insert(Value::Eight);
                    _set.insert(Value::Nine);

                    _set
                };

                for value in all_possible.difference(&constraints) {
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

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut val = String::new();
        val.push_str(" _____________________________\n");
        for y in 0..=8 {
            val.push('|');
            for x in 0..=8 {
                let space = self.get(x, y);
                val.push_str(&format!(" {} ", space));
                if (x + 1) % 3 == 0 {
                    val.push('|');
                }
            }
            val.push('\n');
            if (y + 1) % 3 == 0 {
                val.push_str("|_____________________________|\n");
            }
        }
        write!(f, "{}", val)
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
            if c.is_whitespace() {
                continue;
            }
            numbers.push(c.to_string().parse()?);
        }

        Ok(Self::from(numbers))
    }
}
