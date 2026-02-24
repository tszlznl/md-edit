use crate::theme::Theme;
use eframe::egui;

/// Syntax highlighter for Markdown
pub struct MarkdownHighlighter {
    theme: Theme,
}

/// A highlighted token
#[derive(Clone, Debug)]
pub struct Token {
    pub text: String,
    pub style: TokenStyle,
}

/// Style for a token
#[derive(Clone, Debug, Default)]
pub struct TokenStyle {
    pub color: Option<egui::Color32>,
    pub background: Option<egui::Color32>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub code: bool,
}

/// A line of tokens
#[derive(Clone, Debug)]
pub struct Line {
    pub tokens: Vec<Token>,
}

impl MarkdownHighlighter {
    /// Create a new highlighter with the given theme
    pub fn new(theme: Theme) -> Self {
        Self { theme }
    }

    /// Update the theme
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    /// Highlight a line of text
    pub fn highlight_line(&self, line: &str) -> Line {
        let mut tokens = Vec::new();
        let mut chars = line.chars().peekable();
        let mut current_text = String::new();
        let mut in_code_span = false;
        let mut in_bold = false;
        let mut in_italic = false;
        let mut in_strikethrough = false;

        while let Some(c) = chars.next() {
            match c {
                '`' => {
                    if !current_text.is_empty() {
                        tokens.push(Token {
                            text: current_text.clone(),
                            style: self.create_style(in_bold, in_italic, in_strikethrough, in_code_span),
                        });
                        current_text.clear();
                    }
                    in_code_span = !in_code_span;
                }
                '*' | '_' => {
                    let next_is_same = chars.peek() == Some(&c);

                    if next_is_same && !in_code_span {
                        // Bold
                        chars.next(); // Consume second char
                        if !current_text.is_empty() {
                            tokens.push(Token {
                                text: current_text.clone(),
                                style: self.create_style(in_bold, in_italic, in_strikethrough, in_code_span),
                            });
                            current_text.clear();
                        }
                        in_bold = !in_bold;
                    } else if !in_code_span {
                        // Italic
                        if !current_text.is_empty() {
                            tokens.push(Token {
                                text: current_text.clone(),
                                style: self.create_style(in_bold, in_italic, in_strikethrough, in_code_span),
                            });
                            current_text.clear();
                        }
                        in_italic = !in_italic;
                    } else {
                        current_text.push(c);
                    }
                }
                '~' => {
                    let next_is_same = chars.peek() == Some(&'~');
                    if next_is_same && !in_code_span {
                        chars.next();
                        if !current_text.is_empty() {
                            tokens.push(Token {
                                text: current_text.clone(),
                                style: self.create_style(in_bold, in_italic, in_strikethrough, in_code_span),
                            });
                            current_text.clear();
                        }
                        in_strikethrough = !in_strikethrough;
                    } else {
                        current_text.push(c);
                    }
                }
                _ => {
                    current_text.push(c);
                }
            }
        }

        // Add remaining text
        if !current_text.is_empty() {
            tokens.push(Token {
                text: current_text,
                style: self.create_style(in_bold, in_italic, in_strikethrough, in_code_span),
            });
        }

        // If no tokens were created, create an empty one
        if tokens.is_empty() {
            tokens.push(Token {
                text: String::new(),
                style: TokenStyle::default(),
            });
        }

        Line { tokens }
    }

    fn create_style(&self, bold: bool, italic: bool, strikethrough: bool, code: bool) -> TokenStyle {
        TokenStyle {
            color: None,
            background: if code { Some(self.theme.code_bg) } else { None },
            bold,
            italic,
            underline: false,
            strikethrough,
            code,
        }
    }
}

impl Default for MarkdownHighlighter {
    fn default() -> Self {
        Self::new(Theme::default())
    }
}
