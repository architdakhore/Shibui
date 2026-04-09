//! Decorations module
//! 
//! Server-side window decorations (SSD) including borders, shadows, and title bars.

use log::{debug};
use crate::window::Window;
use crate::layout::WindowGeometry;

/// Window border style
#[derive(Debug, Clone)]
pub struct BorderStyle {
    /// Border width (pixels)
    pub width: i32,
    /// Border color (hex)
    pub color: String,
    /// Active border color (hex)
    pub active_color: String,
    /// Border radius (pixels)
    pub radius: i32,
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self {
            width: 2,
            color: "#4C7899".to_string(),
            active_color: "#57A0C9".to_string(),
            radius: 0,
        }
    }
}

/// Shadow style
#[derive(Debug, Clone)]
pub struct ShadowStyle {
    /// Enable shadow
    pub enabled: bool,
    /// Shadow color (hex)
    pub color: String,
    /// Shadow spread (pixels)
    pub spread: i32,
    /// Shadow blur (pixels)
    pub blur: i32,
    /// Shadow offset X
    pub offset_x: i32,
    /// Shadow offset Y
    pub offset_y: i32,
}

impl Default for ShadowStyle {
    fn default() -> Self {
        Self {
            enabled: true,
            color: "#000000".to_string(),
            spread: 4,
            blur: 8,
            offset_x: 0,
            offset_y: 4,
        }
    }
}

/// Title bar style
#[derive(Debug, Clone)]
pub struct TitleBarStyle {
    /// Enable title bar
    pub enabled: bool,
    /// Title bar height (pixels)
    pub height: i32,
    /// Background color (hex)
    pub bg_color: String,
    /// Text color (hex)
    pub text_color: String,
    /// Font family
    pub font: String,
    /// Font size
    pub font_size: i32,
    /// Button alignment
    pub button_alignment: ButtonAlignment,
}

/// Button alignment
#[derive(Debug, Clone, Copy)]
pub enum ButtonAlignment {
    Left,
    Right,
}

impl Default for TitleBarStyle {
    fn default() -> Self {
        Self {
            enabled: false, // Default to CSD (client-side decorations)
            height: 30,
            bg_color: "#2a2a2a".to_string(),
            text_color: "#ffffff".to_string(),
            font: "Sans".to_string(),
            font_size: 12,
            button_alignment: ButtonAlignment::Right,
        }
    }
}

/// Decoration manager
pub struct DecorationManager {
    /// Border style
    pub border_style: BorderStyle,
    /// Shadow style
    pub shadow_style: ShadowStyle,
    /// Title bar style
    pub title_bar_style: TitleBarStyle,
}

impl DecorationManager {
    /// Create new decoration manager
    pub fn new() -> Self {
        Self {
            border_style: BorderStyle::default(),
            shadow_style: ShadowStyle::default(),
            title_bar_style: TitleBarStyle::default(),
        }
    }
    
    /// Calculate border geometry
    pub fn border_geometry(&self, window: &Window) -> WindowGeometry {
        let border = self.border_style.width;
        
        WindowGeometry {
            x: window.geometry.x - border,
            y: window.geometry.y - border,
            width: window.geometry.width + (border * 2),
            height: window.geometry.height + (border * 2),
        }
    }
    
    /// Get border color for window
    pub fn border_color(&self, window: &Window) -> &str {
        if window.focused {
            &self.border_style.active_color
        } else {
            &self.border_style.color
        }
    }
    
    /// Calculate shadow geometry
    pub fn shadow_geometry(&self, window: &Window) -> WindowGeometry {
        let spread = self.shadow_style.spread;
        let blur = self.shadow_style.blur;
        let offset = spread + blur;
        
        WindowGeometry {
            x: window.geometry.x - offset + self.shadow_style.offset_x,
            y: window.geometry.y - offset + self.shadow_style.offset_y,
            width: window.geometry.width + (offset * 2),
            height: window.geometry.height + (offset * 2),
        }
    }
    
    /// Check if window should have decorations
    pub fn should_decorate(&self, window: &Window) -> bool {
        // Don't decorate fullscreen windows
        if window.state == crate::window::WindowState::Fullscreen {
            return false;
        }
        
        // Don't decorate if CSD is preferred
        // TODO: Check window preferences
        
        true
    }
    
    /// Update border style
    pub fn set_border_style(&mut self, style: BorderStyle) {
        debug!("Updating border style");
        self.border_style = style;
    }
    
    /// Update shadow style
    pub fn set_shadow_style(&mut self, style: ShadowStyle) {
        debug!("Updating shadow style");
        self.shadow_style = style;
    }
    
    /// Update title bar style
    pub fn set_title_bar_style(&mut self, style: TitleBarStyle) {
        debug!("Updating title bar style");
        self.title_bar_style = style;
    }
}

impl Default for DecorationManager {
    fn default() -> Self {
        Self::new()
    }
}
