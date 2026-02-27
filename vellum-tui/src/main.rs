use std::io;

pub mod tui;
use crate::tui::TUI;

fn main() -> io::Result<()> {
    let mut tui = TUI::new();
    while tui.run(500)? {}
    Ok(())
}
