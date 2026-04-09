//! Rendering module
//! 
//! GPU-accelerated rendering using OpenGL ES and optional Vulkan support.

use log::{info, debug};

mod gles;

pub use gles::GlesRenderer;

/// Main renderer trait
pub trait Renderer {
    /// Initialize renderer
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Render a frame
    fn render(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Get renderer name
    fn name(&self) -> &str;
}
