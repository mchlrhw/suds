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
