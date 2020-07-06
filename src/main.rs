use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rustsh::{execute_command, init, print_events};
use std::fs::File;
use std::io::stdin;
use std::process::exit;
use termion::is_tty;
use std::io;

fn main() -> io::Result<()> {
    init();
    if is_tty(&File::open("/dev/stdin")?) {
        enable_raw_mode().unwrap();
        print_events();
        disable_raw_mode().unwrap();
        exit(0)
    }
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    execute_command(&buffer);
    Ok(())
}
