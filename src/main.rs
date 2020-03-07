use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rustsh::{execute_command, init, print_events, help};
use std::fs::File;
use std::io::stdin;
use std::process::exit;
use termion::is_tty;

fn main() {
    init();
    if is_tty(&File::open("/dev/stdin").unwrap()) {
        enable_raw_mode().unwrap();
        print_events();
        disable_raw_mode().unwrap();
        exit(0)
    }
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    match buffer.as_str() {
        "help" => help(&mut buffer, &mut 0),
        _ => print!("{}", execute_command(&buffer))
    }
}
