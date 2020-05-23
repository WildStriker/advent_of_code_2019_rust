use clap::Clap;

/// Day 4: Secure Container
#[derive(Clap)]
pub struct Day04 {
    #[clap(subcommand)]
    pub parts: super::shared::Parts,
}
