//! Rendering module
//! 
//! GPU-accelerated rendering using OpenGL ES and optional Vulkan support.

use log::{info, debug};

mod gles;
mod error_overlay;

pub use gles::GlesRenderer;
pub use error_overlay::ErrorOverlay;

/// Main renderer trait
pub trait Renderer {
    /// Initialize renderer
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Render a frame
    fn render(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Get renderer name
    fn name(&self) -> &str;
}
