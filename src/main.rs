use ratatui::{DefualtTerminal, Frame};

mod sprites;

fn main() {
    print!("\x1b[2j\x1b[1;1h");
    sprites::normal();
}
