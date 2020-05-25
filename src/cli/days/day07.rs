use clap::Clap;

/// Day 7: Amplification Circuit
#[derive(Clap)]
pub struct Day07 {
    #[clap(subcommand)]
    pub parts: super::shared::Parts,
}
