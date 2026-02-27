use crate::cursor::{Cursor, Direction, WrapMode};
use crate::document::{DocumentBuffer, DocumentPosition};
use std::io;
use std::io::stdout;
use std::path::Path;

pub struct Editor {
    pub contents: DocumentBuffer,
    pub cursor: Cursor,
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

impl Editor {
    pub fn new() -> Self {
        Self {
            contents: DocumentBuffer::new(),
            cursor: Cursor::new(),
        }
    }

    pub fn delete(&mut self, start: usize, end: usize) -> DocumentPosition {
        self.contents.delete(start, end);
        self.set_cursor_pos(start)
    }

    pub fn get_cursor_pos(&self) -> DocumentPosition {
        self.contents.get_position(self.cursor.pos)
    }

    pub fn set_cursor_pos(&mut self, char_idx: usize) -> DocumentPosition {
        self.cursor.set_pos(&self.contents, char_idx);
        self.get_cursor_pos()
    }

    pub fn shift_cursor(&mut self, dir: Direction, mag: usize, wrap_mode: WrapMode) -> bool {
        self.cursor.shift(&self.contents, dir, mag, wrap_mode)
    }

    // pub fn write(&mut self, text: &str, char_idx: usize) -> DocumentPosition {
    //     self.contents.write(text, char_idx);
    //     let n = text.chars().count();
    //     self.set_cursor_pos(char_idx + n)
    // }

    pub fn load_document_from_file(&mut self, filepath: &Path) {
        self.contents.load_file(filepath);
    }
}

impl io::Write for Editor {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.contents.push(s);
                Ok(s.len())
            }
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", self.contents.contents.to_string().as_str());
        stdout()
            .flush()
            .expect("Unable to flush editor contents to stdout.");
        self.contents.clear();
        out
    }
}
