//! XDG Shell module
//! 
//! Implements the XDG Shell protocol for window management.
//! This is the main protocol for creating and managing application windows.

use log::{info, debug, error};
use wayland_server::{
    protocol::wl_surface::WlSurface,
    Client, DisplayHandle, Resource,
};
use smithay::{
    desktop::{
        Window,
        space::{Space, SpaceElement},
    },
    reexports::wayland_protocols::xdg::{
        decoration::server::xdg_toplevel_decoration_v1,
        shell::server::{
            xdg_positioner, xdg_surface, xdg_toplevel,
            xdg_wm_base::{self, XdgWmBase},
        },
    },
    utils::{Serial, Logical},
    input::pointer::Focus,
};

use crate::compositor::CompositorState;

/// XDG WM Base handler
pub struct XdgWmHandler {
    /// Base instance
    pub wm_base: XdgWmBase,
    /// Pending surfaces waiting for configure
    pending_surfaces: Vec<WlSurface>,
}

impl XdgWmHandler {
    /// Create new XDG WM handler
    pub fn new(display: &DisplayHandle) -> Result<Self, Box<dyn std::error::Error>> {
        info!("🪟 Initializing XDG Shell protocol...");
        
        let wm_base = XdgWmBase::new::<CompositorState>();
        display.create_global::<XdgWmBase>(6, wm_base.clone());
        
        info!("✅ XDG Shell protocol initialized");
        
        Ok(Self {
            wm_base,
            pending_surfaces: Vec::new(),
        })
    }
    
    /// Handle new toplevel creation
    pub fn handle_new_toplevel(
        &mut self,
        state: &mut CompositorState,
        surface: WlSurface,
        xdg_surface: xdg_surface::XdgSurface,
        toplevel: xdg_toplevel::XdgToplevel,
    ) {
        debug!("New XDG toplevel created");
        
        // Create window
        let window = Window::new(WindowElement {
            surface,
            xdg_surface,
            toplevel,
        });
        
        // Add to workspace
        state.workspace_manager.active_workspace_mut().add_window(0); // TODO: Proper window ID
        
        // Send configure event
        // Window will be positioned by layout engine
    }
}

/// XDG Toplevel element
pub struct WindowElement {
    /// Wayland surface
    pub surface: WlSurface,
    /// XDG surface
    pub xdg_surface: xdg_surface::XdgSurface,
    /// XDG toplevel
    pub toplevel: xdg_toplevel::XdgToplevel,
}

impl WindowElement {
    /// Send configure event to window
    pub fn configure(&self, x: i32, y: i32, width: i32, height: i32) {
        self.xdg_surface.with_pending_state(|state| {
            state.geometry = Some((x, y, width, height).into());
        });
        
        self.toplevel.with_pending_state(|state| {
            state.bounds = Some((width, height));
        });
        
        self.xdg_surface.send_configure();
        debug!("Configured window at ({}, {}) {}x{}", x, y, width, height);
    }
    
    /// Set window title
    pub fn set_title(&self, title: String) {
        self.toplevel.with_pending_state(|state| {
            state.title = Some(title);
        });
    }
    
    /// Set window as fullscreen
    pub fn set_fullscreen(&self, fullscreen: bool) {
        if fullscreen {
            self.toplevel.with_pending_state(|state| {
                state.states.set(xdg_toplevel::State::Fullscreen);
            });
        } else {
            self.toplevel.with_pending_state(|state| {
                state.states.unset(xdg_toplevel::State::Fullscreen);
            });
        }
    }
    
    /// Set window as activated (focused)
    pub fn set_activated(&self, activated: bool) {
        if activated {
            self.toplevel.with_pending_state(|state| {
                state.states.set(xdg_toplevel::State::Activated);
            });
        } else {
            self.toplevel.with_pending_state(|state| {
                state.states.unset(xdg_toplevel::State::Activated);
            });
        }
    }
    
    /// Close window
    pub fn close(&self) {
        self.toplevel.send_close();
        debug!("Sent close request to window");
    }
}

