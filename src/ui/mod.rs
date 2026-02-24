pub mod layouts;
pub mod widgets;

use crate::app::RmdApp;
use crate::ui::layouts::{EditorLayout, LayoutMode};
use eframe::egui;
use std::path::Path;

/// UI components for RMD
impl RmdApp {
    /// Render the menu bar
    pub fn ui_menu_bar(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::menu::bar(ctx, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New (Ctrl+N)").clicked() {
                    self.new_file();
                    ui.close_menu();
                }
                if ui.button("Open (Ctrl+O)").clicked() {
                    self.open_file_dialog();
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Save (Ctrl+S)").clicked() {
                    self.save_file();
                    ui.close_menu();
                }
                if ui.button("Save As (Ctrl+Shift+S)").clicked() {
                    self.save_file_as();
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Exit (Alt+F4)").clicked() {
                    frame.close();
                    ui.close_menu();
                }
            });

            ui.menu_button("Edit", |ui| {
                let can_undo = self.editor.can_undo();
                let can_redo = self.editor.can_redo();

                if ui.add_enabled(can_undo, egui::Button::new("Undo (Ctrl+Z)")).clicked() {
                    self.editor.undo();
                    ui.close_menu();
                }
                if ui.add_enabled(can_redo, egui::Button::new("Redo (Ctrl+Y)")).clicked() {
                    self.editor.redo();
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Cut (Ctrl+X)").clicked() {
                    // self.cut();
                    ui.close_menu();
                }
                if ui.button("Copy (Ctrl+C)").clicked() {
                    // self.copy();
                    ui.close_menu();
                }
                if ui.button("Paste (Ctrl+V)").clicked() {
                    // self.paste();
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Find (Ctrl+F)").clicked() {
                    // self.open_find();
                    ui.close_menu();
                }
                if ui.button("Replace (Ctrl+H)").clicked() {
                    // self.open_replace();
                    ui.close_menu();
                }
            });

            ui.menu_button("View", |ui| {
                ui.menu_button("Layout", |ui| {
                    if ui.radio(self.layout.mode == LayoutMode::EditorOnly, "Editor Only").clicked() {
                        self.layout.set_mode(LayoutMode::EditorOnly);
                        ui.close_menu();
                    }
                    if ui.radio(self.layout.mode == LayoutMode::PreviewOnly, "Preview Only").clicked() {
                        self.layout.set_mode(LayoutMode::PreviewOnly);
                        ui.close_menu();
                    }
                    if ui.radio(self.layout.mode == LayoutMode::Split, "Split View").clicked() {
                        self.layout.set_mode(LayoutMode::Split);
                        ui.close_menu();
                    }
                });
                ui.separator();
                if ui.checkbox(&mut self.show_sidebar, "Show Sidebar").clicked() {
                    // Toggle handled by checkbox
                }
                if ui.checkbox(&mut self.show_toolbar, "Show Toolbar").clicked() {
                    // Toggle handled by checkbox
                }
                if ui.checkbox(&mut self.show_status_bar, "Show Status Bar").clicked() {
                    // Toggle handled by checkbox
                }
                ui.separator();
                if ui.button("Zoom In (Ctrl++)").clicked() {
                    // self.zoom_in();
                    ui.close_menu();
                }
                if ui.button("Zoom Out (Ctrl+-)").clicked() {
                    // self.zoom_out();
                    ui.close_menu();
                }
                if ui.button("Reset Zoom (Ctrl+0)").clicked() {
                    // self.reset_zoom();
                    ui.close_menu();
                }
            });

            ui.menu_button("Help", |ui| {
                if ui.button("Documentation").clicked() {
                    // Open documentation
                    ui.close_menu();
                }
                if ui.button("Keyboard Shortcuts").clicked() {
                    // Show shortcuts
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("About").clicked() {
                    // Show about dialog
                    ui.close_menu();
                }
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if self.has_unsaved_changes {
                    ui.label(egui::RichText::new("●").color(egui::Color32::from_rgb(255, 193, 7)));
                }
            });
        });
    }

    /// Render the toolbar
    pub fn ui_toolbar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("toolbar")
            .exact_height(40.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // File operations
                    if ui.button("New").clicked() {
                        self.new_file();
                    }
                    if ui.button("Open").clicked() {
                        self.open_file_dialog();
                    }
                    if ui.button("Save").clicked() {
                        self.save_file();
                    }

                    ui.separator();

                    // Edit operations
                    let can_undo = self.editor.can_undo();
                    let can_redo = self.editor.can_redo();

                    if ui.add_enabled(can_undo, egui::Button::new("Undo")).clicked() {
                        self.editor.undo();
                    }
                    if ui.add_enabled(can_redo, egui::Button::new("Redo")).clicked() {
                        self.editor.redo();
                    }

                    ui.separator();

                    // View modes
                    ui.label("View:");
                    if ui.selectable_label(self.layout.mode == LayoutMode::EditorOnly, "Editor").clicked() {
                        self.layout.set_mode(LayoutMode::EditorOnly);
                    }
                    if ui.selectable_label(self.layout.mode == LayoutMode::Split, "Split").clicked() {
                        self.layout.set_mode(LayoutMode::Split);
                    }
                    if ui.selectable_label(self.layout.mode == LayoutMode::PreviewOnly, "Preview").clicked() {
                        self.layout.set_mode(LayoutMode::PreviewOnly);
                    }

                    ui.separator();

                    // Theme toggle
                    if ui.button("Theme").clicked() {
                        // Toggle theme
                    }
                });
            });
    }

    /// Render the status bar
    pub fn ui_status_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status_bar")
            .exact_height(24.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // File info
                    if let Some(ref path) = self.current_file {
                        let file_name = path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("Untitled");
                        ui.label(file_name);
                    } else {
                        ui.label("Untitled");
                    }

                    if self.has_unsaved_changes {
                        ui.label(egui::RichText::new("(modified)").color(ui.visuals().warn_fg_color));
                    }

                    ui.separator();

                    // Cursor position
                    let (line, col) = self.editor.cursor_position();
                    ui.label(format!("Ln {}, Col {}", line + 1, col + 1));

                    ui.separator();

                    // Document statistics
                    let text = self.editor.text();
                    let char_count = text.chars().count();
                    let word_count = text.split_whitespace().count();
                    ui.label(format!("{} words, {} chars", word_count, char_count));

                    // Right-aligned info
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("Markdown");
                    });
                });
            });
    }

    /// Render the main content area
    pub fn ui_main_content(&mut self, ui: &mut egui::Ui) {
        let layout_mode = self.layout.mode;

        match layout_mode {
            LayoutMode::EditorOnly => {
                self.render_editor(ui);
            }
            LayoutMode::PreviewOnly => {
                self.render_preview(ui);
            }
            LayoutMode::Split => {
                self.render_split_view(ui);
            }
        }
    }

    /// Render the editor panel
    fn render_editor(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(ui.style()).inner_margin(0.0))
            .show_inside(ui, |ui| {
                let available_size = ui.available_size();

                egui::ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        let text = self.editor.text();

                        // Create a text edit for the editor
                        let mut text_clone = text.clone();
                        let text_edit = egui::TextEdit::multiline(&mut text_clone)
                            .font(egui::TextStyle::Monospace)
                            .code_editor()
                            .desired_width(available_size.x)
                            .desired_rows(100);

                        let response = ui.add(text_edit);
                        if response.changed() {
                            // Update editor content
                            self.editor.set_text(text_clone);
                            self.has_unsaved_changes = true;
                        }
                    });
            });
    }

    /// Render the preview panel
    fn render_preview(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(ui.style()).inner_margin(16.0))
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        let text = self.editor.text();
                        let elements = self.markdown_renderer.render(&text);

                        for element in elements {
                            self.render_element(ui, &element);
                        }
                    });
            });
    }

    /// Render the split view with editor and preview side by side
    fn render_split_view(&mut self, ui: &mut egui::Ui) {
        let split_ratio = self.config.window.editor_ratio;

        // Use a splitter to divide the space
        egui::SidePanel::left("editor_panel")
            .resizable(true)
            .default_width(ui.available_width() * split_ratio)
            .show_inside(ui, |ui| {
                self.render_editor(ui);
            });

        // Preview panel takes remaining space
        self.render_preview(ui);
    }

    /// Render a single element
    fn render_element(&self, ui: &mut egui::Ui, element: &crate::markdown::RenderedElement) {
        use crate::markdown::RenderedElement::*;

        match element {
            Heading(level, text) => {
                let text_size = match level {
                    1 => 32.0,
                    2 => 28.0,
                    3 => 24.0,
                    4 => 20.0,
                    5 => 18.0,
                    _ => 16.0,
                };
                ui.add_space(16.0);
                ui.label(
                    egui::RichText::new(text)
                        .size(text_size)
                        .strong()
                        .color(self.theme.text),
                );
                ui.add_space(8.0);
            }
            Paragraph(text) => {
                ui.label(
                    egui::RichText::new(text)
                        .size(16.0)
                        .color(self.theme.text),
                );
                ui.add_space(12.0);
            }
            CodeBlock(lang, code) => {
                ui.add_space(8.0);
                egui::Frame::none()
                    .fill(self.theme.code_bg)
                    .corner_radius(6.0)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        if !lang.is_empty() {
                            ui.label(
                                egui::RichText::new(lang)
                                    .size(12.0)
                                    .color(self.theme.text_muted)
                                    .monospace(),
                            );
                            ui.add_space(4.0);
                        }
                        ui.label(
                            egui::RichText::new(code)
                                .monospace()
                                .size(14.0)
                                .color(self.theme.text),
                        );
                    });
                ui.add_space(8.0);
            }
            InlineCode(code) => {
                ui.colored_label(
                    self.theme.code_bg,
                    egui::RichText::new(code).monospace().size(14.0),
                );
            }
            BlockQuote(items) => {
                ui.add_space(8.0);
                egui::Frame::none()
                    .fill(self.theme.surface)
                    .inner_margin(12.0)
                    .corner_radius(4.0)
                    .show(ui, |ui| {
                        for item in items {
                            self.render_element(ui, item);
                        }
                    });
                ui.add_space(8.0);
            }
            UnorderedList(items) => {
                ui.add_space(4.0);
                for item in items {
                    ui.horizontal(|ui| {
                        ui.label("•");
                        ui.vertical(|ui| {
                            for elem in item {
                                self.render_element(ui, elem);
                            }
                        });
                    });
                }
                ui.add_space(4.0);
            }
            OrderedList(items) => {
                ui.add_space(4.0);
                for (i, item) in items.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}.", i + 1));
                        ui.vertical(|ui| {
                            for elem in item {
                                self.render_element(ui, elem);
                            }
                        });
                    });
                }
                ui.add_space(4.0);
            }
            HorizontalRule => {
                ui.add_space(8.0);
                ui.add(egui::Separator::default().horizontal());
                ui.add_space(8.0);
            }
            Link(text, url) => {
                if ui.link(text).clicked() {
                    if let Err(e) = webbrowser::open(url) {
                        eprintln!("Failed to open link: {}", e);
                    }
                }
            }
            Image(alt, url) => {
                // For now, just show a placeholder for images
                ui.add_space(8.0);
                egui::Frame::none()
                    .fill(self.theme.surface)
                    .corner_radius(6.0)
                    .inner_margin(16.0)
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(egui::RichText::new("🖼").size(48.0));
                            ui.add_space(4.0);
                            ui.label(egui::RichText::new(alt).size(12.0).color(self.theme.text_muted));
                            ui.label(egui::RichText::new(url).size(10.0).color(self.theme.text_muted).monospace());
                        });
                    });
                ui.add_space(8.0);
            }
            RawHtml(html) => {
                // Show HTML as code block for now
                ui.add_space(4.0);
                egui::Frame::none()
                    .fill(self.theme.code_bg)
                    .corner_radius(4.0)
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(html)
                                .monospace()
                                .size(12.0)
                                .color(self.theme.text_muted),
                        );
                    });
                ui.add_space(4.0);
            }
            LineBreak => {
                ui.add_space(8.0);
            }
            Strong(text) => {
                ui.label(
                    egui::RichText::new(text)
                        .strong()
                        .color(self.theme.text),
                );
            }
            Emphasis(text) => {
                ui.label(
                    egui::RichText::new(text)
                        .italics()
                        .color(self.theme.text),
                );
            }
            Strikethrough(text) => {
                ui.label(
                    egui::RichText::new(text)
                        .strikethrough()
                        .color(self.theme.text_muted),
                );
            }
        }
    }
}

