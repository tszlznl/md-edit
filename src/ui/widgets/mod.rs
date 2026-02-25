//! Custom UI widgets for RMD

use eframe::egui;

/// A split panel widget that divides space between two children
pub struct SplitPanel {
    direction: SplitDirection,
    split_ratio: f32,
    min_size: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

impl SplitPanel {
    pub fn new(direction: SplitDirection) -> Self {
        Self {
            direction,
            split_ratio: 0.5,
            min_size: 100.0,
        }
    }

    pub fn split_ratio(mut self, ratio: f32) -> Self {
        self.split_ratio = ratio.clamp(0.1, 0.9);
        self
    }

    pub fn min_size(mut self, size: f32) -> Self {
        self.min_size = size;
        self
    }

    pub fn show(&mut self,
        ui: &mut egui::Ui,
        first: impl FnOnce(&mut egui::Ui),
        second: impl FnOnce(&mut egui::Ui),
    ) {
        let available_size = ui.available_size();

        match self.direction {
            SplitDirection::Horizontal => {
                let first_width = (available_size.x * self.split_ratio)
                    .max(self.min_size)
                    .min(available_size.x - self.min_size);

                ui.horizontal(|ui| {
                    ui.allocate_ui_with_layout(
                        egui::vec2(first_width, available_size.y),
                        egui::Layout::top_down(egui::Align::Min),
                        first,
                    );

                    ui.add(egui::Separator::default().vertical());

                    ui.allocate_ui_with_layout(
                        egui::vec2(available_size.x - first_width - 10.0, available_size.y),
                        egui::Layout::top_down(egui::Align::Min),
                        second,
                    );
                });
            }
            SplitDirection::Vertical => {
                let first_height = (available_size.y * self.split_ratio)
                    .max(self.min_size)
                    .min(available_size.y - self.min_size);

                ui.vertical(|ui| {
                    ui.allocate_ui_with_layout(
                        egui::vec2(available_size.x, first_height),
                        egui::Layout::top_down(egui::Align::Min),
                        first,
                    );

                    ui.add(egui::Separator::default().horizontal());

                    ui.allocate_ui_with_layout(
                        egui::vec2(available_size.x, available_size.y - first_height - 10.0),
                        egui::Layout::top_down(egui::Align::Min),
                        second,
                    );
                });
            }
        }
    }
}

/// A toolbar button with icon and tooltip
pub struct ToolbarButton {
    icon: &'static str,
    tooltip: Option<&'static str>,
    enabled: bool,
}

impl ToolbarButton {
    pub fn new(icon: &'static str) -> Self {
        Self {
            icon,
            tooltip: None,
            enabled: true,
        }
    }

    pub fn tooltip(mut self, tooltip: &'static str) -> Self {
        self.tooltip = Some(tooltip);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let mut button = egui::Button::new(self.icon);
        if !self.enabled {
            button = button.sense(egui::Sense::hover());
        }

        let response = ui.add_sized(egui::vec2(32.0, 32.0), button);

        if let Some(tooltip) = self.tooltip {
            return response.on_hover_text(tooltip);
        }

        response
    }
}

/// A status indicator widget
pub struct StatusIndicator {
    text: String,
    color: egui::Color32,
}

impl StatusIndicator {
    pub fn new(text: impl Into<String>, color: egui::Color32) -> Self {
        Self {
            text: text.into(),
            color,
        }
    }

    pub fn show(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Status dot
            let (rect, _) = ui.allocate_exact_size(egui::vec2(8.0, 8.0), egui::Sense::hover());
            ui.painter().circle_filled(
                rect.center(),
                4.0,
                self.color,
            );
            ui.add_space(4.0);
            ui.label(&self.text);
        });
    }
}

/// A scrollable code block with syntax highlighting
pub struct CodeBlock {
    code: String,
    language: Option<String>,
}

impl CodeBlock {
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            language: None,
        }
    }

    pub fn language(mut self, lang: impl Into<String>) -> Self {
        self.language = Some(lang.into());
        self
    }

    pub fn show(&self, ui: &mut egui::Ui, theme: &crate::theme::Theme) {
        egui::Frame::none()
            .fill(theme.code_bg)
            .rounding(6.0)
            .inner_margin(12.0)
            .show(ui, |ui2| {
                // Language label
                if let Some(lang) = &self.language {
                    ui2.label(
                        egui::RichText::new(lang)
                            .size(12.0)
                            .color(theme.text_muted)
                            .monospace(),
                    );
                    ui2.add_space(4.0);
                }

                // Code content with scroll area if needed
                egui::ScrollArea::horizontal()
                    .auto_shrink([false, true])
                    .show(ui2, |ui3| {
                        ui3.label(
                            egui::RichText::new(&self.code)
                                .monospace()
                                .size(14.0)
                                .color(theme.text),
                        );
                    });
            });
    }
}
