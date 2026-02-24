use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::ui::layouts::{LayoutMode, SplitDirection};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Theme mode (light, dark, or system)
    #[serde(default)]
    pub theme_mode: ThemeMode,

    /// Editor layout mode
    #[serde(default)]
    pub layout_mode: LayoutMode,

    /// Split direction for split layouts
    #[serde(default)]
    pub split_direction: SplitDirection,

    /// Font settings
    #[serde(default)]
    pub font: FontConfig,

    /// Editor behavior settings
    #[serde(default)]
    pub editor: EditorConfig,

    /// Auto-save settings
    #[serde(default)]
    pub auto_save: bool,

    /// Window state
    #[serde(default)]
    pub window: WindowConfig,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontConfig {
    #[serde(default = "default_editor_font")]
    pub editor_font: String,
    #[serde(default = "default_editor_font_size")]
    pub editor_font_size: f32,
    #[serde(default = "default_ui_font")]
    pub ui_font: String,
    #[serde(default = "default_ui_font_size")]
    pub ui_font_size: f32,
    #[serde(default)]
    pub line_height: f32,
}

fn default_editor_font() -> String {
    "Fira Code".to_string()
}

fn default_editor_font_size() -> f32 {
    14.0
}

fn default_ui_font() -> String {
    "Segoe UI".to_string()
}

fn default_ui_font_size() -> f32 {
    14.0
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            editor_font: default_editor_font(),
            editor_font_size: default_editor_font_size(),
            ui_font: default_ui_font(),
            ui_font_size: default_ui_font_size(),
            line_height: 1.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    #[serde(default = "default_true")]
    pub word_wrap: bool,
    #[serde(default = "default_true")]
    pub show_line_numbers: bool,
    #[serde(default = "default_true")]
    pub highlight_current_line: bool,
    #[serde(default = "default_true")]
    pub auto_indent: bool,
    #[serde(default = "default_true")]
    pub use_spaces_for_tabs: bool,
    #[serde(default)]
    pub tab_size: usize,
    #[serde(default = "default_auto_save_interval")]
    pub auto_save_interval_seconds: u64,
}

fn default_true() -> bool {
    true
}

fn default_auto_save_interval() -> u64 {
    30
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            word_wrap: true,
            show_line_numbers: true,
            highlight_current_line: true,
            auto_indent: true,
            use_spaces_for_tabs: true,
            tab_size: 4,
            auto_save_interval_seconds: 30,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    #[serde(default)]
    pub width: f32,
    #[serde(default)]
    pub height: f32,
    #[serde(default)]
    pub maximized: bool,
    #[serde(default)]
    pub sidebar_width: f32,
    #[serde(default)]
    pub editor_ratio: f32,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 1400.0,
            height: 900.0,
            maximized: false,
            sidebar_width: 250.0,
            editor_ratio: 0.5,
        }
    }
}

impl Config {
    /// Load configuration from the default config file
    pub fn load() -> anyhow::Result<Self> {
        let config_path = Self::config_path()?;
        let contents = std::fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Load configuration or return default if not found
    pub fn load_or_default() -> Self {
        Self::load().unwrap_or_default()
    }

    /// Save configuration to the default config file
    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = Self::config_path()?;

        // Ensure config directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let contents = toml::to_string_pretty(self)?;
        std::fs::write(config_path, contents)?;
        Ok(())
    }

    /// Get the path to the config file
    fn config_path() -> anyhow::Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        Ok(config_dir.join("rmd").join("config.toml"))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::default(),
            layout_mode: LayoutMode::default(),
            split_direction: SplitDirection::default(),
            font: FontConfig::default(),
            editor: EditorConfig::default(),
            auto_save: false,
            window: WindowConfig::default(),
        }
    }
}
