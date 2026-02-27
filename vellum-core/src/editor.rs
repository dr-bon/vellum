use crate::cursor::{Cursor, Direction, WrapMode};
use crate::document::{DocumentBuffer, DocumentPosition};
use std::path::Path;

#[allow(dead_code)]
pub struct Editor {
    view_size: (usize, usize),
    pub contents: DocumentBuffer,
    pub cursor: Cursor,
}

impl Editor {
    pub fn new(view_size: (usize, usize)) -> Self {
        Self {
            view_size,
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

    pub fn write(&mut self, text: &str, char_idx: usize) -> DocumentPosition {
        self.contents.write(text, char_idx);
        let n = text.chars().count();
        self.set_cursor_pos(char_idx + n)
    }

    pub fn load_document_from_file(&mut self, filepath: &Path) {
        self.contents.load_file(filepath);
    }
}
