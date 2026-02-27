use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::ClearType;
use crossterm::{cursor, event, execute, terminal};
use std::io;
use std::io::{stdout, Write};
use std::time::Duration;
use vellum_app::actions::{Action, ActionResult};
use vellum_app::application::Application;

struct TerminalView {
    view_size: (usize, usize),
}

impl TerminalView {
    fn new() -> Self {
        let view_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self { view_size }
    }

    fn draw_rows(&self) {
        for i in 0..self.view_size.1 {
            print!("~");
            if i < self.view_size.1 - 1 {
                println!("\r")
            }
            stdout()
                .flush()
                .expect("Unable to flush terminal row output.");
        }
    }

    fn clear_screen() -> io::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    fn refresh_screen(&self) -> io::Result<()> {
        Self::clear_screen()?;
        self.draw_rows();
        execute!(stdout(), cursor::MoveTo(0, 0))
    }
}

pub struct TUI {
    view: TerminalView,
    app: Application,
}

impl Default for TUI {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TUI {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable terminal raw mode.");
        TerminalView::clear_screen().expect("Unable to clear screen.");
    }
}

impl TUI {
    pub fn new() -> Self {
        Self {
            view: TerminalView::new(),
            app: Application::new(),
        }
    }

    fn read_key(&self, poll_duration_ms: u64) -> io::Result<KeyEvent> {
        loop {
            self.view
                .refresh_screen()
                .expect("Failed to refresh screen");
            if event::poll(Duration::from_millis(poll_duration_ms))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }

    fn process_keypress(&mut self) -> io::Result<bool> {
        match self.read_key(500)? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => return Ok(false),
            KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                if let ActionResult::CursorShifted { line, col, .. } =
                    self.app.execute_action(Action::ShiftCursorUp)
                {
                    execute!(stdout(), cursor::MoveTo(line as u16, col as u16))?;
                }
            }
            KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                if let ActionResult::CursorShifted { line, col, .. } =
                    self.app.execute_action(Action::ShiftCursorDown)
                {
                    execute!(stdout(), cursor::MoveTo(line as u16, col as u16))?;
                }
            }
            KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                if let ActionResult::CursorShifted { line, col, .. } =
                    self.app.execute_action(Action::ShiftCursorLeft)
                {
                    execute!(stdout(), cursor::MoveTo(line as u16, col as u16))?;
                }
            }
            KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                if let ActionResult::CursorShifted { line, col, .. } =
                    self.app.execute_action(Action::ShiftCursorRight)
                {
                    execute!(stdout(), cursor::MoveTo(line as u16, col as u16))?;
                }
            }
            _ => {}
        }
        Ok(true)
    }

    pub fn run(&mut self) -> io::Result<bool> {
        terminal::enable_raw_mode().expect("Failed to enable terminal raw mode."); // TODO: Is this ok to be here?
        self.process_keypress()
    }
}
