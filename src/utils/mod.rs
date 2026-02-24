//! Utility functions for RMD

use std::path::Path;

/// Check if a file is a supported Markdown file
pub fn is_markdown_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext = ext.to_string_lossy().to_lowercase();
        matches!(ext.as_str(), "md" | "markdown" | "mdown" | "mkd" | "mkdn" | "mdwn")
    } else {
        false
    }
}

/// Check if a file is a text file
pub fn is_text_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext = ext.to_string_lossy().to_lowercase();
        matches!(ext.as_str(), "txt" | "md" | "markdown" | "rst" | "text")
    } else {
        // Files without extension might be text files
        true
    }
}

/// Format a file size in human readable format
pub fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];

    if size == 0 {
        return "0 B".to_string();
    }

    let exp = (size as f64).log(1024.0).min(UNITS.len() as f64 - 1.0) as usize;
    let size = size as f64 / 1024f64.powi(exp as i32);

    if exp == 0 {
        format!("{} {}", size as u64, UNITS[exp])
    } else {
        format!("{:.2} {}", size, UNITS[exp])
    }
}

/// Format a duration in human readable format
pub fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();

    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    }
}

/// Truncate text with ellipsis if it exceeds max length
pub fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len.saturating_sub(3)])
    }
}

/// Normalize line endings to LF
pub fn normalize_line_endings(text: &str) -> String {
    text.replace("\r\n", "\n").replace('\r', "\n")
}

/// Count words in text
pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Count lines in text
pub fn count_lines(text: &str) -> usize {
    text.lines().count()
}

/// Estimate reading time in minutes
pub fn estimate_reading_time(word_count: usize) -> usize {
    // Average reading speed: 200 words per minute
    (word_count / 200).max(1)
}

/// Get file extension from path
pub fn get_extension(path: &Path) -> Option<String> {
    path.extension()
        .map(|e| e.to_string_lossy().to_lowercase())
}

/// Check if file is hidden (starts with .)
pub fn is_hidden_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

/// Sanitize filename for safe file system usage
pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(512), "512 B");
        assert_eq!(format_file_size(1024), "1.00 KB");
        assert_eq!(format_file_size(1536), "1.50 KB");
        assert_eq!(format_file_size(1024 * 1024), "1.00 MB");
    }

    #[test]
    fn test_normalize_line_endings() {
        assert_eq!(normalize_line_endings("a\r\nb"), "a\nb");
        assert_eq!(normalize_line_endings("a\rb"), "a\nb");
        assert_eq!(normalize_line_endings("a\nb"), "a\nb");
    }

    #[test]
    fn test_truncate_text() {
        assert_eq!(truncate_text("hello", 10), "hello");
        assert_eq!(truncate_text("hello world", 8), "hello...");
    }

    #[test]
    fn test_count_words() {
        assert_eq!(count_words("hello world"), 2);
        assert_eq!(count_words("  hello   world  "), 2);
        assert_eq!(count_words(""), 0);
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("hello/world"), "hello_world");
        assert_eq!(sanitize_filename("hello:world"), "hello_world");
        assert_eq!(sanitize_filename("hello<world>"), "hello_world_");
    }
}
