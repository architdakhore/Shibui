//! Workspace management module
//! 
//! Handles workspace creation, switching, and organization.
//! Similar to Hyprland/niri workspace system (not tags like MangoWM).

use log::{info, debug};
use std::collections::HashMap;
use crate::config::Config;
use crate::layout::{WindowGeometry, LayoutInfo};

/// Workspace manager
pub struct WorkspaceManager {
    /// All workspaces
    workspaces: HashMap<i32, Workspace>,
    /// Currently active workspace
    active_workspace: i32,
    /// Workspace count
    workspace_count: i32,
}

/// Individual workspace
pub struct Workspace {
    /// Workspace ID (1-based)
    pub id: i32,
    /// Workspace name
    pub name: String,
    /// Windows in this workspace
    pub windows: Vec<WindowId>,
    /// Window layouts
    pub layouts: HashMap<WindowId, LayoutInfo>,
    /// Whether workspace is visible
    pub visible: bool,
}

/// Window identifier
pub type WindowId = usize;

impl WorkspaceManager {
    /// Create a new workspace manager
    pub fn new(config: &Config) -> Self {
        info!("📊 Initializing workspace manager...");
        
        let workspace_count = config.workspace_count.unwrap_or(10) as i32;
        let mut workspaces = HashMap::new();
        
        // Create workspaces
        for i in 1..=workspace_count {
            workspaces.insert(i, Workspace::new(i));
        }
        
        let mut manager = Self {
            workspaces,
            active_workspace: 1,
            workspace_count,
        };
        
        // Mark initial workspace as visible
        manager.set_active_workspace(1);
        manager
    }
    
    /// Get active workspace
    pub fn active_workspace(&self) -> &Workspace {
        self.workspaces.get(&self.active_workspace).unwrap()
    }
    
    /// Get mutable active workspace
    pub fn active_workspace_mut(&mut self) -> &mut Workspace {
        self.workspaces.get_mut(&self.active_workspace).unwrap()
    }
    
    /// Set active workspace
    pub fn set_active_workspace(&mut self, id: i32) {
        if id < 1 || id > self.workspace_count {
            debug!("Invalid workspace ID: {}", id);
            return;
        }
        
        // Hide current workspace
        if let Some(current) = self.workspaces.get_mut(&self.active_workspace) {
            current.visible = false;
        }
        
        self.active_workspace = id;
        
        // Show new workspace
        if let Some(new) = self.workspaces.get_mut(&id) {
            new.visible = true;
        }
        
        info!("📊 Switched to workspace {}", id);
    }
    
    /// Move to next workspace
    pub fn next_workspace(&mut self) {
        let next = if self.active_workspace >= self.workspace_count {
            1
        } else {
            self.active_workspace + 1
        };
        self.set_active_workspace(next);
    }
    
    /// Move to previous workspace
    pub fn previous_workspace(&mut self) {
        let prev = if self.active_workspace <= 1 {
            self.workspace_count
        } else {
            self.active_workspace - 1
        };
        self.set_active_workspace(prev);
    }
    
    /// Add window to active workspace
    pub fn add_window(&mut self, window_id: WindowId) {
        debug!("Adding window {} to workspace {}", window_id, self.active_workspace);
        
        if let Some(workspace) = self.workspaces.get_mut(&self.active_workspace) {
            workspace.add_window(window_id);
        }
    }
    
    /// Remove window from all workspaces
    pub fn remove_window(&mut self, window_id: WindowId) {
        for workspace in self.workspaces.values_mut() {
            workspace.remove_window(window_id);
        }
    }
    
    /// Move window to specific workspace
    pub fn move_window_to_workspace(&mut self, window_id: WindowId, workspace_id: i32) {
        // Remove from current workspace
        self.remove_window(window_id);
        
        // Add to target workspace
        if let Some(workspace) = self.workspaces.get_mut(&workspace_id) {
            workspace.add_window(window_id);
            info!("📊 Moved window {} to workspace {}", window_id, workspace_id);
        }
    }
    
    /// Get all visible workspaces
    pub fn visible_workspaces(&self) -> Vec<&Workspace> {
        self.workspaces.values().filter(|w| w.visible).collect()
    }
    
    /// Get all workspaces
    pub fn all_workspaces(&self) -> Vec<&Workspace> {
        self.workspaces.values().collect()
    }
    
    /// Get workspace by ID
    pub fn get_workspace(&self, id: i32) -> Option<&Workspace> {
        self.workspaces.get(&id)
    }
    
    /// Rename workspace
    pub fn rename_workspace(&mut self, id: i32, name: String) {
        if let Some(workspace) = self.workspaces.get_mut(&id) {
            workspace.name = name;
            debug!("Renamed workspace {} to '{}'", id, workspace.name);
        }
    }
}

impl Workspace {
    /// Create a new workspace
    pub fn new(id: i32) -> Self {
        Self {
            id,
            name: format!("Workspace {}", id),
            windows: Vec::new(),
            layouts: HashMap::new(),
            visible: false,
        }
    }
    
    /// Add a window to workspace
    pub fn add_window(&mut self, window_id: WindowId) {
        if !self.windows.contains(&window_id) {
            self.windows.push(window_id);
        }
    }
    
    /// Remove a window from workspace
    pub fn remove_window(&mut self, window_id: WindowId) {
        self.windows.retain(|&id| id != window_id);
        self.layouts.remove(&window_id);
    }
    
    /// Check if workspace has windows
    pub fn has_windows(&self) -> bool {
        !self.windows.is_empty()
    }
    
    /// Get window count
    pub fn window_count(&self) -> usize {
        self.windows.len()
    }
}
