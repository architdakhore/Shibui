//! Error overlay module
//! 
//! Displays config errors on screen similar to Hyprland's error bar.
//! Shows validation errors when config reload fails.

use log::{info, debug, error};

/// Error overlay for displaying config errors
pub struct ErrorOverlay {
    /// Whether overlay is visible
    pub visible: bool,
    /// Error message to display
    pub message: String,
    /// Error severity
    pub severity: ErrorSeverity,
    /// Display duration in seconds (0 = persistent until dismissed)
    pub duration_secs: u32,
    /// Time when error was shown
    pub shown_at: Option<std::time::Instant>,
}

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorSeverity {
    /// Info message (blue)
    Info,
    /// Warning (yellow)
    Warning,
    /// Error (red)
    Error,
    /// Critical error (bright red, persistent)
    Critical,
}

impl ErrorOverlay {
    /// Create new error overlay
    pub fn new() -> Self {
        Self {
            visible: false,
            message: String::new(),
            severity: ErrorSeverity::Error,
            duration_secs: 0,
            shown_at: None,
        }
    }
    
    /// Show an info message
    pub fn show_info(&mut self, message: &str) {
        self.show(message, ErrorSeverity::Info, 5);
    }
    
    /// Show a warning
    pub fn show_warning(&mut self, message: &str) {
        self.show(message, ErrorSeverity::Warning, 10);
    }
    
    /// Show an error
    pub fn show_error(&mut self, message: &str) {
        self.show(message, ErrorSeverity::Error, 0); // Persistent
    }
    
    /// Show a critical error
    pub fn show_critical(&mut self, message: &str) {
        self.show(message, ErrorSeverity::Critical, 0); // Persistent
    }
    
    /// Show error with severity and duration
    pub fn show(&mut self, message: &str, severity: ErrorSeverity, duration_secs: u32) {
        info!("🚨 Error overlay: {} ({:?})", message, severity);
        
        self.visible = true;
        self.message = message.to_string();
        self.severity = severity;
        self.duration_secs = duration_secs;
        self.shown_at = Some(std::time::Instant::now());
    }
    
    /// Hide the overlay
    pub fn hide(&mut self) {
        debug!("👁️ Hiding error overlay");
        self.visible = false;
        self.message.clear();
        self.shown_at = None;
    }
    
    /// Check if overlay should auto-hide
    pub fn should_auto_hide(&self) -> bool {
        if self.duration_secs == 0 {
            return false; // Persistent
        }
        
        if let Some(shown_at) = self.shown_at {
            shown_at.elapsed().as_secs() >= self.duration_secs
        } else {
            false
        }
    }
    
    /// Update overlay (check for auto-hide)
    pub fn update(&mut self) {
        if self.visible && self.should_auto_hide() {
            self.hide();
        }
    }
    
    /// Get overlay opacity for rendering (0.0 - 1.0)
    pub fn opacity(&self) -> f32 {
        if !self.visible {
            return 0.0;
        }
        
        // Fade out in last second
        if let Some(shown_at) = self.shown_at {
            let elapsed = shown_at.elapsed().as_secs_f32();
            let total = self.duration_secs as f32;
            
            if total > 1.0 && elapsed > total - 1.0 {
                return (total - elapsed).max(0.0);
            }
        }
        
        1.0
    }
    
    /// Get background color based on severity
    pub fn background_color(&self) -> [f32; 4] {
        match self.severity {
            ErrorSeverity::Info => [0.2, 0.4, 0.8, 0.9],      // Blue
            ErrorSeverity::Warning => [0.9, 0.7, 0.1, 0.9],   // Yellow
            ErrorSeverity::Error => [0.9, 0.2, 0.2, 0.9],     // Red
            ErrorSeverity::Critical => [1.0, 0.0, 0.0, 0.95], // Bright red
        }
    }
    
    /// Check if overlay is currently visible
    pub fn is_visible(&self) -> bool {
        self.visible && self.opacity() > 0.0
    }
}

impl Default for ErrorOverlay {
    fn default() -> Self {
        Self::new()
    }
}

/// Error notification for IPC
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorNotification {
    /// Error message
    pub message: String,
    /// Severity
    pub severity: String,
    /// Timestamp
    pub timestamp: u64,
    /// Whether error is persistent
    pub persistent: bool,
}

impl ErrorNotification {
    /// Create from error overlay
    pub fn from_overlay(overlay: &ErrorOverlay) -> Self {
        Self {
            message: overlay.message.clone(),
            severity: match overlay.severity {
                ErrorSeverity::Info => "info".to_string(),
                ErrorSeverity::Warning => "warning".to_string(),
                ErrorSeverity::Error => "error".to_string(),
                ErrorSeverity::Critical => "critical".to_string(),
            },
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            persistent: overlay.duration_secs == 0,
        }
    }
}
