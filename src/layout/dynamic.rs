//! Dynamic Layout - Hyprland-style
//! 
//! Automatically adapts layout based on number of windows.
//! Uses a master-stack arrangement that evolves as windows are added.

use log::debug;
use crate::config::Config;
use super::{WindowGeometry, LayoutInfo, LayoutMode};

/// Dynamic layout engine
pub struct DynamicLayout {
    /// Number of windows in master column
    master_count: usize,
    /// Master column width ratio (0.0 - 1.0)
    master_ratio: f32,
    /// Gap between windows
    gap_size: i32,
}

impl DynamicLayout {
    /// Create new dynamic layout
    pub fn new(config: &Config) -> Self {
        Self {
            master_count: 1,
            master_ratio: config.master_ratio.unwrap_or(0.55),
            gap_size: config.gap_size.unwrap_or(8) as i32,
        }
    }
    
    /// Calculate dynamic layout for windows
    pub fn calculate(&self, workspace: WindowGeometry, window_count: usize) -> Vec<LayoutInfo> {
        if window_count == 0 {
            return Vec::new();
        }
        
        let mut layouts = Vec::with_capacity(window_count);
        let gap = self.gap_size;
        
        // Single window - full screen
        if window_count == 1 {
            layouts.push(LayoutInfo {
                geometry: WindowGeometry {
                    x: workspace.x + gap,
                    y: workspace.y + gap,
                    width: workspace.width - (gap * 2),
                    height: workspace.height - (gap * 2),
                },
                is_focused: true,
                is_floating: false,
            });
            return layouts;
        }
        
        // Two windows - side by side
        if window_count == 2 {
            let master_width = ((workspace.width - gap * 3) as f32 * self.master_ratio) as i32;
            let stack_width = workspace.width - gap * 3 - master_width;
            
            // Master (left)
            layouts.push(LayoutInfo {
                geometry: WindowGeometry {
                    x: workspace.x + gap,
                    y: workspace.y + gap,
                    width: master_width,
                    height: workspace.height - gap * 2,
                },
                is_focused: false,
                is_floating: false,
            });
            
            // Stack (right)
            layouts.push(LayoutInfo {
                geometry: WindowGeometry {
                    x: workspace.x + gap * 2 + master_width,
                    y: workspace.y + gap,
                    width: stack_width,
                    height: workspace.height - gap * 2,
                },
                is_focused: false,
                is_floating: false,
            });
            return layouts;
        }
        
        // Three or more windows - master + stack
        let master_width = ((workspace.width - gap * 3) as f32 * self.master_ratio) as i32;
        let stack_width = workspace.width - gap * 3 - master_width;
        let stack_count = window_count - 1;
        
        // Master window (full height on left)
        layouts.push(LayoutInfo {
            geometry: WindowGeometry {
                x: workspace.x + gap,
                y: workspace.y + gap,
                width: master_width,
                height: workspace.height - gap * 2,
            },
            is_focused: false,
            is_floating: false,
        });
        
        // Stack windows (vertical split on right)
        let stack_height = (workspace.height - gap * 2 - gap * (stack_count - 1) as i32) / stack_count as i32;
        
        for i in 0..stack_count {
            layouts.push(LayoutInfo {
                geometry: WindowGeometry {
                    x: workspace.x + gap * 2 + master_width,
                    y: workspace.y + gap + (stack_height + gap) * i as i32,
                    width: stack_width,
                    height: stack_height,
                },
                is_focused: false,
                is_floating: false,
            });
        }
        
        layouts
    }
    
    /// Increase master area size
    pub fn increase_master_ratio(&mut self) {
        self.master_ratio = (self.master_ratio + 0.05).min(0.8);
        debug!("Master ratio: {}", self.master_ratio);
    }
    
    /// Decrease master area size
    pub fn decrease_master_ratio(&mut self) {
        self.master_ratio = (self.master_ratio - 0.05).max(0.2);
        debug!("Master ratio: {}", self.master_ratio);
    }
    
    /// Increase number of master windows
    pub fn increase_master_count(&mut self) {
        self.master_count += 1;
        debug!("Master count: {}", self.master_count);
    }
    
    /// Decrease number of master windows
    pub fn decrease_master_count(&mut self) {
        if self.master_count > 1 {
            self.master_count -= 1;
            debug!("Master count: {}", self.master_count);
        }
    }
}
