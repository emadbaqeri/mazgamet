use repl::start;
use std::io::{self, BufReader, BufWriter};

fn main() {
    println!("Hello! This is the Mazgamet programming language!");
    println!("Feel free to type in commands");

    let stdin = io::stdin();
    let stdout = io::stdout();

    let reader = BufReader::new(stdin.lock());
    let writer = BufWriter::new(stdout.lock());

    start(reader, writer);
}
