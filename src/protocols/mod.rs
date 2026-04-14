//! Protocols module
//! 
//! Implements various Wayland protocols for compositor functionality.

pub mod xdg_shell;
pub mod layer_shell;
pub mod output;
pub mod input_method;
pub mod virtual_keyboard;
pub mod screencopy;

pub use xdg_shell::*;
pub use layer_shell::*;
pub use output::*;
