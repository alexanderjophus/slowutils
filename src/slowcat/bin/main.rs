use std::io::Write;

use clap::{arg, command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File to display
    file: String,
    /// Delay between characters in milliseconds
    #[arg(short, long, default_value_t = 100)]
    delay: u64,
}

fn main() {
    let args = Args::parse();

    let contents =
        std::fs::read_to_string(args.file).expect("Something went wrong reading the file");
    for c in contents.chars() {
        print!("{}", c);
        std::io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(args.delay));
    }
}
