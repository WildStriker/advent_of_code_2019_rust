use clap::Clap;

/// Day 9: Sensor Boost
#[derive(Clap)]
pub struct Day09 {
    #[clap(subcommand)]
    pub parts: super::shared::Parts,
}
