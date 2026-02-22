use crate::document::{DocumentBuffer, DocumentPosition};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    TokenLeft,
    TokenRight,
    LineStart,
    LineEnd,
    DocStart,
    DocEnd,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cursor {
    // Position within the DocumentBuffer / rope
    pub pos: usize,
    // Used to remember the starting column when shifting between rows of different column sizes
    preferred_col: Option<usize>,
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            pos: 0,
            preferred_col: Some(0),
        }
    }

    pub fn shift(&mut self, doc: &DocumentBuffer, dir: Direction, wrap_doc: bool, wrap_line: bool) {
        let doc_pos = doc.get_position(self.pos);
        match dir {
            Direction::Left => {
                match doc_pos {
                    // If pos is doc start
                    DocumentPosition::DocStart {
                        line: _,
                        col: _,
                        idx: _,
                    } => {
                        // if doc wrapping enabled, move to doc end, set preferred col
                        if wrap_doc {
                            let doc_end = doc.get_end();
                            self.pos = doc_end.2;
                            self.preferred_col = Some(doc_end.1);
                        }
                        // else, don't move
                    }
                    // if pos is line start
                    DocumentPosition::LineStart {
                        line,
                        col: _,
                        idx: _,
                    } => {
                        // if line wrapping enabled, move to end of prev line, set preferred col
                        if wrap_line {
                            // DocStart accounts for line == 0 so we can safely ignore it here
                            let prev_line = doc.line(line - 1);
                            self.pos = prev_line.start_idx + prev_line.len_chars;
                            self.preferred_col = Some(prev_line.len_chars);
                        }
                        // else, don't move
                    }
                    // move left, preferred col -= 1
                    DocumentPosition::DocEnd { line: _, col, idx }
                    | DocumentPosition::LineEnd { line: _, col, idx }
                    | DocumentPosition::LineCol { line: _, col, idx } => {
                        self.pos = idx - 1;
                        self.preferred_col = Some(col - 1);
                    }
                }
            }
            Direction::Right => {
                match doc_pos {
                    // If pos is doc end
                    DocumentPosition::DocEnd {
                        line: _,
                        col: _,
                        idx: _,
                    } => {
                        // if doc wrapping enabled, move to doc start, preferred col = 0
                        if wrap_doc {
                            self.pos = 0;
                            self.preferred_col = Some(0);
                        }
                        // else, don't move
                    }
                    // if pos is line end
                    DocumentPosition::LineEnd {
                        line,
                        col: _,
                        idx: _,
                    } => {
                        // if line wrapping enabled, move to start of next line, preferred col = 0
                        if wrap_line {
                            // DocEnd accounts for line == last_line, so we can safely ignore it here
                            let next_line = doc.line(line + 1);
                            self.pos = next_line.start_idx;
                            self.preferred_col = Some(0);
                        }
                        // else, don't move
                    }
                    // move right, preferred col += 1
                    DocumentPosition::DocStart { line: _, col, idx }
                    | DocumentPosition::LineStart { line: _, col, idx }
                    | DocumentPosition::LineCol { line: _, col, idx } => {
                        self.pos = idx + 1;
                        self.preferred_col = Some(col + 1);
                    }
                }
            }
            Direction::Up => {
                match doc_pos {
                    DocumentPosition::DocStart {
                        line,
                        col: _,
                        idx: _,
                    }
                    | DocumentPosition::DocEnd {
                        line,
                        col: _,
                        idx: _,
                    }
                    | DocumentPosition::LineStart {
                        line,
                        col: _,
                        idx: _,
                    }
                    | DocumentPosition::LineEnd {
                        line,
                        col: _,
                        idx: _,
                    }
                    | DocumentPosition::LineCol {
                        line,
                        col: _,
                        idx: _,
                    } => {
                        // if first line
                        if line == 0 {
                            // if doc wrap enabled, go to last line at min(preferred_col, last_line_col)
                            if wrap_doc {
                                let last_line = doc.line(doc.line_count() - 1);
                                self.pos = last_line.start_idx
                                    + self.preferred_col.unwrap_or(0).min(last_line.len_chars);
                            }
                            // else, don't move
                        }
                        // go to prev line at min(preferred_col, prev_line_col)
                        else {
                            let prev_line = doc.line(line - 1);
                            self.pos = prev_line.start_idx
                                + self.preferred_col.unwrap_or(0).min(prev_line.len_chars);
                        }
                    }
                }
            }
            Direction::Down => {
                match doc_pos {
                    DocumentPosition::DocStart {
                        line,
                        col: _,
                        idx: _,
                    }
                    | DocumentPosition::DocEnd {
                        line,
                        col: _,
                        idx: _,
                    }
                    | DocumentPosition::LineStart {
                        line,
                        col: _,
                        idx: _,
                    }
                    | DocumentPosition::LineEnd {
                        line,
                        col: _,
                        idx: _,
                    }
                    | DocumentPosition::LineCol {
                        line,
                        col: _,
                        idx: _,
                    } => {
                        // if last line
                        if line == doc.line_count() - 1 {
                            // if doc wrap enabled, go to first line at min(preferred_col, first_line_col)
                            if wrap_doc {
                                let first_line = doc.line(0);
                                self.pos = first_line.start_idx
                                    + self.preferred_col.unwrap_or(0).min(first_line.len_chars);
                            }
                            // else, don't move
                        }
                        // go to next line at min(preferred_col, next_line_col)
                        else {
                            let next_line = doc.line(line + 1);
                            self.pos = next_line.start_idx
                                + self.preferred_col.unwrap_or(0).min(next_line.len_chars);
                        }
                    }
                }
            }
            Direction::TokenLeft => {
                // if doc start
                // if doc wrap enabled, go to TokenLeft of last token of last line, set preferred col
                // else, don't move
                // if line start
                // if line wrap enabled, go to TokenLeft of last token of prev line, set preferred col
                // else, don't move
                // else
                // go to TokenLeft, set preferred col
            }
            Direction::TokenRight => {
                // if doc end
                // if doc wrap enabled, go to TokenRight of first token of first line, set preferred col
                // else, don't move
                // if line end
                // if line wrap enabled, go to TokenRight of first token of next line, set preferred col
                // else, don't move
                // else
                // go to TokenRight, set preferred col
            }
            Direction::LineStart => {
                match doc_pos {
                    // if line start, do nothing
                    DocumentPosition::LineStart { line, col, idx } => {}
                    // go to current line start, preferred col = 0
                    DocumentPosition::DocStart { line, col, idx }
                    | DocumentPosition::DocEnd { line, col, idx }
                    | DocumentPosition::LineEnd { line, col, idx }
                    | DocumentPosition::LineCol { line, col, idx } => {
                        self.pos = doc.line(line).start_idx;
                        self.preferred_col = Some(0);
                    }
                }
            }
            Direction::LineEnd => {
                match doc_pos {
                    // if line end, do nothing
                    DocumentPosition::LineEnd { line, col, idx } => {}
                    // go to current line end, set preferred col
                    DocumentPosition::DocStart { line, col, idx }
                    | DocumentPosition::DocEnd { line, col, idx }
                    | DocumentPosition::LineStart { line, col, idx }
                    | DocumentPosition::LineCol { line, col, idx } => {
                        let doc_line = doc.line(line);
                        self.pos = doc_line.start_idx + doc_line.len_chars;
                        self.preferred_col = Some(doc_line.len_chars);
                    }
                }
            }
            Direction::DocStart => {
                match doc_pos {
                    // if doc start, do nothing
                    DocumentPosition::DocStart { line, col, idx } => {}
                    // go to doc start, preferred col = 0
                    DocumentPosition::DocEnd { line, col, idx }
                    | DocumentPosition::LineStart { line, col, idx }
                    | DocumentPosition::LineEnd { line, col, idx }
                    | DocumentPosition::LineCol { line, col, idx } => {
                        self.pos = 0;
                        self.preferred_col = Some(0);
                    }
                }
            }
            Direction::DocEnd => {
                match doc_pos {
                    // if doc end, do nothing
                    DocumentPosition::DocEnd { line, col, idx } => {}
                    // go to doc end, set preferred col
                    DocumentPosition::DocStart { line, col, idx }
                    | DocumentPosition::LineStart { line, col, idx }
                    | DocumentPosition::LineEnd { line, col, idx }
                    | DocumentPosition::LineCol { line, col, idx } => {
                        let doc_line = doc.line(doc.line_count() - 1);
                        self.pos = doc_line.start_idx + doc_line.len_chars;
                        self.preferred_col = Some(doc_line.len_chars);
                    }
                }
            }
        }
    }
}
