//! Window Management module
//! 
//! Handles window lifecycle, positioning, and state.

use log::{info, debug};
use crate::layout::{WindowGeometry, LayoutInfo};
use crate::workspace::WindowId;

/// Window state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowState {
    /// Normal tiled window
    Tiled,
    /// Floating window
    Floating,
    /// Fullscreen window
    Fullscreen,
    /// Maximized window
    Maximized,
    /// Minimized window
    Minimized,
}

/// Parse hex color string to RGB array
fn parse_color(hex: &str) -> [f32; 3] {
    // Remove '#' if present
    let cleaned = hex.trim_start_matches('#');
    
    if cleaned.len() != 6 {
        // Default to blue if invalid
        return [0.298, 0.477, 0.614];
    }
    
    let r = u8::from_str_radix(&cleaned[0..2], 16).unwrap_or(76) as f32 / 255.0;
    let g = u8::from_str_radix(&cleaned[2..4], 16).unwrap_or(120) as f32 / 255.0;
    let b = u8::from_str_radix(&cleaned[4..6], 16).unwrap_or(153) as f32 / 255.0;
    
    [r, g, b]
}

/// Window information
pub struct Window {
    /// Window ID
    pub id: WindowId,
    /// Window title
    pub title: String,
    /// Application ID
    pub app_id: String,
    /// Current state
    pub state: WindowState,
    /// Window geometry
    pub geometry: WindowGeometry,
    /// Whether window is focused
    pub focused: bool,
    /// Whether window is urgent
    pub urgent: bool,
    /// Workspace ID
    pub workspace_id: i32,
    /// Border color (pre-parsed RGB array for efficient rendering)
    pub border_color: [f32; 3],
    /// Border width
    pub border_width: i32,
}

impl Window {
    /// Create new window
    pub fn new(id: WindowId, title: String, app_id: String, workspace_id: i32) -> Self {
        Self {
            id,
            title,
            app_id,
            state: WindowState::Tiled,
            geometry: WindowGeometry {
                x: 0,
                y: 0,
                width: 800,
                height: 600,
            },
            focused: false,
            urgent: false,
            workspace_id,
            border_color: parse_color("#4C7899"),  // Parse once at construction
            border_width: 2,
        }
    }
    
    /// Set window geometry
    pub fn set_geometry(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.geometry = WindowGeometry { x, y, width, height };
        debug!("Window {} geometry: {}x{}+{}+{}", self.id, width, height, x, y);
    }
    
    /// Apply layout info
    pub fn apply_layout(&mut self, layout: &LayoutInfo) {
        self.geometry = layout.geometry;
        self.focused = layout.is_focused;
        
        if layout.is_floating {
            self.state = WindowState::Floating;
        } else {
            self.state = WindowState::Tiled;
        }
    }
    
    /// Set focused state
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
        debug!("Window {} focused: {}", self.id, focused);
    }
    
    /// Set urgent state
    pub fn set_urgent(&mut self, urgent: bool) {
        self.urgent = urgent;
        debug!("Window {} urgent: {}", self.id, urgent);
    }
    
    /// Set fullscreen
    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.state = if fullscreen {
            WindowState::Fullscreen
        } else {
            WindowState::Tiled
        };
        debug!("Window {} fullscreen: {}", self.id, fullscreen);
    }
    
    /// Set maximized
    pub fn set_maximized(&mut self, maximized: bool) {
        self.state = if maximized {
            WindowState::Maximized
        } else {
            WindowState::Tiled
        };
        debug!("Window {} maximized: {}", self.id, maximized);
    }
    
    /// Toggle floating
    pub fn toggle_floating(&mut self) {
        self.state = match self.state {
            WindowState::Tiled => WindowState::Floating,
            WindowState::Floating => WindowState::Tiled,
            _ => self.state,
        };
        debug!("Window {} toggled floating", self.id);
    }
    
    /// Request window closure
    /// 
    /// # Returns
    /// The window ID that should be closed. The caller (WindowManager or XdgWmHandler)
    /// must send the actual close request to the Wayland surface.
    /// 
    /// # Design Notes
    /// - This method only returns the ID; the actual close is handled by the protocol layer
    /// - The XdgWmHandler will send a `close()` event to the Wayland client
    /// - The client may choose to ignore the request (though well-behaved clients will close)
    /// 
    /// # TODO
    /// Integrate with Smithay's `desktop::Window` type to unify logical and surface state.
    /// This would allow direct access to the Wayland surface for sending close requests.
    pub fn request_close(&self) -> WindowId {
        debug!("🚪 Close requested for window {}", self.id);
        self.id
    }
}

/// Window manager
pub struct WindowManager {
    /// All windows
    pub windows: Vec<Window>,
    /// Next window ID
    next_id: WindowId,
    /// Currently focused window
    pub focused_window: Option<WindowId>,
}

impl WindowManager {
    /// Create new window manager
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            next_id: 0,
            focused_window: None,
        }
    }
    
    /// Add a new window
    pub fn add_window(&mut self, title: String, app_id: String, workspace_id: i32) -> WindowId {
        let id = self.next_id;
        self.next_id += 1;
        
        let window = Window::new(id, title, app_id, workspace_id);
        info!("🪟 Window created: {} ({})", title, app_id);
        
        self.windows.push(window);
        self.focused_window = Some(id);
        
        id
    }
    
    /// Remove a window
    pub fn remove_window(&mut self, id: WindowId) {
        info!("🪟 Window removed: {}", id);
        self.windows.retain(|w| w.id != id);
        
        if self.focused_window == Some(id) {
            self.focused_window = None;
        }
    }
    
    /// Get window by ID
    pub fn get_window(&self, id: WindowId) -> Option<&Window> {
        self.windows.iter().find(|w| w.id == id)
    }
    
    /// Get mutable window by ID
    pub fn get_window_mut(&mut self, id: WindowId) -> Option<&mut Window> {
        self.windows.iter_mut().find(|w| w.id == id)
    }
    
    /// Get focused window
    pub fn focused_window(&self) -> Option<&Window> {
        self.focused_window.and_then(|id| self.get_window(id))
    }
    
    /// Set focused window
    pub fn focus_window(&mut self, id: WindowId) {
        // Clear old focus
        if let Some(old) = self.focused_window {
            if let Some(window) = self.get_window_mut(old) {
                window.set_focused(false);
            }
        }
        
        // Set new focus
        if let Some(window) = self.get_window_mut(id) {
            window.set_focused(true);
            self.focused_window = Some(id);
            info!("🪟 Focused window: {} ({})", window.title, window.app_id);
        }
    }
    
    /// Get windows in workspace
    pub fn windows_in_workspace(&self, workspace_id: i32) -> Vec<&Window> {
        self.windows.iter().filter(|w| w.workspace_id == workspace_id).collect()
    }
    
    /// Move window to workspace
    pub fn move_to_workspace(&mut self, window_id: WindowId, workspace_id: i32) {
        if let Some(window) = self.get_window_mut(window_id) {
            window.workspace_id = workspace_id;
            info!("🪟 Moved window {} to workspace {}", window_id, workspace_id);
        }
    }
    
    /// Get all windows
    pub fn all_windows(&self) -> &[Window] {
        &self.windows
    }
    
    /// Get window count
    pub fn window_count(&self) -> usize {
        self.windows.len()
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}
