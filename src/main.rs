use aoc2018::{self, bail, Error, Reader};
use clap::Parser;
use std::path::PathBuf;
use std::process::exit;
use std::{fs, io};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Day
    day: usize,

    /// Optional path to input file; if not supplied will read from stdin
    input: Option<PathBuf>,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        let mut e: &dyn std::error::Error = &e;
        while let Some(source) = e.source() {
            eprintln!("  - caused by: {}", source);
            e = source;
        }
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    let args = Args::parse();

    let stdin = io::stdin();

    let input = match args.input {
        Some(path) => {
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            Reader::File(reader)
        }
        None => {
            let guard = stdin.lock();
            Reader::Stdin(guard)
        }
    };

    let (answer1, answer2) = match args.day {
        1 => aoc2018::day01::run(input)?,
        n if n > 0 && n < 26 => bail!("Day {} is not yet implemented.", n),
        _ => bail!("Day must be between 1 and 25, inclusive."),
    };

    println!("{}", answer1);
    println!("{}", answer2);

    Ok(())
}
