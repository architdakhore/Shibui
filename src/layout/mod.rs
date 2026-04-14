//! Layout management module
//! 
//! Implements all layouts:
//! - Dwindle (Hyprland-style spiral)
//! - Horizontal (niri-style scrolling)
//! - Vertical (MangoWM-style scrolling)
//! - Center (MangoWM-style)
//! - Floating (Hyprland-style)

use log::{info, debug};
use crate::config::Config;

mod dwindle;
mod horizontal;
mod vertical;
mod center;
mod floating;

pub use dwindle::DwindleLayout;
pub use dwindle::DwindleLayout;
pub use horizontal::HorizontalLayout;
pub use vertical::VerticalLayout;
pub use center::CenterLayout;
pub use floating::FloatingLayout;

/// Layout manager handles switching between different layout modes
pub struct LayoutManager {
    /// Current layout mode
    current_mode: LayoutMode,
    /// Dwindle layout engine (Hyprland-style spiral)
    dwindle: DwindleLayout,
    /// Horizontal layout engine (niri-style scrolling)
    horizontal: HorizontalLayout,
    /// Vertical layout engine (MangoWM-style scrolling)
    vertical: VerticalLayout,
    /// Center layout engine (MangoWM-style)
    center: CenterLayout,
    /// Floating layout engine (Hyprland-style)
    floating: FloatingLayout,
}

/// Available layout modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutMode {
    /// Hyprland-style dwindle spiral tiling
    Dwindle,
    /// niri-style horizontal tiling
    Horizontal,
    /// MangoWM-style vertical tiling
    Vertical,
    /// MangoWM-style center layout
    Center,
    /// Hyprland-style floating
    Floating,
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
            "dwindle" => LayoutMode::Dwindle,
            "horizontal" => LayoutMode::Horizontal,
            "vertical" => LayoutMode::Vertical,
            "center" => LayoutMode::Center,
            "floating" => LayoutMode::Floating,
            _ => LayoutMode::Dwindle,
        };
        
        Self {
            current_mode: mode,
            dwindle: DwindleLayout::new(config),
            horizontal: HorizontalLayout::new(config),
            vertical: VerticalLayout::new(config),
            center: CenterLayout::new(config),
            floating: FloatingLayout::new(config),
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
            LayoutMode::Dwindle => LayoutMode::Horizontal,
            LayoutMode::Horizontal => LayoutMode::Vertical,
            LayoutMode::Vertical => LayoutMode::Center,
            LayoutMode::Center => LayoutMode::Floating,
            LayoutMode::Floating => LayoutMode::Dwindle,
        };
        self.set_mode(next_mode);
    }
    
    /// Calculate window positions for all windows in workspace
    pub fn calculate_layout(&self, workspace_geometry: WindowGeometry, window_count: usize) -> Vec<LayoutInfo> {
        match self.current_mode {
            LayoutMode::Dwindle => self.dwindle.calculate(workspace_geometry, window_count),
            LayoutMode::Horizontal => self.horizontal.calculate(workspace_geometry, window_count),
            LayoutMode::Vertical => self.vertical.calculate(workspace_geometry, window_count),
            LayoutMode::Center => self.center.calculate(workspace_geometry, window_count),
            LayoutMode::Floating => {
                // For floating, return stored geometries
                self.floating.calculate(workspace_geometry, window_count)
            }
        }
    }
    
    /// Handle window focus change
    pub fn focus_window(&mut self, window_id: usize) {
        debug!("Focusing window {}", window_id);
        // Implementation depends on specific layout
    }
    
    /// Add a new window to the layout
    pub fn add_window(&mut self, window_id: usize, workspace_geo: Option<WindowGeometry>) {
        debug!("Adding window {} to layout", window_id);
        
        if self.current_mode == LayoutMode::Floating {
            self.floating.add_window(window_id, workspace_geo);
        }
        // Other layouts compute positions automatically
    }
    
    /// Remove a window from the layout
    pub fn remove_window(&mut self, window_id: usize) {
        debug!("Removing window {} from layout", window_id);
        
        if self.current_mode == LayoutMode::Floating {
            self.floating.remove_window(window_id);
        }
        // Other layouts will recalculate
    }
    
    /// Handle floating window drag start
    pub fn start_drag(&mut self, window_id: usize, mouse_x: i32, mouse_y: i32) {
        if self.current_mode == LayoutMode::Floating {
            self.floating.start_drag(window_id, mouse_x, mouse_y);
        }
    }
    
    /// Handle floating window drag update
    pub fn update_drag(&mut self, mouse_x: i32, mouse_y: i32, bounds: WindowGeometry) {
        if self.current_mode == LayoutMode::Floating {
            self.floating.update_drag(mouse_x, mouse_y, bounds);
        }
    }
    
    /// Handle floating window drag end
    pub fn end_drag(&mut self) {
        if self.current_mode == LayoutMode::Floating {
            self.floating.end_drag();
        }
    }
    
    /// Handle floating window resize
    pub fn start_resize(&mut self, window_id: usize) {
        if self.current_mode == LayoutMode::Floating {
            self.floating.start_resize(window_id);
        }
    }
    
    pub fn update_resize(&mut self, width_delta: i32, height_delta: i32, min_w: i32, min_h: i32) {
        if self.current_mode == LayoutMode::Floating {
            self.floating.update_resize(width_delta, height_delta, min_w, min_h);
        }
    }
    
    pub fn end_resize(&mut self) {
        if self.current_mode == LayoutMode::Floating {
            self.floating.end_resize();
        }
    }
}
