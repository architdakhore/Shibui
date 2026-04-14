//! Center Layout - MangoWM-style
//! 
//! Main window is centered with other windows tiled around it.
//! Supports multiple arrangement patterns.

use log::debug;
use crate::config::Config;
use super::{WindowGeometry, LayoutInfo};

/// Center layout engine
pub struct CenterLayout {
    /// Center window size ratio (0.0 - 1.0)
    center_ratio: f32,
    /// Gap between windows
    gap_size: i32,
    /// Layout pattern
    pattern: CenterPattern,
}

/// Center layout pattern
#[derive(Debug, Clone, Copy)]
pub enum CenterPattern {
    /// Main window centered, others in a column on right
    RightColumn,
    /// Main window centered, others in a column on left
    LeftColumn,
    /// Main window centered, others split top/bottom
    TopBottom,
    /// Main window centered, others in grid around it
    Grid,
}

impl CenterLayout {
    /// Create new center layout
    pub fn new(config: &Config) -> Self {
        Self {
            center_ratio: config.center_ratio.unwrap_or(0.6),
            gap_size: config.gap_size.unwrap_or(8) as i32,
            pattern: CenterPattern::RightColumn,
        }
    }
    
    /// Calculate center layout for windows
    pub fn calculate(&self, workspace: WindowGeometry, window_count: usize) -> Vec<LayoutInfo> {
        if window_count == 0 {
            return Vec::new();
        }
        
        let mut layouts = Vec::with_capacity(window_count);
        let gap = self.gap_size;
        
        // Single window - centered
        if window_count == 1 {
            let center_width = (workspace.width as f32 * self.center_ratio) as i32;
            let center_height = (workspace.height as f32 * self.center_ratio) as i32;
            let x = workspace.x + (workspace.width - center_width) / 2;
            let y = workspace.y + (workspace.height - center_height) / 2;
            
            layouts.push(LayoutInfo {
                geometry: WindowGeometry {
                    x,
                    y,
                    width: center_width,
                    height: center_height,
                },
                is_focused: true,
                is_floating: false,
            });
            return layouts;
        }
        
        // Calculate center window geometry
        let center_width = (workspace.width as f32 * self.center_ratio) as i32;
        let center_height = (workspace.height as f32 * self.center_ratio) as i32;
        let center_x = workspace.x + (workspace.width - center_width) / 2;
        let center_y = workspace.y + (workspace.height - center_height) / 2;
        
        // Add center window
        layouts.push(LayoutInfo {
            geometry: WindowGeometry {
                x: center_x,
                y: center_y,
                width: center_width,
                height: center_height,
            },
            is_focused: false,
            is_floating: false,
        });
        
        // Position remaining windows based on pattern
        let stack_count = window_count - 1;
        
        match self.pattern {
            CenterPattern::RightColumn => {
                // Stack windows in a column on the right
                let stack_width = workspace.width - gap * 3 - center_width;
                let stack_x = center_x + center_width + gap * 2;
                let stack_height = (workspace.height - gap * 2 - gap * (stack_count - 1) as i32) / stack_count as i32;
                
                for i in 0..stack_count {
                    layouts.push(LayoutInfo {
                        geometry: WindowGeometry {
                            x: stack_x,
                            y: workspace.y + gap + (stack_height + gap) * i as i32,
                            width: stack_width,
                            height: stack_height,
                        },
                        is_focused: false,
                        is_floating: false,
                    });
                }
            }
            CenterPattern::LeftColumn => {
                // Stack windows in a column on the left
                let stack_width = center_x - gap * 2 - workspace.x;
                let stack_x = workspace.x + gap;
                let stack_height = (workspace.height - gap * 2 - gap * (stack_count - 1) as i32) / stack_count as i32;
                
                for i in 0..stack_count {
                    layouts.push(LayoutInfo {
                        geometry: WindowGeometry {
                            x: stack_x,
                            y: workspace.y + gap + (stack_height + gap) * i as i32,
                            width: stack_width,
                            height: stack_height,
                        },
                        is_focused: false,
                        is_floating: false,
                    });
                }
            }
            CenterPattern::TopBottom => {
                // Split stack windows top and bottom
                let top_count = (stack_count + 1) / 2;
                let bottom_count = stack_count / 2;
                let stack_width = workspace.width - gap * 2;
                
                // Top row
                let top_height = (workspace.height - center_height - gap * 3) / 2;
                for i in 0..top_count {
                    let col_width = (stack_width - gap * (top_count - 1) as i32) / top_count as i32;
                    layouts.push(LayoutInfo {
                        geometry: WindowGeometry {
                            x: workspace.x + gap + (col_width + gap) * i as i32,
                            y: workspace.y + gap,
                            width: col_width,
                            height: top_height,
                        },
                        is_focused: false,
                        is_floating: false,
                    });
                }
                
                // Bottom row
                for i in 0..bottom_count {
                    let col_width = (stack_width - gap * (bottom_count - 1) as i32) / bottom_count as i32;
                    layouts.push(LayoutInfo {
                        geometry: WindowGeometry {
                            x: workspace.x + gap + (col_width + gap) * i as i32,
                            y: center_y + center_height + gap * 2,
                            width: col_width,
                            height: top_height,
                        },
                        is_focused: false,
                        is_floating: false,
                    });
                }
            }
            CenterPattern::Grid => {
                // TODO: Implement grid pattern around center
                // For now, fall back to right column
                let stack_width = workspace.width - gap * 3 - center_width;
                let stack_x = center_x + center_width + gap * 2;
                let stack_height = (workspace.height - gap * 2 - gap * (stack_count - 1) as i32) / stack_count as i32;
                
                for i in 0..stack_count {
                    layouts.push(LayoutInfo {
                        geometry: WindowGeometry {
                            x: stack_x,
                            y: workspace.y + gap + (stack_height + gap) * i as i32,
                            width: stack_width,
                            height: stack_height,
                        },
                        is_focused: false,
                        is_floating: false,
                    });
                }
            }
        }
        
        layouts
    }
    
    /// Set layout pattern
    pub fn set_pattern(&mut self, pattern: CenterPattern) {
        self.pattern = pattern;
        debug!("Center pattern: {:?}", pattern);
    }
    
    /// Cycle to next pattern
    pub fn cycle_pattern(&mut self) {
        self.pattern = match self.pattern {
            CenterPattern::RightColumn => CenterPattern::LeftColumn,
            CenterPattern::LeftColumn => CenterPattern::TopBottom,
            CenterPattern::TopBottom => CenterPattern::Grid,
            CenterPattern::Grid => CenterPattern::RightColumn,
        };
        debug!("Center pattern: {:?}", self.pattern);
    }
}
