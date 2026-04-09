//! OpenGL ES renderer
//! 
//! High-performance GPU rendering using OpenGL ES 3.0+

use log::{info, debug};
use glow::{HasContext, Context};
use crate::render::Renderer;

/// OpenGL ES renderer
pub struct GlesRenderer {
    /// OpenGL context
    gl: Context,
    /// Whether renderer is initialized
    initialized: bool,
    /// Current framebuffer
    framebuffer: Option<u32>,
    /// Viewport size
    viewport_width: u32,
    viewport_height: u32,
}

impl GlesRenderer {
    /// Create new OpenGL ES renderer
    pub fn new(gl: Context) -> Self {
        Self {
            gl,
            initialized: false,
            framebuffer: None,
            viewport_width: 0,
            viewport_height: 0,
        }
    }
    
    /// Get OpenGL context
    pub fn gl(&self) -> &Context {
        &self.gl
    }
    
    /// Set viewport size
    pub fn set_viewport(&mut self, width: u32, height: u32) {
        self.viewport_width = width;
        self.viewport_height = height;
        
        unsafe {
            self.gl.viewport(0, 0, width as i32, height as i32);
        }
    }
}

impl Renderer for GlesRenderer {
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🎨 Initializing OpenGL ES renderer...");
        
        unsafe {
            // Enable blending
            self.gl.enable(glow::BLEND);
            self.gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
            
            // Set clear color
            self.gl.clear_color(0.1, 0.1, 0.1, 1.0);
            
            // Enable depth testing
            self.gl.enable(glow::DEPTH_TEST);
            
            info!("✅ OpenGL ES renderer initialized");
        }
        
        self.initialized = true;
        Ok(())
    }
    
    fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.initialized {
            return Err("Renderer not initialized".into());
        }
        
        unsafe {
            // Clear screen
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            
            // TODO: Render windows, borders, decorations
            // TODO: Render overview if active
            // TODO: Render animations
            
            debug!("Frame rendered");
        }
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        "OpenGL ES"
    }
}
