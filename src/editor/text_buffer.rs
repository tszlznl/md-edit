use std::ops::{Index, Range};

/// A rope-like text buffer optimized for text editing operations.
/// Stores text as a gap buffer for efficient insertions and deletions.
pub struct TextBuffer {
    /// The underlying storage (using Vec<u8> for copy_within support)
    storage: Vec<u8>,
    /// The gap start index
    gap_start: usize,
    /// The gap end index
    gap_end: usize,
    /// Cached line starts for fast line lookup
    line_starts: Vec<usize>,
    /// Whether line starts cache is dirty
    line_cache_dirty: bool,
}

impl TextBuffer {
    /// Create a new empty text buffer
    pub fn new() -> Self {
        Self {
            storage: Vec::with_capacity(1024),
            gap_start: 0,
            gap_end: 0,
            line_starts: vec![0],
            line_cache_dirty: false,
        }
    }

    /// Create a text buffer from a string
    pub fn from(text: impl Into<String>) -> Self {
        let text = text.into();
        let bytes = text.into_bytes();
        let len = bytes.len();

        let mut buffer = Self {
            storage: bytes,
            gap_start: len,
            gap_end: len,
            line_starts: vec![0],
            line_cache_dirty: true,
        };

        buffer.rebuild_line_cache();
        buffer
    }

