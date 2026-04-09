//! Overview mode module
//! 
//! niri-style workspace overview with dual-axis scrolling support.
//! Shows all workspaces and windows in a scaled view.

use log::{debug, info};
use crate::workspace::WorkspaceManager;
use crate::layout::WindowGeometry;

/// Overview mode manager
pub struct Overview {
    /// Whether overview is active
    active: bool,
    /// Scale factor for overview (windows scaled down)
    scale: f32,
    /// Gap between workspaces in overview
    gap: i32,
    /// Scroll offset for horizontal scrolling
    scroll_x: i32,
    /// Scroll offset for vertical scrolling
    scroll_y: i32,
    /// Target scroll X for smooth animation
    target_scroll_x: i32,
    /// Target scroll Y for smooth animation
    target_scroll_y: i32,
    /// Scroll animation progress
    scroll_progress: f32,
    /// Blur enabled
    blur: bool,
}

/// Overview workspace representation
pub struct OverviewWorkspace {
    /// Workspace ID
    pub id: i32,
    /// Workspace name
    pub name: String,
    /// Position in overview
    pub x: i32,
    pub y: i32,
    /// Scaled size
    pub width: i32,
    pub height: i32,
    /// Number of windows
    pub window_count: usize,
    /// Whether this is the active workspace
    pub is_active: bool,
}

impl Overview {
    /// Create new overview
    pub fn new() -> Self {
        Self {
            active: false,
            scale: 0.15, // 15% scale by default
            gap: 20,
            scroll_x: 0,
            scroll_y: 0,
            target_scroll_x: 0,
            target_scroll_y: 0,
            scroll_progress: 1.0,
            blur: true,
        }
    }
    
    /// Toggle overview
    pub fn toggle(&mut self) {
        self.active = !self.active;
        if self.active {
            info!("👁️ Overview activated");
        } else {
            info!("👁️ Overview deactivated");
        }
    }
    
    /// Activate overview
    pub fn activate(&mut self) {
        if !self.active {
            self.active = true;
            info!("👁️ Overview activated");
        }
    }
    
    /// Deactivate overview
    pub fn deactivate(&mut self) {
        if self.active {
            self.active = false;
            info!("👁️ Overview deactivated");
        }
    }
    
    /// Check if overview is active
    pub fn is_active(&self) -> bool {
        self.active
    }
    
    /// Calculate overview layout
    pub fn calculate_layout(&self, workspace_manager: &WorkspaceManager, screen_geometry: WindowGeometry) -> Vec<OverviewWorkspace> {
        let mut overview_workspaces = Vec::new();
        
        let all_workspaces = workspace_manager.all_workspaces();
        let active_id = workspace_manager.active_workspace().id;
        
        // Calculate grid layout (5x2 for 10 workspaces)
        let workspaces_per_row = 5;
        let total_rows = (all_workspaces.len() + workspaces_per_row - 1) / workspaces_per_row;
        
        let workspace_width = ((screen_geometry.width - self.gap * (workspaces_per_row + 1) as i32) / workspaces_per_row as i32)
            .min(400); // Max width
        let workspace_height = ((screen_geometry.height - self.gap * (total_rows + 1) as i32) / total_rows as i32)
            .min(300); // Max height
        
        for (index, workspace) in all_workspaces.iter().enumerate() {
            let row = index / workspaces_per_row;
            let col = index % workspaces_per_row;
            
            let x = screen_geometry.x + self.gap + (workspace_width + self.gap) * col as i32 - self.scroll_x;
            let y = screen_geometry.y + self.gap + (workspace_height + self.gap) * row as i32 - self.scroll_y;
            
            overview_workspaces.push(OverviewWorkspace {
                id: workspace.id,
                name: workspace.name.clone(),
                x,
                y,
                width: workspace_width,
                height: workspace_height,
                window_count: workspace.window_count(),
                is_active: workspace.id == active_id,
            });
        }
        
        overview_workspaces
    }
    
    /// Scroll horizontally
    pub fn scroll_horizontal(&mut self, delta: i32, max_scroll: i32) {
        self.target_scroll_x = (self.target_scroll_x + delta).clamp(0, max_scroll);
        self.scroll_progress = 0.0;
        debug!("Overview scroll X: {}", self.target_scroll_x);
    }
    
    /// Scroll vertically
    pub fn scroll_vertical(&mut self, delta: i32, max_scroll: i32) {
        self.target_scroll_y = (self.target_scroll_y + delta).clamp(0, max_scroll);
        self.scroll_progress = 0.0;
        debug!("Overview scroll Y: {}", self.target_scroll_y);
    }
    
    /// Update scroll animation
    pub fn update_scroll(&mut self, delta_time: f32) {
        if self.scroll_progress < 1.0 {
            self.scroll_progress += delta_time * 5.0;
            self.scroll_progress = self.scroll_progress.min(1.0);
            
            let progress = self.ease_out_cubic(self.scroll_progress);
            
            self.scroll_x = self.target_scroll_x + ((self.scroll_x - self.target_scroll_x) as f32 * (1.0 - progress)) as i32;
            self.scroll_y = self.target_scroll_y + ((self.scroll_y - self.target_scroll_y) as f32 * (1.0 - progress)) as i32;
        }
    }
    
    /// Ease-out cubic interpolation
    fn ease_out_cubic(&self, x: f32) -> f32 {
        1.0 - (1.0 - x).powi(3)
    }
    
    /// Set scale factor
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale.clamp(0.1, 0.5);
        debug!("Overview scale: {}", self.scale);
    }
    
    /// Toggle blur
    pub fn toggle_blur(&mut self) {
        self.blur = !self.blur;
        debug!("Overview blur: {}", self.blur);
    }
    
    /// Get current scale
    pub fn scale(&self) -> f32 {
        self.scale
    }
    
    /// Get blur enabled
    pub fn blur_enabled(&self) -> bool {
        self.blur
    }
}