// XDG WM Base request handler
impl xdg_wm_base::XdgWmBaseHandler for CompositorState {
    fn wm_base_created(
        &mut self,
        _state: &mut XdgWmBase,
        _client: &Client,
        surface: WlSurface,
    ) {
        debug!("XDG WM base created for surface");
        self.xdg_wm_handler.pending_surfaces.push(surface);
    }
}

// XDG Surface request handler
impl xdg_surface::XdgSurfaceHandler for CompositorState {
    fn ack_configure(
        &mut self,
        _state: &mut xdg_surface::XdgSurface,
        _serial: Serial,
    ) {
        debug!("Window acknowledged configure");
    }
}

// XDG Toplevel request handler
impl xdg_toplevel::XdgToplevelHandler for CompositorState {
    fn toplevel_destroy(
        &mut self,
        _state: &mut xdg_toplevel::XdgToplevel,
    ) {
        debug!("Toplevel destroyed");
        // TODO: Remove window from workspace
    }
    
    fn set_title(
        &mut self,
        state: &mut xdg_toplevel::XdgToplevel,
        title: String,
    ) {
        debug!("Window title set: {}", title);
        // Store title for window
    }
    
    fn set_app_id(
        &mut self,
        state: &mut xdg_toplevel::XdgToplevel,
        app_id: String,
    ) {
        debug!("Window app_id set: {}", app_id);
        // Store app_id for window rules
    }
    
    fn move_request(
        &mut self,
        state: &mut xdg_toplevel::XdgToplevel,
        seat: wayland_server::protocol::wl_seat::WlSeat,
        serial: Serial,
    ) {
        debug!("Window move request");
        // TODO: Implement interactive move
    }
    
    fn resize_request(
        &mut self,
        state: &mut xdg_toplevel::XdgToplevel,
        seat: wayland_server::protocol::wl_seat::WlSeat,
        serial: Serial,
        edges: xdg_toplevel::ResizeEdge,
    ) {
        debug!("Window resize request: {:?}", edges);
        // TODO: Implement interactive resize
    }
    
    fn set_maximized(
        &mut self,
        state: &mut xdg_toplevel::XdgToplevel,
    ) {
        debug!("Window maximize request");
        // TODO: Implement maximize
    }
    
    fn unset_maximized(
        &mut self,
        state: &mut xdg_toplevel::XdgToplevel,
    ) {
        debug!("Window unmaximize request");
        // TODO: Implement unmaximize
    }
    
    fn set_fullscreen(
        &mut self,
        state: &mut xdg_toplevel::XdgToplevel,
        output: Option<wayland_server::protocol::wl_output::WlOutput>,
    ) {
        debug!("Window fullscreen request");
        // TODO: Implement fullscreen
    }
    
    fn unset_fullscreen(
        &mut self,
        state: &mut xdg_toplevel::XdgToplevel,
    ) {
        debug!("Window unfullscreen request");
        // TODO: Implement unfullscreen
    }
    
    fn set_minimized(
        &mut self,
        state: &mut xdg_toplevel::XdgToplevel,
    ) {
        debug!("Window minimize request");
        // TODO: Implement minimize
    }
}

// XDG Decoration handler
impl xdg_toplevel_decoration_v1::XdgToplevelDecorationV1Handler for CompositorState {
    fn new_decoration(
        &mut self,
        _state: &mut xdg_toplevel_decoration_v1::XdgToplevelDecorationV1,
    ) {
        debug!("New decoration request");
        // TODO: Implement server-side decorations
    }
    
    fn set_mode(
        &mut self,
        state: &mut xdg_toplevel_decoration_v1::XdgToplevelDecorationV1,
        mode: xdg_toplevel_decoration_v1::Mode,
    ) {
        debug!("Decoration mode set: {:?}", mode);
        // TODO: Apply decoration mode
    }
    
    fn unset_mode(
        &mut self,
        state: &mut xdg_toplevel_decoration_v1::XdgToplevelDecorationV1,
    ) {
        debug!("Decoration mode unset");
        // TODO: Reset decoration mode
    }
}
