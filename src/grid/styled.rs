use crossterm::style::{style, Attribute, Color, StyledContent};

use crate::{space::Space, value::Value};

use super::Grid;

impl Grid {
    pub fn to_styled(&self) -> Vec<StyledContent<String>> {
        let mut val = vec![];
        val.push(style(",-----------------------------,\n".to_string()));
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
                    Space::Empty => style(format!(" {} ", space)).with(Color::Grey),
                };
                val.push(styled);
                if (x + 1) % 3 == 0 {
                    val.push(style('|'.to_string()));
                }
            }
            val.push(style('\n'.to_string()));
            if y == 8 {
                break;
            }
            if (y + 1) % 3 == 0 {
                val.push(style("|---------+---------+---------|\n".to_string()));
            }
        }
        val.push(style("'-----------------------------'\n".to_string()));

        val
    }
}
