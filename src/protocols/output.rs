//! Output Management module
//! 
//! Handles monitor/output configuration, modes, and hotplugging.

use log::{info, debug};
use wayland_server::{
    protocol::wl_output::{self, WlOutput},
    Client, DisplayHandle, Resource,
};
use smithay::backend::allocator::{Format, Fourcc, Modifier, Size as AllocSize};
use smithay::backend::renderer::element::surface::WaylandSurfaceRenderElement;
use smithay::output::{Output, Mode, PhysicalProperties, Subpixel};
use smithay::reexports::wayland_protocols::wp::viewporter::server::wp_viewporter;

use crate::compositor::CompositorState;

/// Output manager
pub struct OutputManager {
    /// List of outputs
    pub outputs: Vec<OutputInfo>,
    /// Output manager global
    pub output_manager: Option<WlOutputManager>,
}

/// Output information
pub struct OutputInfo {
    /// Output handle
    pub output: Output,
    /// Wayland output
    pub wl_output: WlOutput,
    /// Name
    pub name: String,
    /// Description
    pub description: String,
    /// Physical size (mm)
    pub physical_size: (i32, i32),
    /// Current mode
    pub current_mode: Option<Mode>,
    /// Available modes
    pub modes: Vec<Mode>,
    /// Position in global space
    pub position: (i32, i32),
    /// Scale factor
    pub scale: i32,
    /// Transform (rotation)
    pub transform: Transform,
    /// Enabled state
    pub enabled: bool,
}

/// Output transform
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Transform {
    Normal,
    Rotate90,
    Rotate180,
    Rotate270,
    Flipped,
    FlippedRotate90,
    FlippedRotate180,
    FlippedRotate270,
}

/// Output manager global
pub struct WlOutputManager {
    /// Outputs
    outputs: Vec<WlOutput>,
}

impl OutputManager {
    /// Create new output manager
    pub fn new() -> Self {
        info!("🖥️ Initializing Output Manager...");
        
        Self {
            outputs: Vec::new(),
            output_manager: None,
        }
    }
    
    /// Add a new output
    pub fn add_output(
        &mut self,
        display: &DisplayHandle,
        name: String,
        description: String,
        physical_size: (i32, i32),
        modes: Vec<Mode>,
    ) -> &mut OutputInfo {
        info!("🖥️ Adding output: {} ({})", name, description);
        
        let output = Output::new(
            name.clone(),
            PhysicalProperties {
                size: physical_size.into(),
                subpixel: Subpixel::Unknown,
                maker: "Unknown".to_string(),
                model: description.clone(),
                serial: None,
            },
        );
        
        // Add modes
        for mode in &modes {
            output.add_mode(*mode);
        }
        
        // Create Wayland output
        let wl_output = output.create_global::<CompositorState>(display);
        
        // Set preferred mode
        if let Some(preferred) = modes.first() {
            output.change_current_state(Some(*preferred), None, None, None);
        }
        
        // Add to list
        self.outputs.push(OutputInfo {
            output: output.clone(),
            wl_output,
            name,
            description,
            physical_size,
            current_mode: modes.first().copied(),
            modes,
            position: (0, 0),
            scale: 1,
            transform: Transform::Normal,
            enabled: true,
        });
        
        let last = self.outputs.last_mut().unwrap();
        info!("✅ Output added: {}", last.name);
        last
    }
    
    /// Remove an output
    pub fn remove_output(&mut self, name: &str) {
        info!("🖥️ Removing output: {}", name);
        
        self.outputs.retain(|o| o.name != name);
        debug!("Output removed");
    }
    
    /// Get output by name
    pub fn get_output(&self, name: &str) -> Option<&OutputInfo> {
        self.outputs.iter().find(|o| o.name == name)
    }
    
    /// Get output by WL output
    pub fn get_output_by_wl(&self, wl_output: &WlOutput) -> Option<&OutputInfo> {
        self.outputs.iter().find(|o| &o.wl_output == wl_output)
    }
    
    /// Set output mode
    pub fn set_mode(&mut self, name: &str, mode: Mode) -> bool {
        if let Some(output) = self.outputs.iter_mut().find(|o| o.name == name) {
            info!("🖥️ Setting mode for {}: {}x{}@{}", 
                name, mode.size.w, mode.size.h, mode.refresh);
            
            output.output.change_current_state(Some(mode), None, None, None);
            output.current_mode = Some(mode);
            true
        } else {
            false
        }
    }
    
