use std::error;
use std::fs;
use std::io;

use clap::Clap;

mod days;

/// Advent of Code 2019
#[derive(Clap)]
#[clap(version = "1.0", author = "Joe Soares <error698@gmail.com>")]
struct Opts {
    /// Input File, if not provided uses stdin
    #[clap(short, long)]
    input: Option<String>,

    #[clap(subcommand)]
    subcmd: days::Days,
}

pub fn run() -> Result<String, Box<dyn error::Error>> {
    let opts: Opts = Opts::parse();

    match opts.input {
        Some(filename) => {
            let reader = io::BufReader::new(fs::File::open(filename)?);
            days::run(reader, opts.subcmd)
        }
        None => {
            if atty::is(atty::Stream::Stdin) {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::Other,
                    "No Input stream found! Please provide a file by piping or using the input flag")));
            }
            let reader = io::BufReader::new(io::stdin());
            days::run(reader, opts.subcmd)
        }
    }
}
