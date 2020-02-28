use color_lexer::print_events;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
    enable_raw_mode().unwrap();
    print_events();
    disable_raw_mode().unwrap();
}
