//! Input handling module
//! 
//! Handles keyboard, mouse, and touch input devices using libinput.

use anyhow::{Result, Context};
use log::{info, debug};
use std::collections::HashMap;

/// Input handler for managing input devices
pub struct InputHandler {
    /// Keyboard devices
    keyboards: HashMap<String, KeyboardDevice>,
    /// Pointer devices
    pointers: HashMap<String, PointerDevice>,
    /// Touch devices
    touch_devices: HashMap<String, TouchDevice>,
    /// Modifier state
    modifiers: ModifierState,
}

/// Keyboard device representation
pub struct KeyboardDevice {
    /// Device name
    pub name: String,
    /// XKB keymap
    pub keymap: String,
    /// Current key state
    pub keys_pressed: Vec<u32>,
}

/// Pointer device (mouse/touchpad)
pub struct PointerDevice {
    /// Device name
    pub name: String,
    /// Pointer position
    pub x: f64,
    pub y: f64,
    /// Button state
    pub buttons: u32,
}

/// Touch device
pub struct TouchDevice {
    /// Device name
    pub name: String,
    /// Active touches
    pub touches: Vec<TouchPoint>,
}

/// Touch point
pub struct TouchPoint {
    pub id: i32,
    pub x: f64,
    pub y: f64,
}

/// Modifier key state
pub struct ModifierState {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub logo: bool,  // Super/Windows key
}

impl InputHandler {
    /// Create a new input handler
    pub fn new() -> Result<Self> {
        info!("🎮 Initializing input handler...");
        
        Ok(Self {
            keyboards: HashMap::new(),
            pointers: HashMap::new(),
            touch_devices: HashMap::new(),
            modifiers: ModifierState {
                shift: false,
                ctrl: false,
                alt: false,
                logo: false,
            },
        })
    }
    
    /// Add a keyboard device
    pub fn add_keyboard(&mut self, name: String) -> Result<()> {
        debug!("Adding keyboard: {}", name);
        
        let keyboard = KeyboardDevice {
            name: name.clone(),
            keymap: "us".to_string(), // Default keymap
            keys_pressed: Vec::new(),
        };
        
        self.keyboards.insert(name, keyboard);
        info!("✅ Keyboard added");
        Ok(())
    }
    
    /// Add a pointer device
    pub fn add_pointer(&mut self, name: String) -> Result<()> {
        debug!("Adding pointer: {}", name);
        
        let pointer = PointerDevice {
            name: name.clone(),
            x: 0.0,
            y: 0.0,
            buttons: 0,
        };
        
        self.pointers.insert(name, pointer);
        info!("✅ Pointer added");
        Ok(())
    }
    
    /// Handle keyboard event with XKB-based modifier detection
    /// 
    /// IMPORTANT: For production use, integrate with xkbcommon::xkb::State
    /// Use xkb_state.mod_name_is_active() with standard modifier names:
    /// - xkb::MOD_NAME_SHIFT, xkb::MOD_NAME_CTRL, xkb::MOD_NAME_ALT, xkb::MOD_NAME_LOGO
    /// This ensures compatibility with different keyboard layouts and remappings.
    pub fn handle_key(&mut self, key: u32, state: KeyState, xkb_state: Option<&xkbcommon::xkb::State>) {
        match state {
            KeyState::Pressed => {
                // Use XKB state if available for proper modifier detection
                if let Some(state) = xkb_state {
                    // Proper XKB-based modifier detection
                    self.modifiers.shift = state.mod_name_is_active(xkbcommon::xkb::MOD_NAME_SHIFT);
                    self.modifiers.ctrl = state.mod_name_is_active(xkbcommon::xkb::MOD_NAME_CTRL);
                    self.modifiers.alt = state.mod_name_is_active(xkbcommon::xkb::MOD_NAME_ALT);
                    self.modifiers.logo = state.mod_name_is_active(xkbcommon::xkb::MOD_NAME_LOGO);
                } else {
                    // Fallback: hardcoded keycodes (NOT recommended for production)
                    // These may break on non-standard keyboards or remapped layouts
                    match key {
                        42 | 54 => self.modifiers.shift = true,  // Left/Right Shift
                        29 | 97 => self.modifiers.ctrl = true,   // Left/Right Control
                        56 => self.modifiers.alt = true,          // Left Alt
                        125 => self.modifiers.logo = true,        // Left Super/Logo
                        _ => {}
                    }
                }
                
                // Add to pressed keys
                for keyboard in self.keyboards.values_mut() {
                    if !keyboard.keys_pressed.contains(&key) {
                        keyboard.keys_pressed.push(key);
                    }
                }
            }
            KeyState::Released => {
                // Update modifiers on key release
                if let Some(state) = xkb_state {
                    self.modifiers.shift = state.mod_name_is_active(xkbcommon::xkb::MOD_NAME_SHIFT);
                    self.modifiers.ctrl = state.mod_name_is_active(xkbcommon::xkb::MOD_NAME_CTRL);
                    self.modifiers.alt = state.mod_name_is_active(xkbcommon::xkb::MOD_NAME_ALT);
                    self.modifiers.logo = state.mod_name_is_active(xkbcommon::xkb::MOD_NAME_LOGO);
                } else {
                    // Fallback for hardcoded keycodes
                    match key {
                        42 | 54 => self.modifiers.shift = false,
                        29 | 97 => self.modifiers.ctrl = false,
                        56 => self.modifiers.alt = false,
                        125 => self.modifiers.logo = false,
                        _ => {}
                    }
                }
                
                // Remove from pressed keys
                for keyboard in self.keyboards.values_mut() {
                    keyboard.keys_pressed.retain(|&k| k != key);
                }
            }
        }
    }
    
    /// Handle pointer motion with bounds checking
    /// 
    /// Parameters:
    /// - dx, dy: Delta movement from input device
    /// - bounds: Optional screen bounds (width, height) to clamp pointer position
    pub fn handle_motion(&mut self, dx: f64, dy: f64, bounds: Option<(u32, u32)>) {
        for pointer in self.pointers.values_mut() {
            // Apply delta
            pointer.x += dx;
            pointer.y += dy;
            
            // Clamp to output bounds if provided
            // This prevents negative or out-of-bounds cursor coordinates
            if let Some((width, height)) = bounds {
                pointer.x = pointer.x.max(0.0).min(width as f64);
                pointer.y = pointer.y.max(0.0).min(height as f64);
            }
        }
    }
    
    /// Handle pointer button
    pub fn handle_button(&mut self, button: u32, state: ButtonState) {
        for pointer in self.pointers.values_mut() {
            match state {
                ButtonState::Pressed => {
                    pointer.buttons |= (1 << button);
                }
                ButtonState::Released => {
                    pointer.buttons &= !(1 << button);
                }
            }
        }
    }
    
    /// Get current modifier state
    pub fn modifiers(&self) -> &ModifierState {
        &self.modifiers
    }
    
    /// Check if a specific key is pressed
    pub fn is_key_pressed(&self, key: u32) -> bool {
        self.keyboards.values().any(|kb| kb.keys_pressed.contains(&key))
    }
}

/// Key state
#[derive(Debug, Clone, Copy)]
pub enum KeyState {
    Pressed,
    Released,
}

/// Button state
#[derive(Debug, Clone, Copy)]
pub enum ButtonState {
    Pressed,
    Released,
}
