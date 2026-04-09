//! Vertical Layout - MangoWM-style
//! 
//! Windows are arranged vertically with smooth scrolling.
//! Focus moves up/down through windows.

use log::debug;
use crate::config::Config;
use super::{WindowGeometry, LayoutInfo};

/// Vertical layout engine with scrolling support
pub struct VerticalLayout {
    /// Current scroll offset (pixels)
    scroll_offset: i32,
    /// Target scroll offset for smooth animation
    target_scroll: i32,
    /// Height of each window row
    row_height: i32,
    /// Gap between windows
    gap_size: i32,
    /// Scroll animation progress (0.0 - 1.0)
    scroll_progress: f32,
}

impl VerticalLayout {
    /// Create new vertical layout
    pub fn new(config: &Config) -> Self {
        Self {
            scroll_offset: 0,
            target_scroll: 0,
            row_height: config.window_height.unwrap_or(600) as i32,
            gap_size: config.gap_size.unwrap_or(8) as i32,
            scroll_progress: 1.0,
        }
    }
    
    /// Calculate vertical layout for windows
    pub fn calculate(&self, workspace: WindowGeometry, window_count: usize) -> Vec<LayoutInfo> {
        if window_count == 0 {
            return Vec::new();
        }
        
        let mut layouts = Vec::with_capacity(window_count);
        let gap = self.gap_size;
        let row_height = self.row_height;
        let total_height = workspace.height;
        
        // Calculate visible rows
        let visible_rows = ((total_height + gap) / (row_height + gap)).max(1);
        
        // Center the layout if we have fewer windows than visible rows
        let start_y = if window_count < visible_rows as usize {
            let content_height = window_count as i32 * row_height + (window_count as i32 - 1) * gap;
            workspace.y + (total_height - content_height) / 2
        } else {
            workspace.y + self.scroll_offset
        };
        
        // Position each window vertically
        for i in 0..window_count {
            let x = workspace.x + gap;
            let y = start_y + (row_height + gap) * i as i32;
            
            layouts.push(LayoutInfo {
                geometry: WindowGeometry {
                    x,
                    y,
                    width: workspace.width - gap * 2,
                    height: row_height,
                },
                is_focused: i == 0, // First window focused by default
                is_floating: false,
            });
        }
        
        layouts
    }
    
    /// Scroll up
    pub fn scroll_up(&mut self, amount: i32) {
        self.target_scroll = (self.target_scroll - amount).max(0);
        self.scroll_progress = 0.0;
        debug!("Scrolling up, target: {}", self.target_scroll);
    }
    
    /// Scroll down
    pub fn scroll_down(&mut self, amount: i32, max_scroll: i32) {
        self.target_scroll = (self.target_scroll + amount).min(max_scroll);
        self.scroll_progress = 0.0;
        debug!("Scrolling down, target: {}", self.target_scroll);
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
    
    /// Set row height
    pub fn set_row_height(&mut self, height: i32) {
        self.row_height = height;
        debug!("Row height: {}", self.row_height);
    }
}
