//! Input Method module
//! 
//! Implements input method protocol for IME (Input Method Editor) support.
//! Essential for CJK (Chinese, Japanese, Korean) input.

use log::{info, debug};
use wayland_server::{
    protocol::wl_surface::WlSurface,
    Client, Resource,
};
use smithay::reexports::wayland_protocols_misc::zwp_input_method_v2::server::{
    zwp_input_method_manager_v2, zwp_input_method_v2, zwp_input_popup_surface_v2,
};

use crate::compositor::CompositorState;

/// Input method manager
pub struct InputMethodManager {
    /// Manager instance
    pub manager: zwp_input_method_manager_v2::ZwpInputMethodManagerV2,
    /// Active input method
    pub active_input_method: Option<InputMethod>,
}

/// Input method
pub struct InputMethod {
    /// Input method resource
    pub input_method: zwp_input_method_v2::ZwpInputMethodV2,
    /// Commit string
    pub commit_string: String,
    /// Pre-edit string
    pub preedit_string: String,
}

impl InputMethodManager {
    /// Create new input method manager
    pub fn new(display: &wayland_server::DisplayHandle) -> Result<Self, Box<dyn std::error::Error>> {
        info!("⌨️ Initializing Input Method protocol...");
        
        let manager = zwp_input_method_manager_v2::ZwpInputMethodManagerV2::new::<CompositorState>();
        display.create_global::<zwp_input_method_manager_v2::ZwpInputMethodManagerV2>(1, manager.clone());
        
        info!("✅ Input Method protocol initialized");
        
        Ok(Self {
            manager,
            active_input_method: None,
        })
    }
    
    /// Handle input method creation
    pub fn handle_input_method(&mut self, input_method: zwp_input_method_v2::ZwpInputMethodV2) {
        debug!("Input method created");
        
        self.active_input_method = Some(InputMethod {
            input_method,
            commit_string: String::new(),
            preedit_string: String::new(),
        });
    }
    
    /// Commit text from input method
    pub fn commit_text(&mut self, text: String) {
        debug!("IME commit: {}", text);
        
        if let Some(im) = &mut self.active_input_method {
            im.commit_string = text;
            // TODO: Send to focused surface
        }
    }
    
    /// Set pre-edit text
    pub fn set_preedit(&mut self, text: String) {
        if let Some(im) = &mut self.active_input_method {
            im.preedit_string = text;
        }
    }
}

// Input method manager handler
impl zwp_input_method_manager_v2::ZwpInputMethodManagerV2Handler for CompositorState {
    fn get_input_method(
        &mut self,
        _state: &mut zwp_input_method_manager_v2::ZwpInputMethodManagerV2,
        seat: wayland_server::protocol::wl_seat::WlSeat,
        input_method: zwp_input_method_v2::ZwpInputMethodV2,
    ) {
        debug!("Get input method request");
        self.input_method_manager.handle_input_method(input_method);
    }
}

// Input method handler
impl zwp_input_method_v2::ZwpInputMethodV2Handler for CompositorState {
    fn destroy(&mut self, state: &mut zwp_input_method_v2::ZwpInputMethodV2) {
        debug!("Input method destroyed");
        self.input_method_manager.active_input_method = None;
    }
    
    fn commit_string(
        &mut self,
        state: &mut zwp_input_method_v2::ZwpInputMethodV2,
        text: String,
    ) {
        debug!("IME commit string: {}", text);
        self.input_method_manager.commit_text(text);
    }
    
    fn preedit_string(
        &mut self,
        state: &mut zwp_input_method_v2::ZwpInputMethodV2,
        text: String,
        cursor_begin: i32,
        cursor_end: i32,
    ) {
        debug!("IME preedit: {} ({}-{})", text, cursor_begin, cursor_end);
        self.input_method_manager.set_preedit(text);
    }
    
    fn delete_surrounding_text(
        &mut self,
        state: &mut zwp_input_method_v2::ZwpInputMethodV2,
        before_length: u32,
        after_length: u32,
    ) {
        debug!("IME delete surrounding: {} before, {} after", before_length, after_length);
    }
    
    fn done(&mut self, state: &mut zwp_input_method_v2::ZwpInputMethodV2) {
        debug!("IME done");
    }
}
