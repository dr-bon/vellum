use crate::document::DocumentBuffer;

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
pub enum WrapMode {
    None,
    Horizontal,
    Vertical,
    All,
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
            pos: 0,                 // Character index within the document, row/col agnostic.
            preferred_col: Some(0), // The last column the cursor was in, for vertical movement remembrance
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default()
    }

    pub fn set_pos(&mut self, doc: &DocumentBuffer, new_pos: usize) {
        if new_pos != self.pos {
            self.pos = new_pos.clamp(0, doc.char_count());
            let new_line = doc.line(self.pos);
            let new_line_len = doc.line_len(new_line.line_no);
            let new_col = new_line_len.min(self.preferred_col.unwrap_or(0));
            self.preferred_col = Some(new_col)
        }
    }

    pub fn shift(
        &mut self,
        doc: &DocumentBuffer,
        dir: Direction,
        mag: usize,
        wrap_mode: WrapMode,
    ) -> bool {
        // NOTE: There are 0..doc.len_chars()+1 valid cursor positions
        let original_pos = self.pos;
        let doc_len = isize::try_from(doc.contents.len_chars())
            .expect("DocumentBuffer length too large for isize");
        if doc_len == 0 {
            self.reset();
            return self.pos != original_pos;
        }
        let cur_pos = isize::try_from(self.pos).expect("Cursor position too large for isize");
        match dir {
            Direction::Left => {
                let char_delta = -isize::try_from(mag).expect("Magnitude too large for isize");
                let wrap_horizontal = matches!(wrap_mode, WrapMode::Horizontal | WrapMode::All);
                if wrap_horizontal {
                    self.pos = usize::try_from((cur_pos + char_delta).rem_euclid(doc_len + 1))
                        .expect("Position incompatible with usize");
                    let cur_line = doc.line(self.pos);
                    self.preferred_col = Some(self.pos.saturating_sub(cur_line.start_idx));
                } else {
                    self.pos = usize::try_from((cur_pos + char_delta).max(0))
                        .expect("Position incompatible with usize");
                    let cur_line = doc.line(self.pos);
                    self.preferred_col = Some(self.pos.saturating_sub(cur_line.start_idx));
                }
            }
            Direction::Right => {
                let char_delta = isize::try_from(mag).expect("Magnitude too large for isize");
                let wrap_horizonal = matches!(wrap_mode, WrapMode::Horizontal | WrapMode::All);
                if wrap_horizonal {
                    self.pos = usize::try_from((cur_pos + char_delta).rem_euclid(doc_len + 1))
                        .expect("Position incompatible with usize");
                    let cur_line = doc.line(self.pos);
                    self.preferred_col = Some(self.pos.saturating_sub(cur_line.start_idx));
                } else {
                    self.pos = usize::try_from((cur_pos + char_delta).min(doc_len))
                        .expect("Position incompatible with usize");
                    let cur_line = doc.line(self.pos);
                    self.preferred_col = Some(self.pos.saturating_sub(cur_line.start_idx));
                }
            }
            Direction::Up => {
                let wrap_vertical = matches!(wrap_mode, WrapMode::Vertical | WrapMode::All);
                let mut line = doc.contents.char_to_line(self.pos);
                let line_start = doc.contents.line_to_char(line);
                let cur_col = self.pos.saturating_sub(line_start);
                let goal_col = self.preferred_col.unwrap_or(cur_col);
                let total_lines = doc.contents.len_lines();
                if total_lines == 0 {
                    self.pos = 0;
                    self.preferred_col = Some(0);
                    return self.pos != original_pos;
                }
                for _ in 0..mag {
                    if line == 0 {
                        if wrap_vertical {
                            line = total_lines - 1;
                        } else {
                            break;
                        }
                    } else {
                        line -= 1;
                    }
                }
                let line_slice = doc.contents.line(line);
                let mut line_len = line_slice.len_chars();
                if line_len > 0 && line_slice.char(line_len - 1) == '\n' {
                    line_len -= 1;
                }
                let target_col = goal_col.min(line_len);
                let target_pos = doc.contents.line_to_char(line) + target_col;
                self.pos = target_pos;
                self.preferred_col = Some(goal_col);
            }
            Direction::Down => {
                let wrap_vertical = matches!(wrap_mode, WrapMode::Vertical | WrapMode::All);
                let mut line = doc.contents.char_to_line(self.pos);
                let line_start = doc.contents.line_to_char(line);
                let cur_col = self.pos.saturating_sub(line_start);

                let goal_col = self.preferred_col.unwrap_or(cur_col);

                let total_lines = doc.contents.len_lines();
                if total_lines == 0 {
                    self.pos = 0;
                    self.preferred_col = Some(0);
                    return self.pos != original_pos;
                }

                for _ in 0..mag {
                    if line + 1 >= total_lines {
                        if wrap_vertical {
                            line = 0;
                        } else {
                            break;
                        }
                    } else {
                        line += 1;
                    }
                }

                let line_slice = doc.contents.line(line);
                let mut line_len = line_slice.len_chars();
                if line_len > 0 && line_slice.char(line_len - 1) == '\n' {
                    line_len -= 1;
                }

                let target_col = goal_col.min(line_len);
                let target_pos = doc.contents.line_to_char(line) + target_col;

                self.pos = target_pos;
                self.preferred_col = Some(goal_col);
            }
            _ => {}
        }
        self.pos != original_pos
    }
}