// Stub implementations for actions
impl RmdApp {
    fn new_file(&mut self) {
        if self.has_unsaved_changes {
            // Show save dialog
        }
        self.editor.set_text("");
        self.current_file = None;
        self.has_unsaved_changes = false;
    }

    fn open_file_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Markdown", &["md", "markdown", "mdown", "mkd"])
            .add_filter("Text", &["txt"])
            .add_filter("All files", &["*"])
            .pick_file()
        {
            if let Err(e) = self.editor.open_file(&path) {
                eprintln!("Failed to open file: {}", e);
            } else {
                self.current_file = Some(path);
                self.has_unsaved_changes = false;
            }
        }
    }

    fn save_file(&mut self) {
        if let Some(ref path) = self.current_file {
            if let Err(e) = self.editor.save_file(path) {
                eprintln!("Failed to save file: {}", e);
            } else {
                self.has_unsaved_changes = false;
            }
        } else {
            self.save_file_as();
        }
    }

    fn save_file_as(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Markdown", &["md"])
            .set_file_name("untitled.md")
            .save_file()
        {
            if let Err(e) = self.editor.save_file(&path) {
                eprintln!("Failed to save file: {}", e);
            } else {
                self.current_file = Some(path);
                self.has_unsaved_changes = false;
            }
        }
    }

    fn check_auto_save(&mut self) {
        // Implement auto-save logic
    }
}
