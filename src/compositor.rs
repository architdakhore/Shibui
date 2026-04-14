//! Core compositor module
//! 
//! This module contains the main Wayland compositor implementation,
//! handling the Wayland server, client connections, and event loop.

use anyhow::{Result, Context};
use log::{info, debug, error, warn};
use wayland_server::{Display, DisplayHandle};
use smithay::reexports::wayland_server::prelude::*;

use crate::workspace::WorkspaceManager;
use crate::input::InputHandler;
use crate::render::Renderer;
use crate::config::{Config, ConfigReloader};
use crate::layout::LayoutManager;
use crate::overview::Overview;
use crate::animations::AnimationManager;
use crate::backend::BackendManager;
use crate::protocols::{
    xdg_shell::XdgWmHandler,
    layer_shell::LayerShellHandler,
    output::OutputManager,
    input_method::InputMethodManager,
    virtual_keyboard::VirtualKeyboardManager,
    screencopy::ScreenCopyManager,
};
use crate::render::error_overlay::ErrorOverlay;

/// Compositor state shared across all modules
pub struct Compositor {
    /// Wayland display server
    display: Display<CompositorState>,
    /// Display handle for creating globals
    display_handle: DisplayHandle,
    /// Compositor state
    state: CompositorState,
    /// Configuration
    config: Config,
    /// Config reloader for hot reload
    config_reloader: Option<ConfigReloader>,
    /// Error overlay for displaying config errors
    error_overlay: ErrorOverlay,
    /// IPC server
    ipc_server: Option<crate::ipc::IpcServer>,
}

/// Compositor state shared across modules
pub struct CompositorState {
    /// Workspace manager
    pub workspace_manager: WorkspaceManager,
    /// Input handler
    pub input_handler: InputHandler,
    /// Renderer
    pub renderer: Option<Box<dyn Renderer>>,
    /// Layout manager
    pub layout_manager: LayoutManager,
    /// Overview mode
    pub overview: Overview,
    /// Animation manager
    pub animation_manager: AnimationManager,
    /// Backend manager
    pub backend_manager: BackendManager,
    /// Configuration
    pub config: Config,
    
    // Protocol handlers
    pub xdg_wm_handler: XdgWmHandler,
    pub layer_shell_handler: LayerShellHandler,
    pub output_manager: OutputManager,
    pub input_method_manager: InputMethodManager,
    pub virtual_keyboard_manager: VirtualKeyboardManager,
    pub screen_copy_manager: ScreenCopyManager,
}

impl Compositor {
    /// Create a new compositor instance
    pub fn new() -> Result<Self> {
        info!("🔧 Initializing compositor...");
        
        // Load configuration
        let config = Config::load().context("Failed to load configuration")?;
        info!("✅ Configuration loaded");
        
        // Create Wayland display
        let display = Display::new().context("Failed to create Wayland display")?;
        let display_handle = display.handle();
        info!("✅ Wayland display created");
        
        // Initialize compositor state
        let state = CompositorState {
            workspace_manager: WorkspaceManager::new(&config),
            input_handler: InputHandler::new()?,
            renderer: None, // Will be initialized after backend setup
            layout_manager: LayoutManager::new(&config),
            overview: Overview::new(),
            animation_manager: AnimationManager::new(),
            backend_manager: BackendManager::new(&config)?,
            config: config.clone(),
            xdg_wm_handler: XdgWmHandler::new(),
            layer_shell_handler: LayerShellHandler::new(),
            output_manager: OutputManager::new(),
            input_method_manager: InputMethodManager::new(),
            virtual_keyboard_manager: VirtualKeyboardManager::new(),
            screen_copy_manager: ScreenCopyManager::new(),
        };
        
        info!("✅ Compositor state initialized");
        
        // Initialize config reloader
        let config_path = Config::get_config_path();
        let config_reloader = ConfigReloader::new(config_path.clone(), config.clone())
            .context("Failed to create config reloader")?;
        
        info!("✅ Config reloader created");
        
        Ok(Self {
            display,
            display_handle,
            state,
            config,
            config_reloader: Some(config_reloader),
            error_overlay: ErrorOverlay::new(),
            ipc_server: None,  // Will be initialized in run()
        })
    }
    
