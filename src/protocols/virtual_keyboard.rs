//! Virtual Keyboard module
//! 
//! Implements virtual keyboard protocol for on-screen keyboards and remote input.

use log::{info, debug};
use wayland_server::Resource;
use smithay::reexports::wayland_protocols::misc::zwp_virtual_keyboard_v1::server::{
    zwp_virtual_keyboard_manager_v1, zwp_virtual_keyboard_v1,
};

use crate::input::{InputHandler, KeyState};
use crate::compositor::CompositorState;

/// Virtual keyboard manager
pub struct VirtualKeyboardManager {
    /// Manager instance
    pub manager: zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1,
    /// Active virtual keyboards
    pub keyboards: Vec<VirtualKeyboard>,
}

/// Virtual keyboard
pub struct VirtualKeyboard {
    /// Keyboard resource
    pub keyboard: zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1,
    /// Seat association
    pub seat_id: String,
}

impl VirtualKeyboardManager {
    /// Create new virtual keyboard manager
    pub fn new(display: &wayland_server::DisplayHandle) -> Result<Self, Box<dyn std::error::Error>> {
        info!("⌨️ Initializing Virtual Keyboard protocol...");
        
        let manager = zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1::new::<CompositorState>();
        display.create_global::<zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1>(1, manager.clone());
        
        info!("✅ Virtual Keyboard protocol initialized");
        
        Ok(Self {
            manager,
            keyboards: Vec::new(),
        })
    }
    
    /// Handle virtual keyboard creation
    pub fn handle_virtual_keyboard(
        &mut self,
        keyboard: zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1,
        seat_id: String,
    ) {
        debug!("Virtual keyboard created for seat: {}", seat_id);
        
        self.keyboards.push(VirtualKeyboard {
            keyboard,
            seat_id,
        });
    }
    
    /// Handle key event from virtual keyboard
    pub fn handle_key(
        &mut self,
        keyboard: &zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1,
        time: u32,
        key: u32,
        state: u32,
        input_handler: &mut InputHandler,
    ) {
        debug!("Virtual key event: key={} state={}", key, state);
        
        let key_state = if state == 1 {
            KeyState::Pressed
        } else {
            KeyState::Released
        };
        
        input_handler.handle_key(key, key_state);
    }
}

// Virtual keyboard manager handler
impl zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1Handler for CompositorState {
    fn create_virtual_keyboard(
        &mut self,
        _state: &mut zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1,
        seat: wayland_server::protocol::wl_seat::WlSeat,
        keyboard: zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1,
    ) {
        debug!("Create virtual keyboard request");
        // TODO: Get seat ID properly
        self.virtual_keyboard_manager.handle_virtual_keyboard(
            keyboard,
            "seat0".to_string(),
        );
    }
}

// Virtual keyboard handler
impl zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1Handler for CompositorState {
    fn keymap(
        &mut self,
        state: &mut zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1,
        format: u32,
        fd: wayland_server::backend::OwnedFd,
        size: u32,
    ) {
        debug!("Virtual keyboard keymap: format={}, size={}", format, size);
        // TODO: Load keymap
    }
    
    fn key(
        &mut self,
        state: &mut zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1,
        time: u32,
        key: u32,
        state_u: u32,
    ) {
        debug!("Virtual key pressed");
        self.virtual_keyboard_manager.handle_key(
            state,
            time,
            key,
            state_u,
            &mut self.input_handler,
        );
    }
    
    fn modifiers(
        &mut self,
        state: &mut zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1,
        depressed: u32,
        latched: u32,
        locked: u32,
        group: u32,
    ) {
        debug!("Virtual keyboard modifiers: {:x} {:x} {:x} {}", depressed, latched, locked, group);
    }
}
