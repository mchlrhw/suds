use std::io::{stdout, Write};

use clap::Clap;
use crossterm::{queue, style::PrintStyledContent};
use fehler::throws;
use suds::Grid;

/// Generate, solve and explore sudoku from the command line
#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(subcommand)]
    subcmd: Subcommand,
}

#[derive(Clap)]
enum Subcommand {
    #[clap(name = "generate")]
    Generate(Generate),
    #[clap(name = "solve")]
    Solve(Solve),
    #[clap(name = "explore")]
    Explore(Explore),
}

/// Generate a new sudoku puzzle
#[derive(Clap)]
struct Generate {}

/// Solve a given sudoku puzzle
#[derive(Clap)]
struct Solve {
    #[clap(short = "f", long = "file")]
    path: Option<String>,
}

/// Explore sudoku puzzles
#[derive(Clap)]
struct Explore {
    #[clap(short = "f", long = "file")]
    path: Option<String>,
}

#[throws(Box<dyn std::error::Error>)]
fn main() {
    let mut stdout = stdout();
    let opts = Opts::parse();

    match opts.subcmd {
        Subcommand::Generate(_) => {
            todo!();
        }
        Subcommand::Solve(c) => {
            let grid = match c.path {
                Some(path) => Grid::from_file(&path)?,
                None => Grid::empty(),
            };
            let grid = grid.solve().expect("Unsolvable");
            for s in grid.to_styled() {
                queue!(stdout, PrintStyledContent(s))?;
            }
            stdout.flush()?;
        }
        Subcommand::Explore(c) => {
            let grid = match c.path {
                Some(path) => Grid::from_file(&path)?,
                None => Grid::empty(),
            };
            for s in grid.to_styled() {
                queue!(stdout, PrintStyledContent(s))?;
            }
            stdout.flush()?;
            println!();
            println!("Complete: {}", grid.is_complete());
            println!("Solved: {}", grid.is_solved());
            println!();
        }
    }
}
