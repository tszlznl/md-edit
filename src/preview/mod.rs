use crate::markdown::{MarkdownRenderer, RenderedElement};
use crate::theme::Theme;

/// Preview panel for rendered Markdown
pub struct Preview {
    pub elements: Vec<RenderedElement>,
    pub scroll_offset: f32,
    pub follow_editor: bool,
    pub zoom: f32,
    pub show_toc: bool,
}

impl Preview {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            scroll_offset: 0.0,
            follow_editor: true,
            zoom: 1.0,
            show_toc: false,
        }
    }

    pub fn update_content(&mut self, elements: Vec<RenderedElement>) {
        self.elements = elements;
    }

    pub fn set_scroll_offset(&mut self, offset: f32) {
        self.scroll_offset = offset;
    }

    pub fn toggle_follow_editor(&mut self) {
        self.follow_editor = !self.follow_editor;
    }

    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom * 1.1).min(3.0);
    }

    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom / 1.1).max(0.5);
    }

    pub fn reset_zoom(&mut self) {
        self.zoom = 1.0;
    }

    pub fn toggle_toc(&mut self) {
        self.show_toc = !self.show_toc;
    }

    pub fn generate_toc(&self) -> Vec<(u8, String)> {
        let mut toc = Vec::new();
        for element in &self.elements {
            if let RenderedElement::Heading(level, text) = element {
                toc.push((*level, text.clone()));
            }
        }
        toc
    }

    pub fn word_count(&self) -> usize {
        let mut count = 0;
        for element in &self.elements {
            count += Self::count_words_in_element(element);
        }
        count
    }

    fn count_words_in_element(element: &RenderedElement) -> usize {
        match element {
            RenderedElement::Paragraph(text) |
            RenderedElement::Heading(_, text) => text.split_whitespace().count(),
            RenderedElement::BlockQuote(items) => {
                items.iter().map(|item| {
                    Self::count_words_in_element(item)
                }).sum()
            }
            RenderedElement::UnorderedList(items) |
            RenderedElement::OrderedList(items) => {
                items.iter().map(|item| {
                    item.iter().map(Self::count_words_in_element).sum::<usize>()
                }).sum()
            }
            _ => 0,
        }
    }
}

impl Default for Preview {
    fn default() -> Self {
        Self::new()
    }
}
