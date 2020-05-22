mod cli;
pub mod days;
mod shared;

pub fn run() {
    match cli::run() {
        Ok(v) => println!("{}", v),
        Err(e) => println!("An error has occured: {}", e),
    };
}
