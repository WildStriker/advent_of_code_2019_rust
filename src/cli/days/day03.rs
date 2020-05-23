use clap::Clap;

/// Day 3: Crossed Wires
#[derive(Clap)]
pub struct Day03 {
    #[clap(subcommand)]
    pub parts: super::shared::Parts,
}
