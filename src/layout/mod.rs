//! Layout management module
//! 
//! Implements all tiling layouts:
//! - Dynamic (Hyprland-style)
//! - Horizontal (niri-style)
//! - Vertical (MangoWM-style)
//! - Center (MangoWM-style)

use log::{info, debug};
use crate::config::Config;

mod dynamic;
mod horizontal;
mod vertical;
mod center;

pub use dynamic::DynamicLayout;
pub use horizontal::HorizontalLayout;
pub use vertical::VerticalLayout;
pub use center::CenterLayout;

/// Layout manager handles switching between different layout modes
pub struct LayoutManager {
    /// Current layout mode
    current_mode: LayoutMode,
    /// Dynamic layout engine
    dynamic: DynamicLayout,
    /// Horizontal layout engine
    horizontal: HorizontalLayout,
    /// Vertical layout engine
    vertical: VerticalLayout,
    /// Center layout engine
    center: CenterLayout,
}

/// Available layout modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutMode {
    /// Hyprland-style dynamic tiling
    Dynamic,
    /// niri-style horizontal tiling
    Horizontal,
    /// MangoWM-style vertical tiling
    Vertical,
    /// MangoWM-style center layout
    Center,
}

/// Window geometry
#[derive(Debug, Clone, Copy)]
pub struct WindowGeometry {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

/// Layout information for a window
pub struct LayoutInfo {
    pub geometry: WindowGeometry,
    pub is_focused: bool,
    pub is_floating: bool,
}

impl LayoutManager {
    /// Create a new layout manager
    pub fn new(config: &Config) -> Self {
        info!("📐 Initializing layout manager...");
        
        let mode = match config.layout_mode.as_str() {
            "dynamic" => LayoutMode::Dynamic,
            "horizontal" => LayoutMode::Horizontal,
            "vertical" => LayoutMode::Vertical,
            "center" => LayoutMode::Center,
            _ => LayoutMode::Dynamic,
        };
        
        Self {
            current_mode: mode,
            dynamic: DynamicLayout::new(config),
            horizontal: HorizontalLayout::new(config),
            vertical: VerticalLayout::new(config),
            center: CenterLayout::new(config),
        }
    }
    
    /// Get current layout mode
    pub fn current_mode(&self) -> LayoutMode {
        self.current_mode
    }
    
    /// Set layout mode
    pub fn set_mode(&mut self, mode: LayoutMode) {
        info!("🔄 Switching to {:?} layout", mode);
        self.current_mode = mode;
    }
    
    /// Cycle to next layout mode
    pub fn cycle_mode(&mut self) {
        let next_mode = match self.current_mode {
            LayoutMode::Dynamic => LayoutMode::Horizontal,
            LayoutMode::Horizontal => LayoutMode::Vertical,
            LayoutMode::Vertical => LayoutMode::Center,
            LayoutMode::Center => LayoutMode::Dynamic,
        };
        self.set_mode(next_mode);
    }
    
    /// Calculate window positions for all windows in workspace
    pub fn calculate_layout(&self, workspace_geometry: WindowGeometry, window_count: usize) -> Vec<LayoutInfo> {
        match self.current_mode {
            LayoutMode::Dynamic => self.dynamic.calculate(workspace_geometry, window_count),
            LayoutMode::Horizontal => self.horizontal.calculate(workspace_geometry, window_count),
            LayoutMode::Vertical => self.vertical.calculate(workspace_geometry, window_count),
            LayoutMode::Center => self.center.calculate(workspace_geometry, window_count),
        }
    }
    
    /// Handle window focus change
    pub fn focus_window(&mut self, window_id: usize) {
        debug!("Focusing window {}", window_id);
        // Implementation depends on specific layout
    }
    
    /// Add a new window to the layout
    pub fn add_window(&mut self, window_id: usize) {
        debug!("Adding window {} to layout", window_id);
        // Window will be positioned on next layout calculation
    }
    
    /// Remove a window from the layout
    pub fn remove_window(&mut self, window_id: usize) {
        debug!("Removing window {} from layout", window_id);
        // Layout will be recalculated
    }
}
