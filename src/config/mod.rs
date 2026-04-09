//! Configuration module
//! 
//! Handles loading and managing compositor settings.
//! Quicksell-compatible configuration system.

use log::{info, debug};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// General settings
    #[serde(default)]
    pub general: GeneralConfig,
    
    /// Tiling settings
    #[serde(default)]
    pub tiling: TilingConfig,
    
    /// Workspace settings
    #[serde(default)]
    pub workspaces: WorkspaceConfig,
    
    /// Animation settings
    #[serde(default)]
    pub animations: AnimationConfig,
    
    /// Overview settings
    #[serde(default)]
    pub overview: OverviewConfig,
    
    /// Input settings
    #[serde(default)]
    pub input: InputConfig,
    
    /// Keybindings
    #[serde(default)]
    pub keybindings: KeybindingsConfig,
    
    /// Render settings
    #[serde(default)]
    pub render: RenderConfig,
}

/// General configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Compositor name
    pub name: String,
    /// Layout mode
    pub layout_mode: String,
    /// Log level
    pub log_level: String,
}

/// Tiling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TilingConfig {
    /// Gap between windows (pixels)
    pub gap_size: Option<u32>,
    /// Border width (pixels)
    pub border_width: Option<u32>,
    /// Border color (hex)
    pub border_color: String,
    /// Active border color (hex)
    pub active_border_color: String,
    /// Master area ratio (0.0 - 1.0)
    pub master_ratio: Option<f32>,
    /// Center window ratio (0.0 - 1.0)
    pub center_ratio: Option<f32>,
    /// Window width for horizontal layout
    pub window_width: Option<u32>,
    /// Window height for vertical layout
    pub window_height: Option<u32>,
}

/// Workspace configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    /// Number of workspaces
    pub count: Option<u32>,
    /// Scroll mode
    pub scroll_mode: String,
}

/// Animation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationConfig {
    /// Enable animations
    pub enabled: bool,
    /// Animation duration (milliseconds)
    pub duration_ms: u32,
    /// Animation curve
    pub curve: String,
}

/// Overview configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverviewConfig {
    /// Overview scale
    pub scale: f32,
    /// Gap between workspaces
    pub gap: u32,
    /// Enable blur effect
    pub blur: bool,
    /// Scroll direction
    pub scroll_direction: String,
}

/// Input configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputConfig {
    /// Follow mouse focus
    pub follow_mouse: bool,
    /// Natural scrolling
    pub natural_scroll: bool,
    /// Repeat rate (keys per second)
    pub repeat_rate: u32,
    /// Repeat delay (milliseconds)
    pub repeat_delay: u32,
}

/// Keybindings configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeybindingsConfig {
    /// Modifier key
    pub mod_key: String,
    /// Keybinding mappings
    pub bindings: std::collections::HashMap<String, String>,
}

/// Render configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderConfig {
    /// Enable VSync
    pub vsync: bool,
    /// Enable HDR
    pub hdr: bool,
    /// Render backend
    pub backend: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            tiling: TilingConfig::default(),
            workspaces: WorkspaceConfig::default(),
            animations: AnimationConfig::default(),
            overview: OverviewConfig::default(),
            input: InputConfig::default(),
            keybindings: KeybindingsConfig::default(),
            render: RenderConfig::default(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            name: "ShibUI".to_string(),
            layout_mode: "dynamic".to_string(),
            log_level: "info".to_string(),
        }
    }
}

impl Default for TilingConfig {
    fn default() -> Self {
        Self {
            gap_size: Some(8),
            border_width: Some(2),
            border_color: "#4C7899".to_string(),
            active_border_color: "#57A0C9".to_string(),
            master_ratio: Some(0.55),
            center_ratio: Some(0.6),
            window_width: Some(800),
            window_height: Some(600),
        }
    }
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            count: Some(10),
            scroll_mode: "both".to_string(),
        }
    }
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            duration_ms: 250,
            curve: "ease-out-expo".to_string(),
        }
    }
}

impl Default for OverviewConfig {
    fn default() -> Self {
        Self {
            scale: 0.15,
            gap: 20,
            blur: true,
            scroll_direction: "both".to_string(),
        }
    }
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            follow_mouse: true,
            natural_scroll: true,
            repeat_rate: 25,
            repeat_delay: 300,
        }
    }
}

impl Default for KeybindingsConfig {
    fn default() -> Self {
        let mut bindings = std::collections::HashMap::new();
        
        // Default keybindings
        bindings.insert("Mod+Return".to_string(), "spawn terminal".to_string());
        bindings.insert("Mod+D".to_string(), "spawn launcher".to_string());
        bindings.insert("Mod+Tab".to_string(), "toggle overview".to_string());
        bindings.insert("Mod+H".to_string(), "focus left".to_string());
        bindings.insert("Mod+J".to_string(), "focus down".to_string());
        bindings.insert("Mod+K".to_string(), "focus up".to_string());
        bindings.insert("Mod+L".to_string(), "focus right".to_string());
        bindings.insert("Mod+Shift+Q".to_string(), "close window".to_string());
        bindings.insert("Mod+1".to_string(), "workspace 1".to_string());
        bindings.insert("Mod+2".to_string(), "workspace 2".to_string());
        bindings.insert("Mod+3".to_string(), "workspace 3".to_string());
        
        Self {
            mod_key: "SUPER".to_string(),
            bindings,
        }
    }
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            vsync: true,
            hdr: false,
            backend: "opengl".to_string(),
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();
        
        if config_path.exists() {
            info!("📄 Loading config from {:?}", config_path);
            let content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            info!("📄 Config file not found, using defaults");
            Ok(Config::default())
        }
    }
    
    /// Save configuration to file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();
        
        // Create directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        info!("💾 Config saved to {:?}", config_path);
        Ok(())
    }
    
    /// Get configuration file path
    fn get_config_path() -> PathBuf {
        // Check environment variable first
        if let Ok(path) = std::env::var("FLOWWM_CONFIG") {
            return PathBuf::from(path);
        }
        
        // Default to XDG config directory
        if let Ok(home) = std::env::var("HOME") {
            let xdg_config = std::env::var("XDG_CONFIG_HOME")
                .unwrap_or_else(|_| format!("{}/.config", home));
            return PathBuf::from(xdg_config).join("flowwm/config.toml");
        }
        
        // Fallback
        PathBuf::from("./flowwm.toml")
    }
}

// Convenience accessors
impl Config {
    pub fn layout_mode(&self) -> &str {
        &self.general.layout_mode
    }
    
    pub fn gap_size(&self) -> u32 {
        self.tiling.gap_size.unwrap_or(8)
    }
    
    pub fn master_ratio(&self) -> f32 {
        self.tiling.master_ratio.unwrap_or(0.55)
    }
    
    pub fn center_ratio(&self) -> f32 {
        self.tiling.center_ratio.unwrap_or(0.6)
    }
    
    pub fn window_width(&self) -> u32 {
        self.tiling.window_width.unwrap_or(800)
    }
    
    pub fn window_height(&self) -> u32 {
        self.tiling.window_height.unwrap_or(600)
    }
    
    pub fn workspace_count(&self) -> u32 {
        self.workspaces.count.unwrap_or(10)
    }
}
