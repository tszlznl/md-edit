use crate::{
    config::Config,
    editor::Editor,
    markdown::MarkdownRenderer,
    preview::Preview,
    theme::Theme,
    ui::layouts::{EditorLayout, LayoutMode, SplitDirection},
};
use eframe::egui;
use std::path::PathBuf;

pub struct RmdApp {
    // Core components
    pub editor: Editor,
    pub preview: Preview,
    pub markdown_renderer: MarkdownRenderer,

    // UI state
    pub layout: EditorLayout,
    pub theme: Theme,
    pub config: Config,

    // File state
    pub current_file: Option<PathBuf>,
    pub has_unsaved_changes: bool,

    // Panel visibility
    pub show_sidebar: bool,
    pub show_toolbar: bool,
    pub show_status_bar: bool,
}

impl RmdApp {
    pub fn new(cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        // Initialize theme
        let theme = Theme::from_mode(config.theme_mode);

        // Apply theme to egui context
        theme.apply(&cc.egui_ctx);

        // Load custom fonts if available
        Self::load_custom_fonts(&cc.egui_ctx);

        let markdown_renderer = MarkdownRenderer::new(&theme);
        let editor = Editor::new();
        let preview = Preview::new();

        let layout = EditorLayout::new(
            config.layout_mode,
            config.split_direction,
        );

        Self {
            editor,
            preview,
            markdown_renderer,
            layout,
            theme,
            config,
            current_file: None,
            has_unsaved_changes: false,
            show_sidebar: true,
            show_toolbar: true,
            show_status_bar: true,
        }
    }

    fn load_custom_fonts(ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();

        // Add Fira Code for monospace (code blocks)
        if let Ok(font_data) = std::fs::read("assets/fonts/FiraCode-Regular.ttf") {
            fonts.font_data.insert(
                "FiraCode".to_owned(),
                egui::FontData::from_owned(font_data),
            );
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("FiraCode".to_owned());
        }

        ctx.set_fonts(fonts);
    }
}

impl eframe::App for RmdApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Handle file drops
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                if let Some(file) = i.raw.dropped_files.first() {
                    if let Some(ref path) = file.path {
                        self.editor.open_file(path);
                        self.current_file = Some(path.clone());
                        self.has_unsaved_changes = false;
                    }
                }
            }
        });

        // Top menu bar
        self.ui_menu_bar(ctx, frame);

        // Toolbar
        if self.show_toolbar {
            self.ui_toolbar(ctx);
        }

        // Main content area with sidebar and editor/preview
        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(0.0))
            .show(ctx, |ui| {
                self.ui_main_content(ui);
            });

        // Status bar
        if self.show_status_bar {
            self.ui_status_bar(ctx);
        }

        // Auto-save check (stub)
        if self.config.auto_save && self.has_unsaved_changes {
            // TODO: implement auto-save
        }

        // Request continuous updates for smooth preview
        ctx.request_repaint_after(std::time::Duration::from_millis(16));
    }

    fn on_exit(&mut self, _ctx: Option<&eframe::glow::Context>) {
        // Save config on exit
        if let Err(e) = self.config.save() {
            eprintln!("Failed to save config: {}", e);
        }
    }
}
