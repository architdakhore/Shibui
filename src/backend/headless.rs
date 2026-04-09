//! Headless Backend
//! 
//! Backend without display output, used for testing and automated testing.

use log::{info, debug};
use anyhow::Result;
use std::time::{Duration, Instant};

use super::Backend;

/// Headless backend
pub struct HeadlessBackend {
    /// Screen size (virtual)
    screen_size: (u32, u32),
    /// Running state
    running: bool,
    /// Frame count
    frame_count: u32,
}

impl HeadlessBackend {
    /// Create new headless backend
    pub fn new() -> Result<Self> {
        info!("🔮 Initializing Headless backend...");
        
        Ok(Self {
            screen_size: (1920, 1080),
            running: false,
            frame_count: 0,
        })
    }
}

impl Backend for HeadlessBackend {
    fn initialize(&mut self) -> Result<()> {
        info!("🔧 Initializing Headless backend...");
        
        self.running = true;
        
        info!("✅ Headless backend initialized");
        info!("   Virtual screen: {}x{}", self.screen_size.0, self.screen_size.1);
        
        Ok(())
    }
    
    fn run(&mut self) -> Result<()> {
        info!("🚀 Running Headless backend");
        
        let start = Instant::now();
        
        while self.running {
            // Simulate frame rendering
            self.frame_count += 1;
            
            // Run for a limited time in testing
            if start.elapsed() > Duration::from_secs(5) {
                self.running = false;
            }
            
            std::thread::sleep(Duration::from_millis(16));
        }
        
        info!("👋 Headless backend stopped");
        info!("   Total frames: {}", self.frame_count);
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        "Headless"
    }
    
    fn screen_size(&self) -> (u32, u32) {
        self.screen_size
    }
}

impl HeadlessBackend {
    /// Get frame count
    pub fn frame_count(&self) -> u32 {
        self.frame_count
    }
}
