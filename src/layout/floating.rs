//! Floating Layout - Hyprland-style
//! 
//! Windows can be freely positioned and resized.
//! Supports drag-to-move and drag-to-resize.

use log::{info, debug};
use crate::config::Config;
use super::{WindowGeometry, LayoutInfo};
use crate::workspace::WindowId;

/// Floating window state
pub struct FloatingWindow {
    /// Window ID
    pub id: WindowId,
    /// Window geometry
    pub geometry: WindowGeometry,
    /// Whether window is being dragged
    pub is_dragging: bool,
    /// Whether window is being resized
    pub is_resizing: bool,
    /// Drag offset (mouse position relative to window)
    pub drag_offset_x: i32,
    pub drag_offset_y: i32,
}

/// Floating layout manager
pub struct FloatingLayout {
    /// All floating windows
    pub windows: Vec<FloatingWindow>,
    /// Default window size
    pub default_width: i32,
    pub default_height: i32,
    /// Currently dragged window
    pub dragged_window: Option<WindowId>,
    /// Currently resized window
    pub resized_window: Option<WindowId>,
}

impl FloatingLayout {
    /// Create new floating layout
    pub fn new(config: &Config) -> Self {
        info!("🎈 Initializing Floating layout...");
        
        Self {
            windows: Vec::new(),
            default_width: config.window_width.unwrap_or(800) as i32,
            default_height: config.window_height.unwrap_or(600) as i32,
            dragged_window: None,
            resized_window: None,
        }
    }
    
    /// Add a floating window
    pub fn add_window(&mut self, window_id: WindowId, parent_geometry: Option<WindowGeometry>) {
        debug!("🎈 Adding floating window {}", window_id);
        
        // Position window at center or random offset
        let geometry = if let Some(parent) = parent_geometry {
            // Center in parent area with some randomness
            let offset_x = ((window_id as i32 * 30) % 100) - 50;
            let offset_y = ((window_id as i32 * 40) % 100) - 50;
            
            WindowGeometry {
                x: parent.x + (parent.width - self.default_width) / 2 + offset_x,
                y: parent.y + (parent.height - self.default_height) / 2 + offset_y,
                width: self.default_width,
                height: self.default_height,
            }
        } else {
            // Default position
            WindowGeometry {
                x: 100 + (window_id as i32 * 30),
                y: 100 + (window_id as i32 * 40),
                width: self.default_width,
                height: self.default_height,
            }
        };
        
        self.windows.push(FloatingWindow {
            id: window_id,
            geometry,
            is_dragging: false,
            is_resizing: false,
            drag_offset_x: 0,
            drag_offset_y: 0,
        });
        
        info!("✅ Floating window {} added at {}x{}+{}+{}", 
              window_id, geometry.width, geometry.height, geometry.x, geometry.y);
    }
    
    /// Remove a floating window
    pub fn remove_window(&mut self, window_id: WindowId) {
        debug!("🎈 Removing floating window {}", window_id);
        self.windows.retain(|w| w.id != window_id);
        
        if self.dragged_window == Some(window_id) {
            self.dragged_window = None;
        }
        if self.resized_window == Some(window_id) {
            self.resized_window = None;
        }
    }
    
    /// Start dragging a window
    pub fn start_drag(&mut self, window_id: WindowId, mouse_x: i32, mouse_y: i32) {
        if let Some(window) = self.windows.iter_mut().find(|w| w.id == window_id) {
            window.is_dragging = true;
            window.drag_offset_x = mouse_x - window.geometry.x;
            window.drag_offset_y = mouse_y - window.geometry.y;
            self.dragged_window = Some(window_id);
            debug!("🎈 Started dragging window {}", window_id);
        }
    }
    
    /// Update drag position
    pub fn update_drag(&mut self, mouse_x: i32, mouse_y: i32, bounds: WindowGeometry) {
        if let Some(window_id) = self.dragged_window {
            if let Some(window) = self.windows.iter_mut().find(|w| w.id == window_id) {
                let new_x = mouse_x - window.drag_offset_x;
                let new_y = mouse_y - window.drag_offset_y;
                
                // Clamp to bounds
                window.geometry.x = new_x.max(bounds.x).min(bounds.x + bounds.width - window.geometry.width);
                window.geometry.y = new_y.max(bounds.y).min(bounds.y + bounds.height - window.geometry.height);
                
                debug!("🎈 Dragging window {} to {}+{}", window_id, new_x, new_y);
            }
        }
    }
    
    /// End drag operation
    pub fn end_drag(&mut self) {
        if let Some(window_id) = self.dragged_window {
            if let Some(window) = self.windows.iter_mut().find(|w| w.id == window_id) {
                window.is_dragging = false;
            }
            self.dragged_window = None;
            debug!("🎈 Ended drag operation");
        }
    }
    
    /// Start resizing a window
    pub fn start_resize(&mut self, window_id: WindowId) {
        if let Some(window) = self.windows.iter_mut().find(|w| w.id == window_id) {
            window.is_resizing = true;
            self.resized_window = Some(window_id);
            debug!("🎈 Started resizing window {}", window_id);
        }
    }
    
    /// Update resize dimensions
    pub fn update_resize(&mut self, width_delta: i32, height_delta: i32, min_width: i32, min_height: i32) {
        if let Some(window_id) = self.resized_window {
            if let Some(window) = self.windows.iter_mut().find(|w| w.id == window_id) {
                window.geometry.width = (window.geometry.width + width_delta).max(min_width);
                window.geometry.height = (window.geometry.height + height_delta).max(min_height);
                
                debug!("🎈 Resizing window {} to {}x{}", window_id, window.geometry.width, window.geometry.height);
            }
        }
    }
    
    /// End resize operation
    pub fn end_resize(&mut self) {
        if let Some(window_id) = self.resized_window {
            if let Some(window) = self.windows.iter_mut().find(|w| w.id == window_id) {
                window.is_resizing = false;
            }
            self.resized_window = None;
            debug!("🎈 Ended resize operation");
        }
    }
    
    /// Calculate floating layout (returns all window positions)
    pub fn calculate(&self, _workspace: WindowGeometry, _window_count: usize) -> Vec<LayoutInfo> {
        self.windows.iter().map(|w| LayoutInfo {
            geometry: w.geometry.clone(),
            is_focused: false, // Focus handled separately
            is_floating: true,
        }).collect()
    }
    
    /// Get window at position
    pub fn get_window_at(&self, x: i32, y: i32) -> Option<WindowId> {
        // Check windows in reverse order (top to bottom)
        for window in self.windows.iter().rev() {
            if x >= window.geometry.x && x <= window.geometry.x + window.geometry.width &&
               y >= window.geometry.y && y <= window.geometry.y + window.geometry.height {
                return Some(window.id);
            }
        }
        None
    }
    
    /// Bring window to front
    pub fn bring_to_front(&mut self, window_id: WindowId) {
        if let Some(pos) = self.windows.iter().position(|w| w.id == window_id) {
            let window = self.windows.remove(pos);
            self.windows.push(window);
            debug!("🎈 Brought window {} to front", window_id);
        }
    }
    
    /// Get all window geometries
    pub fn get_all_geometries(&self) -> Vec<(WindowId, WindowGeometry)> {
        self.windows.iter().map(|w| (w.id, w.geometry.clone())).collect()
    }
}

impl Clone for FloatingWindow {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            geometry: self.geometry.clone(),
            is_dragging: self.is_dragging,
            is_resizing: self.is_resizing,
            drag_offset_x: self.drag_offset_x,
            drag_offset_y: self.drag_offset_y,
        }
    }
}
