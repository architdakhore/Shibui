//! Layer Shell module
//! 
//! Implements the wlr-layer-shell protocol for panels, overlays, and desktop widgets.
//! Used by status bars, launchers, and notifications.

use log::{info, debug};
use wayland_server::{
    protocol::wl_surface::WlSurface,
    Client, DisplayHandle, Resource,
};
use smithay::reexports::wayland_protocols_wlr::layer::server::{
    wlr_layer_shell_v1::{self, WlrLayerShellV1},
    zwlr_layer_surface_v1::{self, ZwlrLayerSurfaceV1},
};

use crate::compositor::CompositorState;

/// Layer types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayerType {
    /// Background layer (wallpapers)
    Background,
    /// Bottom layer (below windows)
    Bottom,
    /// Top layer (above windows, below fullscreen)
    Top,
    /// Overlay layer (notifications, OSD)
    Overlay,
}

/// Layer surface
pub struct LayerSurface {
    /// Layer type
    pub layer: LayerType,
    /// Wayland surface
    pub surface: WlSurface,
    /// Layer surface
    pub layer_surface: ZwlrLayerSurfaceV1,
    /// Namespace (for grouping)
    pub namespace: String,
    /// Anchor points
    pub anchor: Anchor,
    /// Exclusive zone
    pub exclusive_zone: i32,
    /// Size
    pub width: u32,
    pub height: u32,
}

/// Anchor points for layer surfaces
#[derive(Debug, Clone, Copy)]
pub struct Anchor {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

impl Anchor {
    /// Create anchor from bitfield
    pub fn from_bits(bits: u32) -> Self {
        Self {
            top: bits & zwlr_layer_surface_v1::Anchor::Top.bits() != 0,
            bottom: bits & zwlr_layer_surface_v1::Anchor::Bottom.bits() != 0,
            left: bits & zwlr_layer_surface_v1::Anchor::Left.bits() != 0,
            right: bits & zwlr_layer_surface_v1::Anchor::Right.bits() != 0,
        }
    }
}

/// Layer shell handler
pub struct LayerShellHandler {
    /// Layer shell global
    pub layer_shell: WlrLayerShellV1,
    /// Active layer surfaces
    pub surfaces: Vec<LayerSurface>,
}

impl LayerShellHandler {
    /// Create new layer shell handler
    pub fn new(display: &DisplayHandle) -> Result<Self, Box<dyn std::error::Error>> {
        info!("📑 Initializing Layer Shell protocol...");
        
        let layer_shell = WlrLayerShellV1::new::<CompositorState>();
        display.create_global::<WlrLayerShellV1>(4, layer_shell.clone());
        
        info!("✅ Layer Shell protocol initialized");
        
        Ok(Self {
            layer_shell,
            surfaces: Vec::new(),
        })
    }
    
    /// Handle new layer surface creation
    pub fn handle_new_surface(
        &mut self,
        surface: WlSurface,
        layer_surface: ZwlrLayerSurfaceV1,
        layer: u32,
        namespace: String,
    ) {
        debug!("New layer surface created: {}", namespace);
        
        let layer_type = match layer {
            0 => LayerType::Background,
            1 => LayerType::Bottom,
            2 => LayerType::Top,
            _ => LayerType::Overlay,
        };
        
        let ls = LayerSurface {
            layer: layer_type,
            surface,
            layer_surface,
            namespace,
            anchor: Anchor {
                top: false,
                bottom: false,
                left: false,
                right: false,
            },
            exclusive_zone: 0,
            width: 0,
            height: 0,
        };
        
        self.surfaces.push(ls);
    }
    
    /// Get all surfaces on a specific layer
    pub fn surfaces_on_layer(&self, layer: LayerType) -> Vec<&LayerSurface> {
        self.surfaces.iter().filter(|s| s.layer == layer).collect()
    }
    
