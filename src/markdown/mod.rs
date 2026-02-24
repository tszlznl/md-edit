use crate::theme::Theme;
use pulldown_cmark::{Event, Parser, Tag, TagEnd};

/// Renders Markdown to rich text for display
pub struct MarkdownRenderer {
    theme: Theme,
}

/// A rendered element in the preview
#[derive(Clone, Debug)]
pub enum RenderedElement {
    Heading(u8, String),
    Paragraph(String),
    CodeBlock(String, String),
    InlineCode(String),
    BlockQuote(Vec<RenderedElement>),
    UnorderedList(Vec<Vec<RenderedElement>>),
    OrderedList(Vec<Vec<RenderedElement>>),
    HorizontalRule,
    Link(String, String),
    Image(String, String),
    RawHtml(String),
    LineBreak,
    Strong(String),
    Emphasis(String),
    Strikethrough(String),
}

impl MarkdownRenderer {
    pub fn new(theme: &Theme) -> Self {
        Self { theme: theme.clone() }
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    pub fn render(&self, markdown: &str) -> Vec<RenderedElement> {
        let parser = Parser::new(markdown);
        let mut elements = Vec::new();
        let mut current_element: Option<RenderedElement> = None;
        let mut list_stack: Vec<(bool, Vec<Vec<RenderedElement>>)> = Vec::new();
        let mut blockquote_stack: Vec<Vec<RenderedElement>> = Vec::new();

        for event in parser {
            match event {
                Event::Start(tag) => {
                    match tag {
                        Tag::List(start_num) => {
                            if let Some(elem) = current_element.take() {
                                elements.push(elem);
                            }
                            list_stack.push((start_num.is_some(), Vec::new()));
                        }
                        Tag::Item => {
                            if let Some(last) = list_stack.last_mut() {
                                last.1.push(Vec::new());
                            }
                        }
                        Tag::BlockQuote(_) => {
                            if let Some(elem) = current_element.take() {
                                elements.push(elem);
                            }
                            blockquote_stack.push(Vec::new());
                        }
                        Tag::CodeBlock(kind) => {
                            if let Some(elem) = current_element.take() {
                                elements.push(elem);
                            }
                            let lang = match kind {
                                pulldown_cmark::CodeBlockKind::Fenced(lang) => lang.to_string(),
                                _ => String::new(),
                            };
                            current_element = Some(RenderedElement::CodeBlock(lang, String::new()));
                        }
                        _ => {}
                    }
                }
                Event::End(tag) => {
                    match tag {
                        TagEnd::List(_) => {
                            if let Some((is_ordered, items)) = list_stack.pop() {
                                if let Some(elem) = current_element.take() {
                                    elements.push(elem);
                                }
                                if is_ordered {
                                    elements.push(RenderedElement::OrderedList(items));
                                } else {
                                    elements.push(RenderedElement::UnorderedList(items));
                                }
                            }
                        }
                        TagEnd::BlockQuote(_) => {
                            if let Some(items) = blockquote_stack.pop() {
                                if let Some(elem) = current_element.take() {
                                    elements.push(elem);
                                }
                                elements.push(RenderedElement::BlockQuote(items));
                            }
                        }
                        TagEnd::CodeBlock => {
                            if let Some(elem) = current_element.take() {
                                elements.push(elem);
                            }
                        }
                        _ => {}
                    }
                }
                Event::Text(text) => {
                    if let Some(ref mut elem) = current_element {
                        match elem {
                            RenderedElement::CodeBlock(_, ref mut code) => {
                                code.push_str(&text);
                            }
                            RenderedElement::Paragraph(ref mut p) => {
                                p.push_str(&text);
                            }
                            _ => {}
                        }
                    } else if let Some(last) = list_stack.last_mut() {
                        if let Some(item) = last.1.last_mut() {
                            item.push(RenderedElement::Paragraph(text.to_string()));
                        }
                    } else if let Some(items) = blockquote_stack.last_mut() {
                        items.push(RenderedElement::Paragraph(text.to_string()));
                    } else {
                        elements.push(RenderedElement::Paragraph(text.to_string()));
                    }
                }
                Event::Code(code) => {
                    if let Some(last) = list_stack.last_mut() {
                        if let Some(item) = last.1.last_mut() {
                            item.push(RenderedElement::InlineCode(code.to_string()));
                        }
                    } else {
                        elements.push(RenderedElement::InlineCode(code.to_string()));
                    }
                }
                Event::Html(html) => {
                    elements.push(RenderedElement::RawHtml(html.to_string()));
                }
                Event::SoftBreak => {
                    // Handle soft breaks if needed
                }
                Event::HardBreak => {
                    if let Some(ref mut elem) = current_element {
                        match elem {
                            RenderedElement::Paragraph(ref mut p) => {
                                p.push('\n');
                            }
                            _ => {}
                        }
                    } else {
                        elements.push(RenderedElement::LineBreak);
                    }
                }
                Event::Rule => {
                    elements.push(RenderedElement::HorizontalRule);
                }
                _ => {}
            }
        }

        // Add any remaining element
        if let Some(elem) = current_element {
            elements.push(elem);
        }

        elements
    }
}

impl Default for MarkdownRenderer {
    fn default() -> Self {
        Self::new(&Theme::default())
    }
}
