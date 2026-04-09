//! Core compositor module
//! 
//! This module contains the main Wayland compositor implementation,
//! handling the Wayland server, client connections, and event loop.

use anyhow::{Result, Context};
use log::{info, debug, error};
use wayland_server::{Display, DisplayHandle};
use smithay::backend::renderer::gles::GlesRenderer;
use smithay::reexports::wayland_server::prelude::*;

use crate::workspace::WorkspaceManager;
use crate::input::InputHandler;
use crate::render::Renderer;
use crate::config::Config;
use crate::layout::LayoutManager;
use crate::overview::Overview;
use crate::animations::AnimationManager;
use crate::backend::{BackendManager, BackendType};
use crate::protocols::{
    xdg_shell::XdgWmHandler,
    layer_shell::LayerShellHandler,
    output::OutputManager,
    input_method::InputMethodManager,
    virtual_keyboard::VirtualKeyboardManager,
    screencopy::ScreenCopyManager,
};

/// Main compositor state
pub struct Compositor {
    /// Wayland display server
    display: Display<CompositorState>,
    /// Display handle for creating globals
    display_handle: DisplayHandle,
    /// Compositor state
    state: CompositorState,
    /// Configuration
    config: Config,
}

/// Compositor state shared across modules
pub struct CompositorState {
    /// Workspace manager
    pub workspace_manager: WorkspaceManager,
    /// Input handler
    pub input_handler: InputHandler,
    /// Renderer
    pub renderer: Option<Renderer>,
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
            config: config.clone(),
        };
        
        info!("✅ Compositor state initialized");
        
        Ok(Self {
            display,
            display_handle,
            state,
            config,
        })
    }
    
    /// Run the compositor main loop
    pub fn run(&mut self) -> Result<()> {
        info!("🚀 Starting compositor event loop...");
        
        // TODO: Initialize backend (DRM/KMS, Winit for testing)
        // TODO: Set up rendering
        // TODO: Initialize input devices
        // TODO: Start event loop
        
        // Placeholder for main loop
        loop {
            // Dispatch Wayland events
            self.display.dispatch_clients(&mut self.state)?;
            
            // Flush events to clients
            self.display.flush_clients()?;
            
            // TODO: Handle input events
            // TODO: Render frames
            // TODO: Handle timeouts
            
            // For now, just break after one iteration (placeholder)
            break;
        }
        
        Ok(())
    }
    
    /// Get compositor name
    pub fn name(&self) -> &str {
        "ShibUI"
    }
    
    /// Get compositor version
    pub fn version(&self) -> &str {
        crate::VERSION
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
