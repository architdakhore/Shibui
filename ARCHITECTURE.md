# FlowWM Architecture

This document describes the internal architecture of FlowWM.

## 🏗️ System Overview

```
┌─────────────────────────────────────────────────────────┐
│                     User Space                          │
├─────────────────────────────────────────────────────────┤
│  FlowWM Compositor                                      │
│  ┌─────────────────────────────────────────────────┐   │
│  │  Input Handler  │  Layout Engine  │  Renderer   │   │
│  │  - libinput     │  - Dynamic      │  - OpenGL ES│   │
│  │  - xkbcommon    │  - Horizontal   │  - Vulkan   │   │
│  │  - Keybindings  │  - Vertical     │  - Shaders  │   │
│  │                 │  - Center       │             │   │
│  └─────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────┐   │
│  │  Workspace Manager  │  Overview  │  Animations  │   │
│  └─────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────┤
│  Wayland Protocol Layer                                 │
│  - wl_compositor                                        │
│  - wl_shm                                               │
│  - xdg_wm_base                                          │
│  - wlr_layer_shell                                      │
│  - wlr_output_management                                │
├─────────────────────────────────────────────────────────┤
│  Smithay Framework                                      │
│  - Backend abstraction (DRM, Winit)                     │
│  - Renderer abstraction                                 │
│  - Input handling                                       │
├─────────────────────────────────────────────────────────┤
│  Linux Kernel                                           │
│  - DRM/KMS  │  libinput  │  evdev                      │
└─────────────────────────────────────────────────────────┘
```

---

## 📦 Module Structure

### `main.rs` - Entry Point

- Initializes logging
- Parses command-line arguments
- Creates and runs compositor instance

### `compositor.rs` - Core Compositor

**Responsibilities:**
- Wayland display server management
- Client connection handling
- Event loop dispatch
- Global object creation

**Key Components:**
```rust
pub struct Compositor {
    display: Display<CompositorState>,
    display_handle: DisplayHandle,
    state: CompositorState,
    config: Config,
}
```

### `input.rs` - Input Handling

**Responsibilities:**
- Keyboard input processing
- Pointer/mouse events
- Touch device support
- Modifier key tracking

**Features:**
- Multi-device support
- Keybinding system
- Repeat key handling
- Natural scrolling

### `layout/` - Tiling Engine

#### `mod.rs` - Layout Manager

Manages switching between layout modes and coordinates layout calculations.

#### `dynamic.rs` - Dynamic Layout (Hyprland-style)

**Algorithm:**
1. Single window: Full screen
2. Two windows: Side-by-side (master/stack)
3. 3+ windows: Master column + vertical stack

**Configurable:**
- Master ratio (width %)
- Master count (number of windows)
- Gap size

#### `horizontal.rs` - Horizontal Layout (niri-style)

**Algorithm:**
- Windows arranged horizontally
- Smooth scrolling animation
- Column-based positioning

**Features:**
- Scroll offset tracking
- Animation interpolation
- Column width configuration

#### `vertical.rs` - Vertical Layout (MangoWM-style)

**Algorithm:**
- Windows arranged vertically
- Smooth scrolling animation
- Row-based positioning

**Features:**
- Scroll offset tracking
- Animation interpolation
- Row height configuration

#### `center.rs` - Center Layout (MangoWM-style)

**Algorithm:**
- Main window centered
- Other windows tiled around
- Multiple patterns supported

**Patterns:**
- Right column
- Left column
- Top/bottom split
- Grid (planned)

### `workspace.rs` - Workspace Management

**Responsibilities:**
- Workspace creation/deletion
- Window assignment
- Workspace switching
- Visibility tracking

**Data Structure:**
```rust
pub struct WorkspaceManager {
    workspaces: HashMap<i32, Workspace>,
    active_workspace: i32,
    workspace_count: i32,
}
```

### `overview.rs` - Overview Mode

**Responsibilities:**
- Overview activation/deactivation
- Workspace thumbnail rendering
- Dual-axis scrolling
- Animation management

**Unique Features:**
- **Both horizontal AND vertical scrolling**
- Configurable scale
- Blur effects
- Smooth animations

### `render/` - Rendering

