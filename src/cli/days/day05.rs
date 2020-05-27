use clap::Clap;

/// Day 5: Sunny with a Chance of Asteroids
#[derive(Clap)]
pub struct Day05 {
    #[clap(subcommand)]
    pub parts: Parts,
}

#[derive(Clap)]
pub enum Parts {
    Part01(Part01),
    Part02(Part02),
}

/// Part 1
#[derive(Clap)]
pub struct Part01 {
    #[clap(short = "i", long = "input", default_value = "1")]
    pub input: isize,
}

/// Part 2
#[derive(Clap)]
pub struct Part02 {
    #[clap(short = "i", long = "input", default_value = "5")]
    pub input: isize,
}
