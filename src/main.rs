//! ShibUI - A High-Performance Wayland Compositor
//! 
//! Features:
//! - Dynamic tiling (Hyprland-style)
//! - Horizontal tiling with scrolling (niri-style)
//! - Vertical tiling with scrolling (MangoWM-style)
//! - Center layout (MangoWM-style)
//! - Overview mode with dual-axis scrolling
//! - Workspace system
//! - Quicksell-compatible configuration
//! - GPU-accelerated rendering
//! - IPC and CLI support
//! - Performance profiling

#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::similar_names)]

mod compositor;
mod input;
mod layout;
mod render;
mod workspace;
mod overview;
mod config;
mod animations;
mod utils;
mod window;
mod decorations;
mod protocols;
mod backend;
mod ipc;
mod profiler;
mod cli;

use anyhow::Result;
use log::{info, error};
use std::env;

/// ShibUI version
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Compositor name
const COMPOSITOR_NAME: &str = "ShibUI";

fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info")
    ).init();

    info!("🌊 {} v{} starting...", COMPOSITOR_NAME, VERSION);
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.contains(&"--version".to_string()) || args.contains(&"-V".to_string()) {
        println!("{} {}", COMPOSITOR_NAME, VERSION);
        return Ok(());
    }
    
    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        print_help();
        return Ok(());
    }

    // Initialize and run the compositor
    match compositor::Compositor::new() {
        Ok(mut compositor) => {
            info!("✅ Compositor initialized successfully");
            compositor.run()?;
        }
        Err(e) => {
            error!("❌ Failed to initialize compositor: {}", e);
            return Err(e);
        }
    }

    info!("👋 {} shutting down", COMPOSITOR_NAME);
    Ok(())
}

fn print_help() {
    println!(r#"
🌊 {} v{} - A High-Performance Wayland Compositor

USAGE:
    flowwm [OPTIONS]

OPTIONS:
    -h, --help       Print this help message
    -V, --version    Print version information
    --config <PATH>  Use custom configuration file
    --debug          Enable debug logging
    --no-animations  Disable animations

LAYOUT MODES:
    dynamic      Hyprland-style dynamic tiling
    horizontal   niri-style horizontal tiling
    vertical     MangoWM-style vertical tiling
    center       MangoWM-style center layout

KEYBINDINGS (Default):
    SUPER+Return     Open terminal
    SUPER+D          Open application launcher
    SUPER+Tab        Toggle overview
    SUPER+HJKL       Focus windows
    SUPER+Shift+Q    Close window
    SUPER+1-9        Switch workspaces

For more information, see the documentation at:
https://github.com/yourusername/flowwm
"#, COMPOSITOR_NAME, VERSION);
}
