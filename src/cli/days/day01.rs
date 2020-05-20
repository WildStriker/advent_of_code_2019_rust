use clap::Clap;

/// Day 1: The Tyranny of the Rocket Equation
#[derive(Clap)]
pub struct Day01 {
    #[clap(subcommand)]
    pub parts: super::shared::Parts,
}
