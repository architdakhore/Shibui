//! Performance Profiling module
//! 
//! Tools for measuring and optimizing compositor performance.

use log::{info, debug};
use std::time::{Duration, Instant};

/// Performance metrics
pub struct PerformanceMetrics {
    /// Frames per second
    pub fps: f32,
    /// Frame time (ms)
    pub frame_time_ms: f32,
    /// Input latency (ms)
    pub input_latency_ms: f32,
    /// Memory usage (MB)
    pub memory_mb: f32,
    /// GPU usage (%)
    pub gpu_usage: f32,
}

/// Performance profiler
pub struct Profiler {
    /// Frame timing
    frame_times: Vec<Duration>,
    /// Last frame time
    last_frame: Instant,
    /// FPS counter
    fps_counter: u32,
    /// FPS update time
    fps_timer: Instant,
    /// Current FPS
    current_fps: f32,
}

impl Profiler {
    /// Create new profiler
    pub fn new() -> Self {
        Self {
            frame_times: Vec::with_capacity(120),
            last_frame: Instant::now(),
            fps_counter: 0,
            fps_timer: Instant::now(),
            current_fps: 0.0,
        }
    }
    
    /// Mark frame start
    pub fn frame_start(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_frame);
        self.last_frame = now;
        
        self.frame_times.push(delta);
        self.fps_counter += 1;
        
        // Keep only last 120 frames
        if self.frame_times.len() > 120 {
            self.frame_times.remove(0);
        }
        
        // Update FPS every second
        if self.fps_timer.elapsed() >= Duration::from_secs(1) {
            self.current_fps = self.fps_counter as f32;
            self.fps_counter = 0;
            self.fps_timer = Instant::now();
            
            debug!("FPS: {:.1}", self.current_fps);
        }
    }
    
    /// Get current metrics
    pub fn get_metrics(&self) -> PerformanceMetrics {
        let avg_frame_time = self.frame_times.iter()
            .map(|d| d.as_secs_f32() * 1000.0)
            .sum::<f32>() / self.frame_times.len().max(1) as f32;
        
        PerformanceMetrics {
            fps: self.current_fps,
            frame_time_ms: avg_frame_time,
            input_latency_ms: 0.0, // TODO: Measure
            memory_mb: 0.0,        // TODO: Measure
            gpu_usage: 0.0,        // TODO: Measure
        }
    }
    
    /// Print performance summary
    pub fn print_summary(&self) {
        let metrics = self.get_metrics();
        
        info!("📊 Performance Summary:");
        info!("   FPS: {:.1}", metrics.fps);
        info!("   Frame Time: {:.2}ms", metrics.frame_time_ms);
        info!("   Input Latency: {:.2}ms", metrics.input_latency_ms);
        info!("   Memory: {:.1}MB", metrics.memory_mb);
        info!("   GPU: {:.1}%", metrics.gpu_usage);
    }
    
    /// Check if performance is good
    pub fn is_performance_good(&self) -> bool {
        self.current_fps >= 60.0
    }
    
    /// Check if performance matches Hyprland
    pub fn matches_hyprland(&self) -> bool {
        self.current_fps >= 144.0
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Benchmark runner
pub struct Benchmark {
    /// Benchmark name
    pub name: String,
    /// Start time
    start: Option<Instant>,
    /// Results
    results: Vec<Duration>,
}

impl Benchmark {
    /// Create new benchmark
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            start: None,
            results: Vec::new(),
        }
    }
    
    /// Start benchmark
    pub fn start(&mut self) {
        self.start = Some(Instant::now());
        debug!("Benchmark '{}' started", self.name);
    }
    
    /// Stop benchmark
    pub fn stop(&mut self) -> Duration {
        if let Some(start) = self.start.take() {
            let duration = start.elapsed();
            self.results.push(duration);
            debug!("Benchmark '{}' completed: {:?}", self.name, duration);
            duration
        } else {
            Duration::from_secs(0)
        }
    }
    
    /// Get average time
    pub fn average(&self) -> Duration {
        if self.results.is_empty() {
            return Duration::from_secs(0);
        }
        
        let total: Duration = self.results.iter().sum();
        total / self.results.len() as u32
    }
    
    /// Print results
    pub fn print_results(&self) {
        let avg = self.average();
        info!("📊 Benchmark '{}' results:", self.name);
        info!("   Runs: {}", self.results.len());
        info!("   Average: {:?}", avg);
        info!("   Min: {:?}", self.results.iter().min().unwrap_or(&Duration::from_secs(0)));
        info!("   Max: {:?}", self.results.iter().max().unwrap_or(&Duration::from_secs(0)));
    }
}