#### `mod.rs` - Renderer Trait

Defines renderer interface for backend abstraction.

#### `gles.rs` - OpenGL ES Renderer

**Responsibilities:**
- GPU-accelerated rendering
- Framebuffer management
- Shader compilation
- Texture handling

**Features:**
- OpenGL ES 3.0+
- VSync support
- Damage tracking
- Zero-copy buffers

### `config/` - Configuration System

**Responsibilities:**
- TOML config parsing
- Default values
- Config validation
- Hot reloading

**Structure:**
```rust
pub struct Config {
    general: GeneralConfig,
    tiling: TilingConfig,
    workspaces: WorkspaceConfig,
    animations: AnimationConfig,
    overview: OverviewConfig,
    input: InputConfig,
    keybindings: KeybindingsConfig,
    render: RenderConfig,
}
```

### `animations.rs` - Animation System

**Responsibilities:**
- Animation scheduling
- Interpolation
- Easing curves
- Progress tracking

**Easing Curves:**
- Linear
- Ease-in/out
- Ease-in-out
- Ease-out-expo
- Spring

---

## 🔄 Event Flow

### 1. Input Event

```
Hardware → libinput → InputHandler → KeybindingMatch → Action → LayoutUpdate → Render
```

### 2. Window Creation

```
Client → XDG Surface → Workspace.AddWindow → Layout.Calculate → Render
```

### 3. Workspace Switch

```
Keybinding → WorkspaceManager.SetActive → Animation.Start → Render.MultipleFrames
```

### 4. Overview Toggle

```
Keybinding → Overview.Toggle → CalculateLayout → Render.WithScale → UserSelect → Workspace.Switch
```

---

## 📊 Data Flow

### Window Management

```
Window Creation
    ↓
Workspace Assignment
    ↓
Layout Calculation
    ↓
Geometry Assignment
    ↓
Rendering
    ↓
Display
```

### Configuration

```
Config File (TOML)
    ↓
Parse & Validate
    ↓
Config Struct
    ↓
Modules (Layout, Input, etc.)
    ↓
Apply Settings
```

---

## 🎯 Performance Optimizations

### 1. Rendering

- **Damage Tracking:** Only redraw changed areas
- **Zero-Copy Buffers:** Direct GPU memory access
- **Async Frame Scheduling:** VRR/FreeSync support
- **Multi-threading:** Separate render thread

### 2. Memory

- **Object Pooling:** Reuse allocations
- **Smart Caching:** Pre-render common elements
- **Lazy Loading:** Load resources on demand

### 3. Input

- **Direct libinput:** Minimal abstraction layers
- **Event Batching:** Process multiple events together
- **Low-latency Path:** Priority queue for input

---

## 🔌 Extension Points

### Plugins (Planned)

```rust
pub trait Plugin {
    fn on_window_created(&mut self, window: &Window);
    fn on_key_pressed(&mut self, key: u32);
    fn on_workspace_switch(&mut self, from: i32, to: i32);
}
```

### Custom Layouts

Implement the `LayoutEngine` trait:

```rust
pub trait LayoutEngine {
    fn calculate(&self, workspace: Rect, windows: usize) -> Vec<LayoutInfo>;
}
```

### Custom Animations

Add new easing curves to `AnimationCurve` enum.

---

## 🧪 Testing Strategy

### Unit Tests

- Layout calculations
- Configuration parsing
- Animation interpolation
- Keybinding matching

### Integration Tests

- Window management
- Workspace switching
- Overview mode
- Input handling

### Performance Tests

- Frame timing
- Memory usage
- Input latency
- Boot time

---

## 📈 Future Enhancements

### Short-term (v0.2)

- [ ] Vulkan renderer
- [ ] Touch gesture support
- [ ] Plugin system
- [ ] Screen capture API

### Mid-term (v0.5)

- [ ] XWayland support
- [ ] Multi-monitor improvements
- [ ] Advanced animations
- [ ] Theme engine

### Long-term (v1.0)

- [ ] Stable API
- [ ] Full Wayland protocol support
- [ ] Mobile/touch optimization
- [ ] Remote desktop support

---

**Last Updated:** April 2026