    /// Get the total byte length of the buffer (excluding gap)
    pub fn len(&self) -> usize {
        self.storage.len() - (self.gap_end - self.gap_start)
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the content as a string (without the gap)
    pub fn as_str(&self) -> String {
        let mut result = String::with_capacity(self.len());
        result.push_str(std::str::from_utf8(&self.storage[..self.gap_start]).unwrap_or(""));
        result.push_str(std::str::from_utf8(&self.storage[self.gap_end..]).unwrap_or(""));
        result
    }

    /// Insert text at a byte position
    pub fn insert(&mut self, pos: usize, text: &str) {
        // Move gap to position
        self.move_gap(pos);

        // Ensure there's enough space in the gap
        let text_len = text.len();
        let gap_size = self.gap_end - self.gap_start;

        if text_len > gap_size {
            // Grow the buffer
            let additional = (text_len - gap_size).max(self.storage.len() / 2);
            self.grow_gap(additional);
        }

        // Insert the text
        let text_bytes = text.as_bytes();
        for (i, &byte) in text_bytes.iter().enumerate() {
            self.storage[self.gap_start + i] = byte;
        }
        self.gap_start += text_len;

        self.line_cache_dirty = true;
    }

    /// Delete a range of bytes
    pub fn delete_range(&mut self, range: Range<usize>) {
        if range.start >= range.end {
            return;
        }

        let len = self.len();
        let start = range.start.min(len);
        let end = range.end.min(len);

        // Move gap to end of deletion range
        self.move_gap(end);

        // Move gap start back to include deleted range
        self.gap_start -= end - start;

        self.line_cache_dirty = true;
    }

    /// Replace a range with new text
    pub fn replace_range(&mut self, range: Range<usize>, text: &str) {
        self.delete_range(range.clone());
        self.insert(range.start, text);
    }

    /// Get the line and column from a byte offset
    pub fn line_col_from_byte_index(&mut self, byte_index: usize) -> (usize, usize) {
        self.rebuild_line_cache_if_needed();

        // Binary search for the line
        let line = match self.line_starts.binary_search(&byte_index) {
            Ok(line) => line,
            Err(line) => line.saturating_sub(1),
        };

        let line_start = self.line_starts[line];
        let col = byte_index.saturating_sub(line_start);

        (line, col)
    }

    /// Get the byte offset from a line and column
    pub fn byte_index_from_line_col(&mut self, line: usize, col: usize) -> usize {
        self.rebuild_line_cache_if_needed();

        if line >= self.line_starts.len() {
            return self.len();
        }

        let line_start = self.line_starts[line];
        let line_end = if line + 1 < self.line_starts.len() {
            self.line_starts[line + 1]
        } else {
            self.len()
        };

        let line_len = line_end - line_start;
        line_start + col.min(line_len)
    }

    /// Get the number of lines
    pub fn line_count(&mut self) -> usize {
        self.rebuild_line_cache_if_needed();
        self.line_starts.len()
    }

    /// Get the text of a specific line (without newline)
    pub fn line_text(&mut self, line: usize) -> Option<String> {
        self.rebuild_line_cache_if_needed();

        if line >= self.line_starts.len() {
            return None;
        }

        let start = self.line_starts[line];
        let end = if line + 1 < self.line_starts.len() {
            self.line_starts[line + 1]
        } else {
            self.len()
        };

        // Remove trailing newline if present
        let text = self.substring(start, end);
        Some(text.trim_end_matches('\n').trim_end_matches('\r').to_string())
    }

    /// Get the line and column from a byte offset (mutable version for cache rebuild)
    pub fn line_col_from_byte_index_mut(&mut self, byte_index: usize) -> (usize, usize) {
        self.rebuild_line_cache_if_needed();

        // Binary search for the line
        let line = match self.line_starts.binary_search(&byte_index) {
            Ok(line) => line,
            Err(line) => line.saturating_sub(1),
        };

        let line_start = self.line_starts[line];
        let col = byte_index.saturating_sub(line_start);

        (line, col)
    }

    /// Get the number of lines (mutable version for cache rebuild)
    pub fn line_count_mut(&mut self) -> usize {
        self.rebuild_line_cache_if_needed();
        self.line_starts.len()
    }

    /// Move the gap to a specific position
    fn move_gap(&mut self, pos: usize) {
        if pos == self.gap_start {
            return;
        }

        let len = self.len();
        let pos = pos.min(len);

        if pos < self.gap_start {
            // Move gap left: copy content from left of gap to right
            let move_len = self.gap_start - pos;
            // Manual copy
            for i in 0..move_len {
                self.storage[self.gap_end - move_len + i] = self.storage[pos + i];
            }
            self.gap_start -= move_len;
            self.gap_end -= move_len;
        } else {
            // Move gap right: copy content from right of gap to left
            let move_len = pos - self.gap_start;
            // Manual copy
            for i in 0..move_len {
                self.storage[self.gap_start + i] = self.storage[self.gap_end + i];
            }
            self.gap_start += move_len;
            self.gap_end += move_len;
        }
    }

    /// Grow the gap by a specific amount
    fn grow_gap(&mut self, additional: usize) {
        let new_gap_end = self.gap_end + additional;
        self.storage.resize(new_gap_end, 0);
        // Manual copy to avoid copy_within issue
        let src_start = self.gap_end;
        let dst_start = self.gap_end + additional;
        let len = self.storage.len() - src_start;
        for i in (0..len).rev() {
            self.storage[dst_start + i] = self.storage[src_start + i];
        }
        self.gap_end = new_gap_end;
    }

    /// Get a substring from the buffer
    fn substring(&self, start: usize, end: usize) -> String {
        let mut result = String::with_capacity(end - start);

        if end <= self.gap_start || start >= self.gap_start {
            // Entirely in one segment
            if end <= self.gap_start {
                result.push_str(std::str::from_utf8(&self.storage[start..end]).unwrap_or(""));
            } else {
                let gap_offset = self.gap_end - self.gap_start;
                result.push_str(std::str::from_utf8(&self.storage[start + gap_offset..end + gap_offset]).unwrap_or(""));
            }
        } else {
            // Spans the gap
            result.push_str(std::str::from_utf8(&self.storage[start..self.gap_start]).unwrap_or(""));
            let gap_offset = self.gap_end - self.gap_start;
            result.push_str(std::str::from_utf8(&self.storage[self.gap_end..end + gap_offset]).unwrap_or(""));
        }

        result
    }

    /// Rebuild the line starts cache if dirty
    fn rebuild_line_cache_if_needed(&mut self) {
        if !self.line_cache_dirty {
            return;
        }
        self.rebuild_line_cache();
    }

    /// Rebuild the line starts cache
    fn rebuild_line_cache(&mut self) {
        self.line_starts.clear();
        self.line_starts.push(0);

        let text = self.as_str();
        for (i, c) in text.char_indices() {
            if c == '\n' {
                let next_start = i + 1;
                if next_start <= text.len() {
                    self.line_starts.push(next_start);
                }
            }
        }

        self.line_cache_dirty = false;
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<Range<usize>> for TextBuffer {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        // This is a simplified implementation
        // In a real implementation, we'd need to handle the gap
        // For now, we assume the index doesn't span the gap
        if index.end <= self.gap_start {
            &self.storage[index]
        } else {
            let gap_offset = self.gap_end - self.gap_start;
            &self.storage[index.start + gap_offset..index.end + gap_offset]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_buffer() {
        let buffer = TextBuffer::new();
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_from_string() {
        let buffer = TextBuffer::from("Hello, World!");
        assert_eq!(buffer.len(), 13);
        assert_eq!(buffer.as_str(), "Hello, World!");
    }

    #[test]
    fn test_insert() {
        let mut buffer = TextBuffer::new();
        buffer.insert(0, "Hello");
        assert_eq!(buffer.as_str(), "Hello");

        buffer.insert(5, ", World!");
        assert_eq!(buffer.as_str(), "Hello, World!");

        buffer.insert(5, " there");
        assert_eq!(buffer.as_str(), "Hello there, World!");
    }

    #[test]
    fn test_delete_range() {
        let mut buffer = TextBuffer::from("Hello, World!");
        buffer.delete_range(5..7);
        assert_eq!(buffer.as_str(), "HelloWorld!");
    }

    #[test]
    fn test_line_count() {
        let buffer = TextBuffer::from("Line 1\nLine 2\nLine 3");
        assert_eq!(buffer.line_count(), 3);
    }

    #[test]
    fn test_line_col_conversion() {
        let buffer = TextBuffer::from("Hello\nWorld\n!");

        // Line 0, Col 0 -> Byte 0
        assert_eq!(buffer.byte_index_from_line_col(0, 0), 0);

        // Line 1, Col 0 -> Byte 6 (after "Hello\n")
        assert_eq!(buffer.byte_index_from_line_col(1, 0), 6);

        // Byte 0 -> Line 0, Col 0
        assert_eq!(buffer.line_col_from_byte_index(0), (0, 0));

        // Byte 6 -> Line 1, Col 0
        assert_eq!(buffer.line_col_from_byte_index(6), (1, 0));
    }
}
