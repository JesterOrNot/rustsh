use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rustsh::print_events;

fn main() {
    enable_raw_mode().unwrap();
    print_events();
    disable_raw_mode().unwrap();
}
