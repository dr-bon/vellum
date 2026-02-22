use ropey::Rope;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use unicode_segmentation::UnicodeSegmentation;

pub enum DocumentPosition {
    DocStart { line: usize, col: usize, idx: usize },
    DocEnd { line: usize, col: usize, idx: usize },
    LineStart { line: usize, col: usize, idx: usize },
    LineEnd { line: usize, col: usize, idx: usize },
    LineCol { line: usize, col: usize, idx: usize },
}

pub struct Line {
    pub start_idx: usize,
    pub len_chars: usize,
}

pub struct DocumentBuffer {
    contents: Rope,
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

    pub fn to_string(&self) -> String {
        self.contents.to_string()
    }

    pub fn from_file(path: &Path) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let rope = Rope::from_reader(reader).map_err(std::io::Error::other)?;
        Ok(DocumentBuffer { contents: rope })
    }

    pub fn to_file(&self, path: &Path) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.contents
            .write_to(&mut writer)
            .map_err(std::io::Error::other)?;
        Ok(())
    }

    pub fn line_count(&self) -> usize {
        self.contents.len_lines()
    }

    pub fn char_count(&self) -> usize {
        self.contents.len_chars()
    }

    pub fn get_position(&self, char_idx: usize) -> DocumentPosition {
        if char_idx == 0 {
            DocumentPosition::DocStart {
                line: 0,
                col: 0,
                idx: 0,
            }
        } else if char_idx >= self.char_count() {
            DocumentPosition::DocEnd {
                line: self.line_count() - 1,
                col: self.contents.line(self.line_count() - 1).len_chars(),
                idx: self.char_count() - 1,
            }
        } else {
            let line_idx = self.contents.char_to_line(char_idx);
            let line_char_idx = self.contents.line_to_char(line_idx);
            let col = char_idx - line_char_idx;
            if col == 0 {
                DocumentPosition::LineStart {
                    line: line_idx,
                    col,
                    idx: line_char_idx,
                }
            } else if col >= self.contents.line(line_idx).len_chars() {
                DocumentPosition::LineEnd {
                    line: line_idx,
                    col,
                    idx: char_idx,
                }
            } else {
                DocumentPosition::LineCol {
                    line: line_idx,
                    col,
                    idx: char_idx,
                }
            }
        }
    }

    pub fn get_end(&self) -> (usize, usize, usize) {
        (
            self.line_count() - 1,
            self.contents.line(self.line_count() - 1).len_chars(),
            self.char_count() - 1,
        )
    }

    pub fn get_start(&self) -> (usize, usize, usize) {
        (0, 0, 0)
    }

    pub fn line(&self, char_idx: usize) -> Line {
        let line = self.contents.char_to_line(char_idx);
        Line {
            start_idx: self.contents.line_to_char(line),
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
}

// TODO: Multi-buffer (for multiple editors)
