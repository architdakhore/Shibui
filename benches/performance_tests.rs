use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Benchmark for layout calculations
fn bench_dynamic_layout(c: &mut Criterion) {
    c.bench_function("dynamic_layout_calculation", |b| {
        b.iter(|| {
            // Simulate layout calculation for 10 windows
            let windows = 10;
            let screen_width = 1920;
            let screen_height = 1080;
            let gap = 8;
            
            // Simple tiling calculation
            let master_width = (screen_width * 0.6) as u32;
            let stack_width = screen_width - master_width;
            let stack_height = (screen_height / (windows - 1)) as u32;
            
            black_box((master_width, stack_width, stack_height));
        })
    });
}

fn bench_workspace_switching(c: &mut Criterion) {
    c.bench_function("workspace_switch_time", |b| {
        b.iter(|| {
            // Simulate workspace switch operation
            let current_workspace = 1;
            let target_workspace = 5;
            let animation_duration_ms = 250;
            
            black_box((current_workspace, target_workspace, animation_duration_ms));
        })
    });
}

fn bench_window_rendering(c: &mut Criterion) {
    c.bench_function("window_render_preparation", |b| {
        b.iter(|| {
            // Simulate window render preparation
            let window_count = 5;
            let buffer_size = (1920 * 1080 * 4) as usize; // RGBA
            let damage_regions = 3;
            
            black_box((window_count, buffer_size, damage_regions));
        })
    });
}

fn bench_input_latency(c: &mut Criterion) {
    c.bench_function("input_event_processing", |b| {
        b.iter(|| {
            // Simulate input event processing
            let event_type = "keyboard";
            let keycode = 36;
            let modifiers = 0;
            
            black_box((event_type, keycode, modifiers));
        })
    });
}

fn bench_overview_mode(c: &mut Criterion) {
    c.bench_function("overview_render_calculation", |b| {
        b.iter(|| {
            // Simulate overview mode calculations
            let workspace_count = 10;
            let scale_factor = 0.15;
            let gap_size = 20;
            let screen_width = 1920;
            let screen_height = 1080;
            
            black_box((workspace_count, scale_factor, gap_size, screen_width, screen_height));
        })
    });
}

criterion_group!(
    benches,
    bench_dynamic_layout,
    bench_workspace_switching,
    bench_window_rendering,
    bench_input_latency,
    bench_overview_mode
);

criterion_main!(benches);