    /// Arrange layers (calculate positions)
    pub fn arrange_layers(&mut self, output_width: u32, output_height: u32) {
        debug!("Arranging layer surfaces");
        
        // Sort by layer type (background first, overlay last)
        self.surfaces.sort_by_key(|s| match s.layer {
            LayerType::Background => 0,
            LayerType::Bottom => 1,
            LayerType::Top => 2,
            LayerType::Overlay => 3,
        });
        
        // Calculate exclusive zones
        let mut margins = (0u32, 0u32, 0u32, 0u32); // top, bottom, left, right
        
        for surface in &mut self.surfaces {
            if surface.exclusive_zone > 0 {
                // Update margins based on anchor
                if surface.anchor.top {
                    margins.0 += surface.exclusive_zone as u32;
                }
                if surface.anchor.bottom {
                    margins.1 += surface.exclusive_zone as u32;
                }
                if surface.anchor.left {
                    margins.2 += surface.exclusive_zone as u32;
                }
                if surface.anchor.right {
                    margins.3 += surface.exclusive_zone as u32;
                }
            }
            
            // Send configure event
            surface.layer_surface.with_pending_state(|state| {
                state.size = (surface.width, surface.height);
            });
            surface.layer_surface.send_configure();
        }
    }
}

// Layer Shell request handler
impl wlr_layer_shell_v1::WlrLayerShellV1Handler for CompositorState {
    fn new_surface(
        &mut self,
        _state: &mut WlrLayerShellV1,
        surface: WlSurface,
        layer_surface: ZwlrLayerSurfaceV1,
        layer: u32,
        namespace: String,
    ) {
        debug!("New layer surface request");
        self.layer_shell_handler.handle_new_surface(
            surface,
            layer_surface,
            layer,
            namespace,
        );
    }
}

// Layer Surface request handler
impl zwlr_layer_surface_v1::ZwlrLayerSurfaceV1Handler for CompositorState {
    fn destroy(&mut self, state: &mut ZwlrLayerSurfaceV1) {
        debug!("Layer surface destroyed");
        // Remove from handler
        self.layer_shell_handler.surfaces.retain(|s| {
            s.layer_surface != *state
        });
    }
    
    fn set_size(
        &mut self,
        state: &mut ZwlrLayerSurfaceV1,
        width: u32,
        height: u32,
    ) {
        debug!("Layer surface size: {}x{}", width, height);
        // Update surface size
        if let Some(surface) = self.layer_shell_handler.surfaces.iter_mut()
            .find(|s| s.layer_surface == *state)
        {
            surface.width = width;
            surface.height = height;
        }
    }
    
    fn set_anchor(
        &mut self,
        state: &mut ZwlrLayerSurfaceV1,
        anchor: u32,
    ) {
        debug!("Layer surface anchor set");
        if let Some(surface) = self.layer_shell_handler.surfaces.iter_mut()
            .find(|s| s.layer_surface == *state)
        {
            surface.anchor = Anchor::from_bits(anchor);
        }
    }
    
    fn set_exclusive_zone(
        &mut self,
        state: &mut ZwlrLayerSurfaceV1,
        zone: i32,
    ) {
        debug!("Layer surface exclusive zone: {}", zone);
        if let Some(surface) = self.layer_shell_handler.surfaces.iter_mut()
            .find(|s| s.layer_surface == *state)
        {
            surface.exclusive_zone = zone;
        }
    }
    
    fn set_keyboard_interactivity(
        &mut self,
        state: &mut ZwlrLayerSurfaceV1,
        keyboard_interactivity: zwlr_layer_surface_v1::KeyboardInteractivity,
    ) {
        debug!("Layer surface keyboard interactivity: {:?}", keyboard_interactivity);
        // TODO: Handle keyboard focus
    }
    
    fn get_layer(
        &mut self,
        state: &mut ZwlrLayerSurfaceV1,
    ) {
        // Return current layer
    }
}
