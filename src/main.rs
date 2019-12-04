use std::path::PathBuf;
use std::process::exit;
use std::{fs, io};
use structopt::StructOpt;

use aoc2019::{self, bail, Error, Reader};

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2019", about = "Daily problems from Advent of Code 2019.")]
struct Opt {
    /// Day
    day: usize,

    /// Optional path to input file; if not suplied will read from stdin
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);

        let mut e: &dyn std::error::Error = &e;
        while let Some(source) = e.source() {
            eprintln!("{}", e);
            e = source;
        }
        exit(1)
    }
}

fn run() -> Result<(), Error> {
    let stdin = io::stdin();
    let opt = Opt::from_args();

    let input = match opt.input {
        Some(path) => {
            let file = fs::File::open(path).unwrap();
            let reader = io::BufReader::new(file);
            Reader::File(reader)
        }
        None => {
            let guard = stdin.lock();
            Reader::Stdin(guard)
        }
    };

    match opt.day {
        1 => aoc2019::day01::run(input)?,
        2 => aoc2019::day02::run(input)?,
        3 => aoc2019::day03::run(input)?,
        n if n > 1 && n < 26 => bail!("This day isn't implemented."),
        _ => bail!("Day must been between 1 and 25, inclusive."),
    }
    Ok(())
}
