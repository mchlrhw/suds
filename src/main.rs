use std::{
    collections::HashSet,
    convert::TryInto,
    fmt,
    io::{stdout, Write},
    num::ParseIntError,
    str::FromStr,
};

use clap::Clap;
use crossterm::{
    queue,
    style::{style, Attribute, Color, PrintStyledContent, StyledContent},
};
use fehler::throws;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Value {
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Space {
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

#[derive(Clone, Copy)]
struct Grid {
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

impl Grid {
    pub fn to_styled(&self) -> Vec<StyledContent<String>> {
        let mut val = vec![];
        val.push(style(" _____________________________\n".to_string()));
        for y in 0..=8 {
            val.push(style('|'.to_string()));
            for x in 0..=8 {
                let space = self.get(x, y);
                let styled = match space {
                    Space::Occupied(v) => match v {
                        Value::One => style(format!(" {} ", v))
                            .with(Color::Rgb { r: 255, g: 0, b: 0 })
                            .attribute(Attribute::Bold),
                        Value::Two => style(format!(" {} ", v))
                            .with(Color::Rgb {
                                r: 255,
                                g: 120,
                                b: 0,
                            })
                            .attribute(Attribute::Bold),
                        Value::Three => style(format!(" {} ", v))
                            .with(Color::Rgb {
                                r: 255,
                                g: 255,
                                b: 0,
                            })
                            .attribute(Attribute::Bold),
                        Value::Four => style(format!(" {} ", v))
                            .with(Color::Rgb {
                                r: 120,
                                g: 255,
                                b: 0,
                            })
                            .attribute(Attribute::Bold),
                        Value::Five => style(format!(" {} ", v))
                            .with(Color::Rgb {
                                r: 0,
                                g: 180,
                                b: 80,
                            })
                            .attribute(Attribute::Bold),
                        Value::Six => style(format!(" {} ", v))
                            .with(Color::Rgb {
                                r: 0,
                                g: 150,
                                b: 255,
                            })
                            .attribute(Attribute::Bold),
                        Value::Seven => style(format!(" {} ", v))
                            .with(Color::Rgb {
                                r: 60,
                                g: 80,
                                b: 220,
                            })
                            .attribute(Attribute::Bold),
                        Value::Eight => style(format!(" {} ", v))
                            .with(Color::Rgb {
                                r: 140,
                                g: 40,
                                b: 255,
                            })
                            .attribute(Attribute::Bold),
                        Value::Nine => style(format!(" {} ", v))
                            .with(Color::Rgb {
                                r: 240,
                                g: 20,
                                b: 255,
                            })
                            .attribute(Attribute::Bold),
                    },
                    Space::Empty => style(space.to_string()).with(Color::Grey),
                };
                val.push(styled);
                if (x + 1) % 3 == 0 {
                    val.push(style('|'.to_string()));
                }
            }
            val.push(style('\n'.to_string()));
            if (y + 1) % 3 == 0 {
                val.push(style("|_____________________________|\n".to_string()));
            }
        }

        val
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

/// Generate, solve and explore sudoku from the command line
#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(subcommand)]
    subcmd: Subcommand,
}

#[derive(Clap)]
enum Subcommand {
    #[clap(name = "gen")]
    Generate(Generate),
}

/// Generate a new sudoku puzzle
#[derive(Clap)]
struct Generate {}

#[throws(crossterm::ErrorKind)]
fn main() {
    let mut stdout = stdout();
    let opts = Opts::parse();

    match opts.subcmd {
        Subcommand::Generate(_) => {
            let grid = Grid::empty();
            let grid = grid.solve().expect("Unsolvable");
            for s in grid.to_styled() {
                queue!(stdout, PrintStyledContent(s))?;
            }
            stdout.flush()?;
        }
    }
}
