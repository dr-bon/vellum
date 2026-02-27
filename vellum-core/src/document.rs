use ropey::Rope;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use unicode_segmentation::UnicodeSegmentation;

pub enum PositionKind {
    DocStart,
    DocEnd,
    LineStart,
    LineEnd,
    LineCol,
}

pub struct DocumentPosition {
    pub kind: PositionKind,
    pub line: usize,
    pub col: usize,
    pub idx: usize,
}

pub struct Line {
    pub start_idx: usize,
    pub line_no: usize,
    pub len_chars: usize,
}

pub struct DocumentBuffer {
    pub contents: Rope,
}

impl Default for DocumentBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentBuffer {
    pub fn new() -> Self {
        DocumentBuffer {
            contents: Rope::new(),
        }
    }

    pub fn from_string(text: &str) -> Self {
        let rope = Rope::from_str(text);
        DocumentBuffer { contents: rope }
    }

    pub fn from_file(path: &Path) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let rope = Rope::from_reader(reader).map_err(std::io::Error::other)?;
        Ok(DocumentBuffer { contents: rope })
    }

    pub fn load_file(&mut self, path: &Path) {
        let file = File::open(path).expect("TODO: Fix this later");
        let reader = BufReader::new(file);
        self.contents = Rope::from_reader(reader).expect("TODO: Fix this later");
    }

    pub fn to_file(&self, path: &Path) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.contents
            .write_to(&mut writer)
            .map_err(std::io::Error::other)?;
        Ok(())
    }

    pub fn line_len(&self, line_idx: usize) -> usize {
        self.contents.line(line_idx).len_chars()
    }

    pub fn last_line_idx(&self) -> usize {
        self.line_count() - 1
    }

    pub fn last_char_idx(&self) -> usize {
        self.char_count() - 1
    }

    pub fn line_count(&self) -> usize {
        self.contents.len_lines()
    }

    pub fn char_count(&self) -> usize {
        self.contents.len_chars()
    }

    pub fn get_position(&self, char_idx: usize) -> DocumentPosition {
        if char_idx == 0 {
            DocumentPosition {
                kind: PositionKind::DocStart,
                line: 0,
                col: 0,
                idx: 0,
            }
        } else if char_idx >= self.char_count() {
            let last_line_idx = self.last_line_idx();
            DocumentPosition {
                kind: PositionKind::DocEnd,
                line: last_line_idx,
                col: self.line_len(last_line_idx),
                idx: self.last_char_idx(),
            }
        } else {
            let line_idx = self.contents.char_to_line(char_idx);
            let line_start_idx = self.contents.line_to_char(line_idx);
            let col = char_idx - line_start_idx;
            if col == 0 {
                DocumentPosition {
                    kind: PositionKind::LineStart,
                    line: line_idx,
                    col,
                    idx: line_start_idx,
                }
            } else if col >= self.line_len(line_idx) {
                DocumentPosition {
                    kind: PositionKind::LineEnd,
                    line: line_idx,
                    col,
                    idx: char_idx,
                }
            } else {
                DocumentPosition {
                    kind: PositionKind::LineCol,
                    line: line_idx,
                    col,
                    idx: char_idx,
                }
            }
        }
    }

    pub fn line(&self, char_idx: usize) -> Line {
        let line = self.contents.char_to_line(char_idx);
        Line {
            start_idx: self.contents.line_to_char(line),
            line_no: self.contents.char_to_line(char_idx),
            len_chars: self.contents.line(line).len_chars(),
        }
    }

    pub fn prev_token_start(&self, char_idx: usize) -> usize {
        if char_idx == 0 {
            return 0;
        }
        let text = self.contents.slice(..char_idx).to_string();
        for (i, _) in text.unicode_word_indices().rev() {
            if i < char_idx {
                return i;
            }
        }
        0
    }

    pub fn next_token_start(&self, char_idx: usize) -> usize {
        if char_idx >= self.char_count() {
            return self.char_count();
        }
        let text = self.contents.slice(char_idx..).to_string();
        for (i, _) in text.unicode_word_indices() {
            if i > 0 {
                // Return the index relative to the original document
                return char_idx + i;
            }
        }
        self.char_count()
    }

    pub fn write(&mut self, txt: &str, char_idx: usize) {
        self.contents.insert(char_idx, txt);
    }

    pub fn delete(&mut self, start: usize, end: usize) {
        self.contents.remove(start..end);
    }
}
