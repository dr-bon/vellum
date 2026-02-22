use serde::Deserialize;
use crossterm::{cursor, event, execute};
use vellum_core::editor::Editor;

#[derive(Deserialize)]
pub enum Action {
    ShiftCursorLeft,
    ShiftCursorRight,
    ShiftCursorUp,
    ShiftCursorDown,
    ShiftCursorDocStart,
    ShiftCursorDocEnd,
    ShiftCursorLineStart,
    ShiftCursorLineEnd,
    ShiftCursorTokenStart,
    ShiftCursorTokenEnd,
    Save,
    Open,
    Close,
    Delete,
    DeleteLine,
    Insert,
    Cut,
    Copy,
    Paste,
    Undo,
    Redo,
    Select,
    SelectAll,
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Indent,
    Dedent,
    Search,
    Exit,
}