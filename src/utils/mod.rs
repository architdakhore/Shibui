//! Utility functions and helpers

use std::process::Command;

/// Spawn a process
pub fn spawn_process(command: &str, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    Command::new(command).args(args).spawn()?;
    Ok(())
}

/// Run a process and wait for completion
pub fn run_process(command: &str, args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new(command).args(args).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Parse color from hex string
pub fn parse_hex_color(color: &str) -> (f32, f32, f32, f32) {
    let color = color.trim_start_matches('#');
    
    let r = u8::from_str_radix(&color[0..2], 16).unwrap_or(0) as f32 / 255.0;
    let g = u8::from_str_radix(&color[2..4], 16).unwrap_or(0) as f32 / 255.0;
    let b = u8::from_str_radix(&color[4..6], 16).unwrap_or(0) as f32 / 255.0;
    let a = if color.len() == 8 {
        u8::from_str_radix(&color[6..8], 16).unwrap_or(255) as f32 / 255.0
    } else {
        1.0
    };
    
    (r, g, b, a)
}

/// Clamp a value between min and max
pub fn clamp<T: Ord>(value: T, min: T, max: T) -> T {
    value.max(min).min(max)
}

/// Linear interpolation
pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}

/// Get current time in seconds
pub fn get_time_seconds() -> f64 {
    use std::time::Instant;
    static START: once_cell::sync::Lazy<Instant> = once_cell::sync::Lazy::new(Instant::now);
    START.elapsed().as_secs_f64()
}

/// Get delta time between frames
pub fn get_delta_time() -> f32 {
    use std::time::Instant;
    static LAST: once_cell::sync::Lazy<Instant> = once_cell::sync::Lazy::new(Instant::now);
    let now = Instant::now();
    let delta = now.duration_since(*LAST).as_secs_f32();
    unsafe {
        // This is a hack for the static mut, in real code use proper synchronization
        *(std::ptr::addr_of_mut!(LAST) as *mut Instant) = now;
    }
    delta
}
