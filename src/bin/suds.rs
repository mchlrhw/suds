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
    #[clap(name = "gen")]
    Generate(Generate),
    #[clap(name = "solve")]
    Solve(Solve),
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

#[throws(Box<dyn std::error::Error>)]
fn main() {
    let mut stdout = stdout();
    let opts = Opts::parse();

    match opts.subcmd {
        Subcommand::Generate(_) => {
            todo!();
        }
        Subcommand::Solve(s) => {
            let grid = match s.path {
                Some(path) => Grid::from_file(&path)?,
                None => Grid::empty(),
            };
            let grid = grid.solve().expect("Unsolvable");
            for s in grid.to_styled() {
                queue!(stdout, PrintStyledContent(s))?;
            }
            stdout.flush()?;
        }
    }
}
