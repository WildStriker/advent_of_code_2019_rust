mod cli;
mod days;

fn main() {
    match cli::run() {
        Ok(v) => println!("{}", v),
        Err(e) => println!("An error has occured: {}", e),
    };
}
