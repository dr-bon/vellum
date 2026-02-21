use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{cursor, event, execute, terminal, queue};
use std::time::Duration;
use crossterm::terminal::ClearType;
use std::io::stdout;
use std::io::Write;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
        Output::clear_screen().expect("Could not clear screen");
    }
}

struct Output {
    win_size: (usize, usize),
    editor_contents: EditorContents,
    cursor_controller: CursorController,
}

impl Output {
    fn new() -> Self {
        let win_size = terminal::size().map(|(x, y)| (x as usize, y as usize)).unwrap();
        Self { win_size, editor_contents: EditorContents::new(), cursor_controller: CursorController::new(win_size) }
    }

    fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    fn move_cursor(&mut self, direction: KeyCode) {
        self.cursor_controller.move_cursor(direction);
    }
    
    fn draw_rows(&mut self) {
        let screen_rows = self.win_size.1;
        let screen_cols = self.win_size.0;
        for i in 0..screen_rows {
            if i == screen_rows / 3 {
                let mut welcome = format!("Flow Editor --- Version {}", 1);
                if welcome.len() > screen_cols {
                    welcome.truncate(screen_cols);
                }
                let mut padding = (screen_cols - welcome.len()) / 2;
                if padding != 0 {
                    self.editor_contents.push('~');
                    padding -= 1;
                }
                (0..padding).for_each(|_| self.editor_contents.push(' '));
                self.editor_contents.push_str(&welcome);
            } else {
                self.editor_contents.push('~');
            }
            queue!(self.editor_contents, terminal::Clear(ClearType::UntilNewLine)).unwrap();
            if i < screen_rows - 1 {
                self.editor_contents.push_str("\r\n");
            }
            stdout().flush();
        }
    }
    fn refresh_screen(&mut self) -> crossterm::Result<()> {
        queue!(self.editor_contents, cursor::Hide, cursor::MoveTo(0, 0))?;
        self.draw_rows();
        let cursor_x = self.cursor_controller.x;
        let cursor_y = self.cursor_controller.y;
        queue!(self.editor_contents, cursor::MoveTo(cursor_x as u16, cursor_y as u16), cursor::Show)?;
        self.editor_contents.flush()
    }

}

struct Reader;

impl Reader {

    fn read_key(&self) -> crossterm::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}

struct Editor {
    reader: Reader,
    output: Output,
}

impl Editor {
    fn new() -> Self {
        Self { reader: Reader, output: Output::new() }
    }

    fn process_keypress(&mut self) -> crossterm::Result<bool> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
            } => return Ok(false),
            KeyEvent {
                code: direction @ (KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right | KeyCode::Home | KeyCode::End),
                modifiers: event::KeyModifiers::NONE,
            } => self.output.move_cursor(direction),
            KeyEvent {
                code: val @ (KeyCode::PageUp | KeyCode::PageDown),
                modifiers: event::KeyModifiers::NONE,
            } => (0..self.output.win_size.1).for_each(|_| {
                self.output.move_cursor(if  matches!(val, KeyCode::PageUp) { KeyCode::Up } else { KeyCode::Down });
            }),
            _ => {}
        }
        Ok(true)
    }

    fn run(&mut self) -> crossterm::Result<bool> {
        self.output.refresh_screen()?;
        self.process_keypress()
    }
}

struct EditorContents {
    content: String,
}

impl EditorContents {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    fn push(&mut self, c: char) {
        self.content.push(c)
    }

    fn push_str(&mut self, s: &str) {
        self.content.push_str(s)
    }
}

impl std::io::Write for EditorContents {

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(std::io::ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}


struct CursorController {
    x: usize,
    y: usize,
    screen_rows: usize,
    screen_cols: usize,
}

impl CursorController {
    fn new(win_size: (usize, usize)) -> Self {
        Self { x: 0, y: 0, screen_rows: win_size.1, screen_cols: win_size.0 }
    }

    fn move_cursor(&mut self, direction: KeyCode) {
        match direction {
            KeyCode::Up => {
                self.y = self.y.saturating_sub(1);
            }
            KeyCode::Left => {
                if self.x != 0 {
                    self.x -= 1;
                }
            }
            KeyCode::Down => {
                if self.y != self.screen_rows - 1 {
                    self.y += 1;
                }
            }
            KeyCode::Right => {
                if self.x != self.screen_cols - 1 {
                    self.x += 1;
                }
            }
            KeyCode::Home => {
                self.x = 0;
            }
            KeyCode::End => {
                self.x = self.screen_cols - 1;
            }
            _ => unimplemented!(),
        }
    }
}

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    let mut editor = Editor::new();
    while editor.run()? {}
    Ok(())
}
