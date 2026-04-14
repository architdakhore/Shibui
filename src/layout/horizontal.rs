//! Horizontal Layout - niri-style
//! 
//! Windows are arranged horizontally with smooth scrolling.
//! Focus moves left/right through windows.

use log::debug;
use crate::config::Config;
use super::{WindowGeometry, LayoutInfo};

/// Horizontal layout engine with scrolling support
pub struct HorizontalLayout {
    /// Current scroll offset (pixels)
    scroll_offset: i32,
    /// Target scroll offset for smooth animation
    target_scroll: i32,
    /// Width of each window column
    column_width: i32,
    /// Gap between windows
    gap_size: i32,
    /// Scroll animation progress (0.0 - 1.0)
    scroll_progress: f32,
}

impl HorizontalLayout {
    /// Create new horizontal layout
    pub fn new(config: &Config) -> Self {
        Self {
            scroll_offset: 0,
            target_scroll: 0,
            column_width: config.window_width.unwrap_or(800) as i32,
            gap_size: config.gap_size.unwrap_or(8) as i32,
            scroll_progress: 1.0,
        }
    }
    
    /// Calculate horizontal layout for windows
    pub fn calculate(&self, workspace: WindowGeometry, window_count: usize) -> Vec<LayoutInfo> {
        if window_count == 0 {
            return Vec::new();
        }
        
        let mut layouts = Vec::with_capacity(window_count);
        let gap = self.gap_size;
        let col_width = self.column_width;
        let total_width = workspace.width;
        
        // Calculate visible columns
        let visible_columns = ((total_width + gap) / (col_width + gap)).max(1);
        
        // Center the layout if we have fewer windows than visible columns
        let start_x = if window_count < visible_columns as usize {
            let content_width = window_count as i32 * col_width + (window_count as i32 - 1) * gap;
            workspace.x + (total_width - content_width) / 2
        } else {
            workspace.x + self.scroll_offset
        };
        
        // Position each window horizontally
        for i in 0..window_count {
            let x = start_x + (col_width + gap) * i as i32;
            let y = workspace.y + gap;
            
            layouts.push(LayoutInfo {
                geometry: WindowGeometry {
                    x,
                    y,
                    width: col_width,
                    height: workspace.height - gap * 2,
                },
                is_focused: i == 0, // First window focused by default
                is_floating: false,
            });
        }
        
        layouts
    }
    
    /// Scroll left
    pub fn scroll_left(&mut self, amount: i32) {
        self.target_scroll = (self.target_scroll - amount).max(0);
        self.scroll_progress = 0.0;
        debug!("Scrolling left, target: {}", self.target_scroll);
    }
    
    /// Scroll right
    pub fn scroll_right(&mut self, amount: i32, max_scroll: i32) {
        self.target_scroll = (self.target_scroll + amount).min(max_scroll);
        self.scroll_progress = 0.0;
        debug!("Scrolling right, target: {}", self.target_scroll);
    }
    
    /// Update scroll animation
    pub fn update_scroll(&mut self, delta_time: f32) {
        if self.scroll_progress < 1.0 {
            self.scroll_progress += delta_time * 5.0; // 5 units per second
            self.scroll_progress = self.scroll_progress.min(1.0);
            
            // Smooth interpolation
            let progress = self.ease_out_cubic(self.scroll_progress);
            self.scroll_offset = self.target_scroll + ((self.scroll_offset - self.target_scroll) as f32 * (1.0 - progress)) as i32;
        }
    }
    
    /// Ease-out cubic interpolation
    fn ease_out_cubic(&self, x: f32) -> f32 {
        1.0 - (1.0 - x).powi(3)
    }
    
    /// Get current scroll offset
    pub fn scroll_offset(&self) -> i32 {
        self.scroll_offset
    }
    
    /// Set column width
    pub fn set_column_width(&mut self, width: i32) {
        self.column_width = width;
        debug!("Column width: {}", self.column_width);
    }
}
