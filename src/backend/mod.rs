//! Backend module
//! 
//! Handles different backends for running the compositor:
//! - DRM/KMS (direct hardware)
//! - Winit (windowed for testing)
//! - Headless (no display, for testing)

use log::{info, debug, error};
use anyhow::{Result, Context};

pub mod drm;
pub mod winit;
pub mod headless;

/// Backend trait
pub trait Backend {
    /// Initialize backend
    fn initialize(&mut self) -> Result<()>;
    
    /// Run backend event loop
    fn run(&mut self) -> Result<()>;
    
    /// Get backend name
    fn name(&self) -> &str;
    
    /// Get screen size
    fn screen_size(&self) -> (u32, u32);
}

/// Backend type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BackendType {
    /// DRM/KMS backend (direct hardware)
    DRM,
    /// Winit backend (windowed)
    Winit,
    /// Headless backend (no display)
    Headless,
}

impl BackendType {
    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "drm" | "kms" => Some(Self::DRM),
            "winit" | "window" => Some(Self::Winit),
            "headless" | "none" => Some(Self::Headless),
            _ => None,
        }
    }
    
    /// Get as string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DRM => "drm",
            Self::Winit => "winit",
            Self::Headless => "headless",
        }
    }
}

/// Backend manager
pub struct BackendManager {
    /// Current backend type
    backend_type: BackendType,
    /// Backend instance
    backend: Option<Box<dyn Backend>>,
}

impl BackendManager {
    /// Create new backend manager
    pub fn new() -> Self {
        Self {
            backend_type: BackendType::Winit, // Default to winit for testing
            backend: None,
        }
    }
    
    /// Set backend type
    pub fn set_backend(&mut self, backend_type: BackendType) {
        info!("🔧 Setting backend: {:?}", backend_type);
        self.backend_type = backend_type;
    }
    
    /// Initialize backend
    pub fn initialize(&mut self) -> Result<()> {
        info!("🔧 Initializing backend: {:?}", self.backend_type);
        
        match self.backend_type {
            BackendType::DRM => {
                // TODO: Initialize DRM backend
                // self.backend = Some(Box::new(drm::DrmBackend::new()?));
                error!("DRM backend not yet implemented");
                return Err(anyhow::anyhow!("DRM backend not implemented"));
            }
            BackendType::Winit => {
                // TODO: Initialize Winit backend
                // self.backend = Some(Box::new(winit::WinitBackend::new()?));
                info!("Using Winit backend for testing");
            }
            BackendType::Headless => {
                // TODO: Initialize Headless backend
                // self.backend = Some(Box::new(headless::HeadlessBackend::new()?));
                info!("Using Headless backend");
            }
        }
        
        if let Some(backend) = &mut self.backend {
            backend.initialize()?;
            info!("✅ Backend initialized: {}", backend.name());
        }
        
        Ok(())
    }
    
    /// Run backend
    pub fn run(&mut self) -> Result<()> {
        if let Some(backend) = &mut self.backend {
            info!("🚀 Starting backend event loop");
            backend.run()?;
        } else {
            error!("Backend not initialized");
            return Err(anyhow::anyhow!("Backend not initialized"));
        }
        
        Ok(())
    }
    
    /// Get current backend type
    pub fn backend_type(&self) -> BackendType {
        self.backend_type
    }
    
    /// Get screen size
    pub fn screen_size(&self) -> (u32, u32) {
        if let Some(backend) = &self.backend {
            backend.screen_size()
        } else {
            (1920, 1080) // Default
        }
    }
}

impl Default for BackendManager {
    fn default() -> Self {
        Self::new()
    }
}
