pub mod highlighter;
pub mod text_buffer;

use crate::config::EditorConfig;
use crate::theme::Theme;
use egui::{text::CCursor, text_edit::TextEditState, *};
use std::path::Path;
use text_buffer::TextBuffer;

/// A rich text editor for Markdown
pub struct Editor {
    buffer: TextBuffer,
    config: EditorConfig,
    has_focus: bool,
    cursor_position: (usize, usize),
    selection: Option<(usize, usize)>,
    history: EditHistory,
    dirty: bool,
    scroll_offset: Vec2,
    text_edit_state: Option<TextEditState>,
}

struct EditHistory {
    undo_stack: Vec<Edit>,
    redo_stack: Vec<Edit>,
    max_size: usize,
}

#[derive(Clone)]
struct Edit {
    old_text: String,
    new_text: String,
    position: usize,
    cursor_before: (usize, usize),
    cursor_after: (usize, usize),
}

impl Editor {
    pub fn new() -> Self {
        Self {
            buffer: TextBuffer::new(),
            config: EditorConfig::default(),
            has_focus: false,
            cursor_position: (0, 0),
            selection: None,
            history: EditHistory::new(1000),
            dirty: false,
            scroll_offset: Vec2::ZERO,
            text_edit_state: None,
        }
    }

    pub fn set_config(&mut self, config: EditorConfig) {
        self.config = config;
    }

    pub fn text(&self) -> String {
        self.buffer.as_str()
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        let text = text.into();
        self.buffer = TextBuffer::from(text);
        self.dirty = false;
        self.history.clear();
    }

    pub fn cursor_position(&self) -> (usize, usize) {
        self.cursor_position
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn open_file(&mut self, path: &Path) -> Result<(), std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        self.set_text(content);
        self.dirty = false;
        Ok(())
    }

    pub fn save_file(&mut self, path: &Path) -> Result<(), std::io::Error> {
        std::fs::write(path, self.buffer.as_str())?;
        self.dirty = false;
        Ok(())
    }

    pub fn undo(&mut self) {
        if let Some(edit) = self.history.undo() {
            self.buffer.replace_range(
                edit.position..edit.position + edit.new_text.len(),
                &edit.old_text,
            );
            self.cursor_position = edit.cursor_before;
            self.dirty = true;
        }
    }

    pub fn redo(&mut self) {
        if let Some(edit) = self.history.redo() {
            self.buffer.replace_range(
                edit.position..edit.position + edit.old_text.len(),
                &edit.new_text,
            );
            self.cursor_position = edit.cursor_after;
            self.dirty = true;
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.history.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.history.redo_stack.is_empty()
    }

    pub fn insert_text(&mut self, text: &str) {
        let cursor_byte = 0; // Simplified
        self.buffer.insert(cursor_byte, text);
        self.dirty = true;
    }

    pub fn backspace(&mut self) {
        let cursor_byte = 0; // Simplified
        if cursor_byte > 0 {
            self.buffer.delete_range(cursor_byte - 1..cursor_byte);
            self.dirty = true;
        }
    }
}

impl EditHistory {
    fn new(max_size: usize) -> Self {
        Self {
            undo_stack: Vec::with_capacity(max_size),
            redo_stack: Vec::with_capacity(max_size),
            max_size,
        }
    }

    fn push(&mut self, edit: Edit) {
        if self.undo_stack.len() >= self.max_size {
            self.undo_stack.remove(0);
        }
        self.undo_stack.push(edit);
        self.redo_stack.clear();
    }

    fn undo(&mut self) -> Option<Edit> {
        if let Some(edit) = self.undo_stack.pop() {
            self.redo_stack.push(edit.clone());
            Some(edit)
        } else {
            None
        }
    }

    fn redo(&mut self) -> Option<Edit> {
        if let Some(edit) = self.redo_stack.pop() {
            self.undo_stack.push(edit.clone());
            Some(edit)
        } else {
            None
        }
    }

    fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}
