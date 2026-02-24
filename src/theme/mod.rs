use eframe::egui;
use crate::config::ThemeMode;

/// Application theme
#[derive(Clone, Debug)]
pub struct Theme {
    /// Primary accent color
    pub accent: egui::Color32,
    /// Background color
    pub background: egui::Color32,
    /// Surface color (panels, cards)
    pub surface: egui::Color32,
    /// Text color
    pub text: egui::Color32,
    /// Muted text color
    pub text_muted: egui::Color32,
    /// Border color
    pub border: egui::Color32,
    /// Selection color
    pub selection: egui::Color32,
    /// Code background
    pub code_bg: egui::Color32,
    /// Link color
    pub link: egui::Color32,
    /// Error color
    pub error: egui::Color32,
    /// Warning color
    pub warning: egui::Color32,
    /// Success color
    pub success: egui::Color32,
}

impl Theme {
    /// Create a dark theme
    pub fn dark() -> Self {
        Self {
            accent: egui::Color32::from_rgb(66, 165, 245),
            background: egui::Color32::from_rgb(18, 18, 18),
            surface: egui::Color32::from_rgb(30, 30, 30),
            text: egui::Color32::from_rgb(255, 255, 255),
            text_muted: egui::Color32::from_rgb(158, 158, 158),
            border: egui::Color32::from_rgb(48, 48, 48),
            selection: egui::Color32::from_rgb(66, 165, 245),
            code_bg: egui::Color32::from_rgb(40, 40, 40),
            link: egui::Color32::from_rgb(66, 165, 245),
            error: egui::Color32::from_rgb(244, 67, 54),
            warning: egui::Color32::from_rgb(255, 152, 0),
            success: egui::Color32::from_rgb(76, 175, 80),
        }
    }

    /// Create a light theme
    pub fn light() -> Self {
        Self {
            accent: egui::Color32::from_rgb(25, 118, 210),
            background: egui::Color32::from_rgb(250, 250, 250),
            surface: egui::Color32::from_rgb(255, 255, 255),
            text: egui::Color32::from_rgb(33, 33, 33),
            text_muted: egui::Color32::from_rgb(117, 117, 117),
            border: egui::Color32::from_rgb(224, 224, 224),
            selection: egui::Color32::from_rgb(25, 118, 210),
            code_bg: egui::Color32::from_rgb(245, 245, 245),
            link: egui::Color32::from_rgb(25, 118, 210),
            error: egui::Color32::from_rgb(211, 47, 47),
            warning: egui::Color32::from_rgb(245, 124, 0),
            success: egui::Color32::from_rgb(56, 142, 60),
        }
    }

    /// Apply this theme to egui context
    pub fn apply(&self, ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();

        visuals.override_text_color = Some(self.text);
        visuals.widgets.inactive.bg_fill = self.surface;
        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, self.text);
        visuals.widgets.active.bg_fill = self.accent;
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, self.text);
        visuals.widgets.hovered.bg_fill = self.surface;
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, self.accent);
        visuals.selection.bg_fill = self.selection.linear_multiply(0.4);
        visuals.selection.stroke = egui::Stroke::new(1.0, self.selection);
        visuals.window_fill = self.background;
        visuals.panel_fill = self.surface;
        visuals.window_stroke = egui::Stroke::new(1.0, self.border);
        visuals.extreme_bg_color = self.code_bg;
        visuals.faint_bg_color = self.surface.linear_multiply(0.5);
        visuals.hyperlink_color = self.link;
        visuals.error_fg_color = self.error;
        visuals.warn_fg_color = self.warning;

        ctx.set_visuals(visuals);
    }

    /// Create a theme from ThemeMode
    pub fn from_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => Self::light(),
            ThemeMode::Dark => Self::dark(),
            ThemeMode::System => {
                // Check system preference
                // For now, default to dark
                Self::dark()
            }
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

/// Theme mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

impl Default for ThemeMode {
    fn default() -> Self {
        ThemeMode::System
    }
}