    /// Run the compositor main loop
    pub fn run(&mut self) -> Result<()> {
        info!("🚀 Starting Shibui compositor event loop...");
        
        // Initialize IPC server
        let mut ipc_server = crate::ipc::IpcServer::new()?;
        ipc_server.start()?;
        self.ipc_server = Some(ipc_server);
        
        // Initialize backend (DRM/KMS, Winit for testing)
        self.state.backend_manager.initialize()?;
        
        // Start config reloader
        if let Some(ref mut reloader) = self.config_reloader {
            if let Err(e) = reloader.start() {
                error!("⚠️  Config reloader failed to start: {}", e);
                self.error_overlay.show_warning(
                    &format!("Config watcher failed: {}. Config changes won't auto-reload.", e)
                );
            } else {
                info!("✅ Config reloader started - changes will apply automatically");
            }
        }
        
        // Set up rendering
        // TODO: Initialize renderer with backend surface
        
        // Initialize input devices
        // TODO: Initialize input devices from backend
        
        // Main event loop - integrate with Smithay's calloop event loop
        // Proper event loop that continues until shutdown signal
        info!("🔄 Entering main event loop");
        loop {
            // Check for shutdown signal
            if crate::SHUTDOWN_FLAG.load(std::sync::atomic::Ordering::Relaxed) {
                info!("⏹️  Shutdown signal received, exiting event loop");
                break;
            }
            
            // Dispatch Wayland events from clients
            // This processes pending client requests
            match self.display.dispatch_clients(&mut self.state) {
                Ok(_) => {},
                Err(e) => {
                    if !crate::SHUTDOWN_FLAG.load(std::sync::atomic::Ordering::Relaxed) {
                        error!("Dispatch failed: {}", e);
                    }
                    break;
                }
            }
            
            // Update animations (16ms = ~60 FPS)
            self.state.animation_manager.update(0.016);
            
            // Update error overlay
            self.error_overlay.update();
            
            // Check for config reload events
            if let Some(ref mut reloader) = self.config_reloader {
                // Try to reload config if file changed
                match reloader.reload_config() {
                    Ok(new_config) => {
                        // Apply new config
                        self.apply_config(&new_config);
                        self.error_overlay.show_info("✅ Config reloaded successfully");
                    }
                    Err(e) => {
                        // Show error overlay
                        let error_msg = format!("❌ Config reload failed: {}", e);
                        error!("{}", error_msg);
                        self.error_overlay.show_error(&error_msg);
                    }
                }
            }
            
            // Process IPC connections
            if let Some(ref ipc_server) = self.ipc_server {
                if let Err(e) = ipc_server.process() {
                    warn!("IPC processing error: {}", e);
                }
            }
            
            // Render frames
            // TODO: Render current state via renderer
            // TODO: Render error overlay if visible
            
            // Flush events to all clients to ensure they receive updates
            match self.display.flush_clients() {
                Ok(_) => {},
                Err(e) => {
                    if !crate::SHUTDOWN_FLAG.load(std::sync::atomic::Ordering::Relaxed) {
                        error!("Flush failed: {}", e);
                    }
                    break;
                }
            }
            
            // Poll for input events
            // TODO: Handle input events from input devices
            
            // Small sleep to prevent busy-waiting
            // TODO: Replace with proper calloop event loop integration
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
        
        info!("✅ Event loop exited cleanly");
        Ok(())
    }
    
    /// Apply new configuration
    fn apply_config(&mut self, new_config: &Config) {
        info!("🔄 Applying new configuration...");
        
        // Update compositor config
        self.config = new_config.clone();
        
        // Update state config
        self.state.config = new_config.clone();
        
        // Apply to relevant managers
        self.state.workspace_manager.apply_config(new_config);
        self.state.layout_manager.apply_config(new_config);
        self.state.animation_manager.apply_config(new_config);
        
        debug!("✅ New configuration applied");
    }
}

/// Graceful shutdown and resource cleanup
impl Drop for Compositor {
    fn drop(&mut self) {
        info!("🧹 Cleaning up compositor resources...");
        
        // Shutdown IPC server
        if let Some(mut ipc_server) = self.ipc_server.take() {
            ipc_server.stop();
            info!("✅ IPC server stopped");
        }
        
        // Shutdown backend
        // self.state.backend_manager.shutdown();
        
        // Flush any remaining client events
        if let Err(e) = self.display.flush_clients() {
            warn!("Error flushing clients during shutdown: {}", e);
        }
        
        info!("✅ Compositor cleanup complete");
    }
}

// Socket management for Wayland clients
impl Compositor {
    /// Add Wayland socket
    pub fn add_socket(&mut self) -> Result<String> {
        let socket_name = self.display.add_socket_auto()?;
        info!("🔌 Wayland socket created: {}", socket_name);
        Ok(socket_name.to_string_lossy().to_string())
    }
}
