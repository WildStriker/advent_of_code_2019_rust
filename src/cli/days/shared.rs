use clap::Clap;

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
pub struct Part02 {}
