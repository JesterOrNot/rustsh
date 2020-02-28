mod lib;

use std::io::Write;
use std::io::{stdout, stdin};

fn main() {
    let mut input = String::new();
    print!(">>> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input);
    lib::lex(&input);
}
