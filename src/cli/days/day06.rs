use clap::Clap;

/// Day 6: Universal Orbit Map
#[derive(Clap)]
pub struct Day06 {
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
pub struct Part01 {}

/// Part 2
#[derive(Clap)]
pub struct Part02 {
    #[clap(short = "s", long = "start", default_value = "YOU")]
    pub start: String,
    #[clap(short = "f", long = "find", default_value = "SAN")]
    pub find: String,
}
