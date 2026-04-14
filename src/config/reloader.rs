//! Configuration hot reloader module
//! 
//! Watches config file for changes and reloads automatically.
//! Similar to niri and Hyprland's live config reloading.

use anyhow::{Result, Context};
use log::{info, debug, error, warn};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::config::Config;

/// Config reloader that watches for file changes
pub struct ConfigReloader {
    /// Path to config file
    config_path: PathBuf,
    /// Current config (shared state)
    current_config: Arc<Mutex<Config>>,
    /// File watcher
    watcher: RecommendedWatcher,
    /// Whether reloader is active
    active: bool,
}

impl ConfigReloader {
    /// Create a new config reloader
    pub fn new(config_path: PathBuf, initial_config: Config) -> Result<Self> {
        info!("🔄 Initializing config reloader...");
        
        let config_arc = Arc::new(Mutex::new(initial_config));
        let config_clone = Arc::clone(&config_arc);
        
        // Create file watcher
        let watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                if let Ok(event) = res {
                    // Handle file change events
                    if let EventKind::Modify(_) | EventKind::Create(_) = event.kind {
                        for path in event.paths {
                            if path.extension().map_or(false, |ext| ext == "toml") {
                                debug!("📝 Config file changed: {:?}", path);
                                // Config will be reloaded by compositor
                            }
                        }
                    }
                }
            },
            notify::Config::default(),
        ).context("Failed to create file watcher")?;
        
        Ok(Self {
            config_path,
            current_config: config_arc,
            watcher,
            active: false,
        })
    }
    
    /// Start watching config file
    pub fn start(&mut self) -> Result<()> {
        info!("👁️ Starting config file watcher...");
        
        // Watch config file
        self.watcher.watch(
            &self.config_path,
            RecursiveMode::NonRecursive,
        ).context("Failed to watch config file")?;
        
        // Also watch config directory for new files
        if let Some(parent) = self.config_path.parent() {
            self.watcher.watch(
                parent,
                RecursiveMode::NonRecursive,
            ).ok(); // Non-fatal if directory watch fails
        }
        
        self.active = true;
        info!("✅ Config reloader active");
        info!("   Watching: {:?}", self.config_path);
        info!("   Changes will be applied automatically");
        
        Ok(())
    }
    
    /// Reload config from file
    pub fn reload_config(&self) -> Result<Config> {
        debug!("🔄 Reloading config from file...");
        
        // Try to load new config
        let new_config = Config::load_from_path(&self.config_path)
            .context("Failed to load config file")?;
        
        // Validate new config
        if let Err(e) = self.validate_config(&new_config) {
            error!("❌ Config validation failed: {}", e);
            // Return error so compositor can display it
            return Err(e);
        }
        
        // Update shared config
        let mut config_guard = self.current_config.lock()
            .context("Failed to acquire config lock")?;
        *config_guard = new_config.clone();
        
        info!("✅ Config reloaded successfully");
        
        Ok(new_config)
    }
    
    /// Validate config before applying
    fn validate_config(&self, config: &Config) -> Result<()> {
        // Validate general settings
        if config.general.name.is_empty() {
            anyhow::bail!("Config error: general.name cannot be empty");
        }
        
        // Validate layout mode
        let valid_modes = ["dwindle", "horizontal", "vertical", "center", "floating"];
        if !valid_modes.contains(&config.general.layout_mode.as_str()) {
            anyhow::bail!(
                "Config error: invalid layout_mode '{}'. Valid values: {:?}",
                config.general.layout_mode,
                valid_modes
            );
        }
        
        // Validate animation duration
        if config.animations.duration_ms > 5000 {
            warn!("Config warning: animation duration {}ms is very long", config.animations.duration_ms);
        }
        
        // Validate colors (basic check)
        if !config.tiling.border_color.starts_with('#') {
            anyhow::bail!("Config error: border_color must be hex color (start with #)");
        }
        
        if !config.tiling.active_border_color.starts_with('#') {
            anyhow::bail!("Config error: active_border_color must be hex color (start with #)");
        }
        
        // Validate gap size
        if let Some(gap) = config.tiling.gap_size {
            if gap > 100 {
                warn!("Config warning: gap_size {} is very large", gap);
            }
        }
        
        // Validate workspace count
        if let Some(count) = config.workspaces.count {
            if count < 1 || count > 100 {
                anyhow::bail!("Config error: workspace count must be between 1 and 100");
            }
        }
        
        Ok(())
    }
    
    /// Get current config (thread-safe)
    pub fn get_config(&self) -> Result<Config> {
        let config_guard = self.current_config.lock()
            .context("Failed to acquire config lock")?;
        Ok(config_guard.clone())
    }
    
    /// Stop watching
    pub fn stop(&mut self) {
        info!("⏹️ Stopping config reloader");
        self.watcher.unwatch(&self.config_path).ok();
        self.active = false;
    }
    
    /// Check if reloader is active
    pub fn is_active(&self) -> bool {
        self.active
    }
}

/// Config reload result
#[derive(Debug, Clone)]
pub struct ConfigReloadResult {
    /// Whether reload was successful
    pub success: bool,
    /// Error message if failed
    pub error_message: Option<String>,
    /// New config if successful
    pub new_config: Option<Config>,
}

impl ConfigReloadResult {
    /// Create success result
    pub fn success(config: Config) -> Self {
        Self {
            success: true,
            error_message: None,
            new_config: Some(config),
        }
    }
    
    /// Create error result
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            error_message: Some(message),
            new_config: None,
        }
    }
}
