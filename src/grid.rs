use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    fmt,
    fs::File,
    io::{prelude::*, BufReader},
    num::ParseIntError,
    ops::Not,
    str::FromStr,
};

use fehler::throws;
use rand::prelude::*;

use crate::{
    space::Space,
    value::{Value, ALL_VALUES},
};

mod styled;

#[derive(Clone, Copy)]
pub struct Grid {
    spaces: [Space; 81],
}

fn is_solved(spaces: Vec<Space>) -> bool {
    let mut values = vec![];
    for space in spaces {
        match space {
            Space::Empty => return false,
            Space::Occupied(value) => values.push(value),
        }
    }
    values.sort();

    let mut all_values = ALL_VALUES.iter().cloned().collect::<Vec<Value>>();
    all_values.sort();

    values == all_values
}

impl Grid {
    pub fn empty() -> Self {
        Self {
            spaces: [Space::Empty; 81],
        }
    }

    pub fn new_solved() -> Self {
        let grid = Self {
            spaces: [Space::Empty; 81],
        };
        grid.solve().unwrap()
    }

    fn get(&self, x: u8, y: u8) -> Space {
        let i: usize = (x + (y * 9)).try_into().unwrap();

        self.spaces[i]
    }

    fn rows(&self) -> Vec<Vec<Space>> {
        let mut rows = vec![];
        for y in 0..=8 {
            let mut row = vec![];
            for x in 0..=8 {
                row.push(self.get(x, y));
            }
            rows.push(row);
        }

        rows
    }

    fn columns(&self) -> Vec<Vec<Space>> {
        let mut columns = vec![];
        for x in 0..=8 {
            let mut column = vec![];
            for y in 0..=8 {
                column.push(self.get(x, y));
            }
            columns.push(column);
        }

        columns
    }

    fn squares(&self) -> Vec<Vec<Space>> {
        let mut squares = vec![];
        for square_y in 0..=2 {
            for square_x in 0..=2 {
                let mut square = vec![];
                for local_y in 0..=2 {
                    let grid_y = local_y + (square_y * 3);
                    for local_x in 0..=2 {
                        let grid_x = local_x + (square_x * 3);
                        let val = self.get(grid_x, grid_y);
                        square.push(val);
                    }
                }
                squares.push(square)
            }
        }

        squares
    }

    fn set(&mut self, x: u8, y: u8, v: Value) {
        let i: usize = (x + (y * 9)).try_into().unwrap();

        self.spaces[i] = Space::Occupied(v);
    }

    fn set_empty(&mut self, x: u8, y: u8) {
        let i: usize = (x + (y * 9)).try_into().unwrap();

        self.spaces[i] = Space::Empty;
    }

    pub fn is_complete(&self) -> bool {
        self.spaces.iter().all(|&s| s.is_occupied())
    }

