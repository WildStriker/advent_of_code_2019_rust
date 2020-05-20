use clap::Clap;

/// Day 2: 1202 Program Alarm
#[derive(Clap)]
pub struct Day02 {
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
    #[clap(short = "n", long = "noun", default_value = "12")]
    pub noun: i32,
    #[clap(short = "v", long = "verb", default_value = "2")]
    pub verb: i32,
}

/// Part 2
#[derive(Clap)]
pub struct Part02 {
    #[clap(short = "t", long = "target", default_value = "19690720")]
    pub target: i32,
}