    /// Set output position
    pub fn set_position(&mut self, name: &str, x: i32, y: i32) -> bool {
        if let Some(output) = self.outputs.iter_mut().find(|o| o.name == name) {
            info!("🖥️ Setting position for {}: ({}, {})", name, x, y);
            
            output.position = (x, y);
            output.output.change_current_state(None, Some((x, y).into()), None, None);
            true
        } else {
            false
        }
    }
    
    /// Set output scale
    pub fn set_scale(&mut self, name: &str, scale: i32) -> bool {
        if let Some(output) = self.outputs.iter_mut().find(|o| o.name == name) {
            info!("🖥️ Setting scale for {}: {}x", name, scale);
            
            output.scale = scale;
            output.output.change_current_state(None, None, None, Some(scale));
            true
        } else {
            false
        }
    }
    
    /// Set output transform
    pub fn set_transform(&mut self, name: &str, transform: Transform) -> bool {
        if let Some(output) = self.outputs.iter_mut().find(|o| o.name == name) {
            info!("🖥️ Setting transform for {:?}", transform);
            
            output.transform = transform;
            true
        } else {
            false
        }
    }
    
    /// Enable/disable output
    pub fn set_enabled(&mut self, name: &str, enabled: bool) -> bool {
        if let Some(output) = self.outputs.iter_mut().find(|o| o.name == name) {
            info!("🖥️ {} output {}", if enabled { "Enabling" } else { "Disabling" }, name);
            
            output.enabled = enabled;
            true
        } else {
            false
        }
    }
    
    /// Get all enabled outputs
    pub fn enabled_outputs(&self) -> Vec<&OutputInfo> {
        self.outputs.iter().filter(|o| o.enabled).collect()
    }
    
    /// Get primary output (first enabled)
    pub fn primary_output(&self) -> Option<&OutputInfo> {
        self.enabled_outputs().first().copied()
    }
}

/// Output configuration
#[derive(Debug, Clone)]
pub struct OutputConfig {
    /// Output name
    pub name: String,
    /// Enable state
    pub enabled: bool,
    /// Mode (width, height, refresh)
    pub mode: Option<(i32, i32, i32)>,
    /// Position
    pub position: Option<(i32, i32)>,
    /// Scale
    pub scale: Option<f32>,
    /// Transform
    pub transform: Option<Transform>,
}

impl OutputConfig {
    /// Parse from string (e.g., "HDMI-A-1: 1920x1080@60")
    pub fn parse(config: &str) -> Result<Self, String> {
        let parts: Vec<&str> = config.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid output config format".to_string());
        }
        
        let name = parts[0].trim().to_string();
        let settings = parts[1].trim();
        
        let mut output = OutputConfig {
            name,
            enabled: true,
            mode: None,
            position: None,
            scale: None,
            transform: None,
        };
        
        // Parse settings
        for setting in settings.split(',') {
            let setting = setting.trim();
            
            if setting == "disable" {
                output.enabled = false;
            } else if setting.contains('x') {
                // Mode: 1920x1080@60
                let mode_parts: Vec<&str> = setting.split('@').collect();
                let res: Vec<&str> = mode_parts[0].split('x').collect();
                
                if res.len() == 2 {
                    let width = res[0].parse::<i32>()
                        .map_err(|_| "Invalid width")?;
                    let height = res[1].parse::<i32>()
                        .map_err(|_| "Invalid height")?;
                    let refresh = mode_parts.get(1)
                        .and_then(|r| r.parse::<i32>().ok())
                        .unwrap_or(60);
                    
                    output.mode = Some((width, height, refresh));
                }
            } else if setting.contains("pos") {
                // Position: pos 100,200
                let pos_parts: Vec<&str> = setting.split_whitespace().collect();
                if pos_parts.len() >= 3 {
                    let x = pos_parts[1].parse::<i32>().map_err(|_| "Invalid X")?;
                    let y = pos_parts[2].parse::<i32>().map_err(|_| "Invalid Y")?;
                    output.position = Some((x, y));
                }
            } else if setting.contains("scale") {
                // Scale: scale 1.5
                let scale_parts: Vec<&str> = setting.split_whitespace().collect();
                if scale_parts.len() >= 2 {
                    let scale = scale_parts[1].parse::<f32>().map_err(|_| "Invalid scale")?;
                    output.scale = Some(scale);
                }
            }
        }
        
        Ok(output)
    }
}