    pub fn is_solved(&self) -> bool {
        self.rows().iter().all(|r| is_solved(r.to_vec()))
            && self.columns().iter().all(|c| is_solved(c.to_vec()))
            && self.squares().iter().all(|s| is_solved(s.to_vec()))
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

    pub fn backtracking_solve(mut self) -> Option<Self> {
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

    pub fn stochastic_solve(mut self) -> Option<Self> {
        // 1) Find out if it's solvable, return early if not
        // TODO
        // 2) Determine which numbers are missing from the grid
        let mut counts: HashMap<Value, u8> = HashMap::new();
        for space in self.spaces.iter() {
            match space {
                Space::Empty => continue,
                Space::Occupied(value) => *counts.entry(*value).or_insert(0) += 1,
            }
        }
        let mut values = vec![];
        for value in ALL_VALUES.iter() {
            let missing = match counts.get(value) {
                Some(count) => 9 - count,
                None => 9,
            };
            for _ in 0..missing {
                values.push(value);
            }
        }
        // 3) Shuffle the numbers and insert into the empty spaces
        let self_clone = self;
        loop {
            let mut values_clone = values.clone();
            values_clone.shuffle(&mut rand::thread_rng());

            for (i, space) in self_clone.spaces.iter().enumerate() {
                match space {
                    Space::Empty => self.spaces[i] = Space::Occupied(*values_clone.pop().unwrap()),
                    Space::Occupied(_) => continue,
                }
            }
            // 4) Check for solved state, return if true, loop if not
            if self.is_solved() {
                return Some(self);
            }
            println!("{}", self);
        }
    }

    pub fn solve(self) -> Option<Self> {
        self.backtracking_solve()
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
    use test_case::test_case;

    use super::*;

    #[test_case(Grid::empty(), false)]
    #[test_case(Grid::new_solved(), true)]
    fn grid_is_complete(grid: Grid, expected: bool) {
        assert_eq!(grid.is_complete(), expected);
    }

    #[test_case(Grid::empty(), false)]
    #[test_case(Grid::new_solved(), true)]
    fn grid_is_solved(grid: Grid, expected: bool) {
        assert_eq!(grid.is_solved(), expected);
    }

    #[test]
    #[throws(ParseIntError)]
    fn round_trip() {
        #[rustfmt::skip]
        let expected = r"
┌─────────────────────────────┐
│ 3  7  5 ┆ 4  1  6 ┆ 8  9  2 │
│ 4  1  6 ┆ 8  9  2 ┆ 3  7  5 │
│ 8  9  2 ┆ 3  7  5 ┆ 4  1  6 │
│┄┄┄┄┄┄┄┄┄┼┄┄┄┄┄┄┄┄┄┼┄┄┄┄┄┄┄┄┄│
│ 7  3  4 ┆ 5  6  1 ┆ 9  2  8 │
│ 5  6  1 ┆ 9  2  8 ┆ 7  3  4 │
│ 9  2  8 ┆ 7  3  4 ┆ 5  6  1 │
│┄┄┄┄┄┄┄┄┄┼┄┄┄┄┄┄┄┄┄┼┄┄┄┄┄┄┄┄┄│
│ 1  5  3 ┆ 6  4  7 ┆ 2  8  9 │
│ 6  4  7 ┆ 2  8  9 ┆ 1  5  3 │
│ 2  8  9 ┆ 1  5  3 ┆ 6  4  7 │
└─────────────────────────────┘
";
        let expected = expected.trim_start();
        let grid: Grid = expected.parse()?;
        assert_eq!(format!("{}", grid), expected);
    }

    #[test]
    #[throws(ParseIntError)]
    fn round_trip_empty() {
        #[rustfmt::skip]
        let expected = r"
┌─────────────────────────────┐
│ .  .  . ┆ .  .  . ┆ .  .  . │
│ .  .  . ┆ .  .  . ┆ .  .  . │
│ .  .  . ┆ .  .  . ┆ .  .  . │
│┄┄┄┄┄┄┄┄┄┼┄┄┄┄┄┄┄┄┄┼┄┄┄┄┄┄┄┄┄│
│ .  .  . ┆ .  .  . ┆ .  .  . │
│ .  .  . ┆ .  .  . ┆ .  .  . │
│ .  .  . ┆ .  .  . ┆ .  .  . │
│┄┄┄┄┄┄┄┄┄┼┄┄┄┄┄┄┄┄┄┼┄┄┄┄┄┄┄┄┄│
│ .  .  . ┆ .  .  . ┆ .  .  . │
│ .  .  . ┆ .  .  . ┆ .  .  . │
│ .  .  . ┆ .  .  . ┆ .  .  . │
└─────────────────────────────┘
";
        let expected = expected.trim_start();
        let grid: Grid = expected.parse()?;
        assert_eq!(grid, Grid::empty());
        assert_eq!(format!("{}", grid), expected);
    }

    #[test]
    #[throws(ParseIntError)]
    fn zeros_as_empty() {
        #[rustfmt::skip]
        let expected = r"
┌─────────────────────────────┐
│ 0  0  0 ┆ 0  0  0 ┆ 0  0  0 │
│ 0  0  0 ┆ 0  0  0 ┆ 0  0  0 │
│ 0  0  0 ┆ 0  0  0 ┆ 0  0  0 │
│┄┄┄┄┄┄┄┄┄┼┄┄┄┄┄┄┄┄┄┼┄┄┄┄┄┄┄┄┄│
│ 0  0  0 ┆ 0  0  0 ┆ 0  0  0 │
│ 0  0  0 ┆ 0  0  0 ┆ 0  0  0 │
│ 0  0  0 ┆ 0  0  0 ┆ 0  0  0 │
│┄┄┄┄┄┄┄┄┄┼┄┄┄┄┄┄┄┄┄┼┄┄┄┄┄┄┄┄┄│
│ 0  0  0 ┆ 0  0  0 ┆ 0  0  0 │
│ 0  0  0 ┆ 0  0  0 ┆ 0  0  0 │
│ 0  0  0 ┆ 0  0  0 ┆ 0  0  0 │
└─────────────────────────────┘
";
        let expected = expected.trim_start();
        let grid: Grid = expected.parse()?;
        assert_eq!(grid, Grid::empty());
    }
}
