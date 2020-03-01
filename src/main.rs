use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rustsh::{init, print_events};

fn main() {
    init();
    enable_raw_mode().unwrap();
    print_events();
    disable_raw_mode().unwrap();
}
