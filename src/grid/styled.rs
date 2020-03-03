use crossterm::style::{style, Attribute, Color, StyledContent};

use crate::space::Space;

use super::Grid;

const TOP_LEFT: char = ',';
const TOP_RIGHT: char = ',';
const BOTTOM_LEFT: char = '\'';
const BOTTOM_RIGHT: char = '\'';
const OUTSIDE_ACROSS: char = '-';
const INSIDE_ACROSS: char = '-';
const OUTSIDE_DOWN: char = '|';
const INSIDE_DOWN: char = '|';
const INSIDE_INTERSECTION: char = '+';

impl Grid {
    pub fn to_styled(&self) -> Vec<StyledContent<String>> {
        let mut val = vec![];

        val.push(style(TOP_LEFT.to_string()));
        for _ in 0..29 {
            val.push(style(OUTSIDE_ACROSS.to_string()));
        }
        val.push(style(TOP_RIGHT.to_string()));
        val.push(style("\n".to_string()));

        for y in 0..9 {
            val.push(style(OUTSIDE_DOWN.to_string()));
            for x in 0..9 {
                let space = self.get(x, y);
                let styled = match space {
                    Space::Occupied(v) => style(format!(" {} ", v))
                        .with(v.color())
                        .attribute(Attribute::Bold),
                    Space::Empty => style(format!(" {} ", space)).with(Color::Grey),
                };
                val.push(styled);
                if x == 8 {
                    break;
                }
                if (x + 1) % 3 == 0 {
                    val.push(style(INSIDE_DOWN.to_string()));
                }
            }
            val.push(style(OUTSIDE_DOWN.to_string()));
            val.push(style('\n'.to_string()));
            if y == 8 {
                break;
            }
            if (y + 1) % 3 == 0 {
                val.push(style(OUTSIDE_DOWN.to_string()));
                for _ in 0..2 {
                    for _ in 0..9 {
                        val.push(style(INSIDE_ACROSS.to_string()));
                    }
                    val.push(style(INSIDE_INTERSECTION.to_string()));
                }
                for _ in 0..9 {
                    val.push(style(INSIDE_ACROSS.to_string()));
                }
                val.push(style(OUTSIDE_DOWN.to_string()));
                val.push(style("\n".to_string()));
            }
        }

        val.push(style(BOTTOM_LEFT.to_string()));
        for _ in 0..29 {
            val.push(style(OUTSIDE_ACROSS.to_string()));
        }
        val.push(style(BOTTOM_RIGHT.to_string()));
        val.push(style("\n".to_string()));

        val
    }
}
