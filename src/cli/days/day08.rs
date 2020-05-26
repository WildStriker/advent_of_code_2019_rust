use clap::Clap;

/// Day 8: Space Image Format
#[derive(Clap)]
pub struct Day08 {
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
    #[clap(short = "w", long = "wide", default_value = "25")]
    pub wide: usize,
    #[clap(short = "t", long = "tall", default_value = "6")]
    pub tall: usize,
}

/// Part 2
#[derive(Clap)]
pub struct Part02 {
    #[clap(short = "w", long = "wide", default_value = "25")]
    pub wide: usize,
    #[clap(short = "t", long = "tall", default_value = "6")]
    pub tall: usize,
}
