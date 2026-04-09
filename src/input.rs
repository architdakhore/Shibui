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
    
    /// Handle keyboard event
    pub fn handle_key(&mut self, key: u32, state: KeyState) {
        match state {
            KeyState::Pressed => {
                // Check for modifier keys
                match key {
                    42 | 54 => self.modifiers.shift = true,  // Shift
                    29 | 97 => self.modifiers.ctrl = true,   // Ctrl
                    56 | 100 => self.modifiers.alt = true,   // Alt
                    125 => self.modifiers.logo = true,        // Super
                    _ => {}
                }
                
                // Add to pressed keys
                for keyboard in self.keyboards.values_mut() {
                    if !keyboard.keys_pressed.contains(&key) {
                        keyboard.keys_pressed.push(key);
                    }
                }
            }
            KeyState::Released => {
                // Update modifier state
                match key {
                    42 | 54 => self.modifiers.shift = false,
                    29 | 97 => self.modifiers.ctrl = false,
                    56 | 100 => self.modifiers.alt = false,
                    125 => self.modifiers.logo = false,
                    _ => {}
                }
                
                // Remove from pressed keys
                for keyboard in self.keyboards.values_mut() {
                    keyboard.keys_pressed.retain(|&k| k != key);
                }
            }
        }
    }
    
    /// Handle pointer motion
    pub fn handle_motion(&mut self, dx: f64, dy: f64) {
        for pointer in self.pointers.values_mut() {
            pointer.x += dx;
            pointer.y += dy;
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
