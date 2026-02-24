use serde::{Deserialize, Serialize};

/// Defines the layout modes for the editor
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutMode {
    /// Only show the editor
    EditorOnly,
    /// Only show the preview
    PreviewOnly,
    /// Show both editor and preview side by side
    Split,
}

impl Default for LayoutMode {
    fn default() -> Self {
        LayoutMode::Split
    }
}

/// Direction for split layouts
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitDirection {
    /// Split horizontally (editor on left, preview on right)
    Horizontal,
    /// Split vertically (editor on top, preview on bottom)
    Vertical,
}

impl Default for SplitDirection {
    fn default() -> Self {
        SplitDirection::Horizontal
    }
}

/// Manages the editor layout
pub struct EditorLayout {
    /// Current layout mode
    pub mode: LayoutMode,
    /// Split direction (for split mode)
    pub split_direction: SplitDirection,
    /// Split ratio (0.0 - 1.0), represents the proportion for the editor
    pub split_ratio: f32,
    /// Minimum panel size
    pub min_panel_size: f32,
    /// Whether the split is being dragged
    pub is_dragging_split: bool,
}

impl EditorLayout {
    /// Create a new layout with the given mode and direction
    pub fn new(mode: LayoutMode, split_direction: SplitDirection) -> Self {
        Self {
            mode,
            split_direction,
            split_ratio: 0.5,
            min_panel_size: 200.0,
            is_dragging_split: false,
        }
    }

    /// Set the layout mode
    pub fn set_mode(&mut self, mode: LayoutMode) {
        self.mode = mode;
    }

    /// Set the split direction
    pub fn set_split_direction(&mut self, direction: SplitDirection) {
        self.split_direction = direction;
    }

    /// Set the split ratio
    pub fn set_split_ratio(&mut self, ratio: f32) {
        self.split_ratio = ratio.clamp(0.1, 0.9);
    }

    /// Toggle between editor-only and split modes
    pub fn toggle_editor_fullscreen(&mut self) {
        match self.mode {
            LayoutMode::EditorOnly => self.mode = LayoutMode::Split,
            _ => self.mode = LayoutMode::EditorOnly,
        }
    }

    /// Toggle between preview-only and split modes
    pub fn toggle_preview_fullscreen(&mut self) {
        match self.mode {
            LayoutMode::PreviewOnly => self.mode = LayoutMode::Split,
            _ => self.mode = LayoutMode::PreviewOnly,
        }
    }

    /// Swap the editor and preview positions in split mode
    pub fn swap_panels(&mut self) {
        self.split_ratio = 1.0 - self.split_ratio;
    }

    /// Get the editor panel size for the current layout
    pub fn get_editor_size(&self, total_size: f32) -> f32 {
        match self.mode {
            LayoutMode::EditorOnly => total_size,
            LayoutMode::PreviewOnly => 0.0,
            LayoutMode::Split => {
                let size = total_size * self.split_ratio;
                size.max(self.min_panel_size)
            }
        }
    }

    /// Get the preview panel size for the current layout
    pub fn get_preview_size(&self, total_size: f32) -> f32 {
        match self.mode {
            LayoutMode::EditorOnly => 0.0,
            LayoutMode::PreviewOnly => total_size,
            LayoutMode::Split => {
                let size = total_size * (1.0 - self.split_ratio);
                size.max(self.min_panel_size)
            }
        }
    }

    /// Check if the layout includes the editor
    pub fn has_editor(&self) -> bool {
        matches!(self.mode, LayoutMode::EditorOnly | LayoutMode::Split)
    }

    /// Check if the layout includes the preview
    pub fn has_preview(&self) -> bool {
        matches!(self.mode, LayoutMode::PreviewOnly | LayoutMode::Split)
    }
}

impl Default for EditorLayout {
    fn default() -> Self {
        Self::new(LayoutMode::default(), SplitDirection::default())
    }
}
