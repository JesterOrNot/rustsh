use crossterm::{terminal::{disable_raw_mode, enable_raw_mode}, Result};
use rustsh::{execute_command, init, print_events};
use std::fs::File;
use std::io::stdin;
use std::process::exit;
use termion::is_tty;

fn main() -> Result<()> {
    init();
    if is_tty(&File::open("/dev/stdin")?) {
        enable_raw_mode()?;
        print_events()?;
        disable_raw_mode()?;
        exit(0)
    }
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    execute_command(&buffer);
    Ok(())
}
