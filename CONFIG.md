# FlowWM Configuration Guide

Complete guide to configuring FlowWM.

## 📁 Configuration File Location

Default location: `~/.config/flowwm/config.toml`

You can also specify a custom config file:
```bash
flowwm --config /path/to/config.toml
```

Or use environment variable:
```bash
export FLOWWM_CONFIG=/path/to/config.toml
flowwm
```

---

## ⚙️ Configuration Sections

### `[general]` - General Settings

```toml
[general]
name = "FlowWM"                    # Compositor name
layout_mode = "dynamic"            # Layout mode: dynamic, horizontal, vertical, center
log_level = "info"                 # Log level: debug, info, warn, error
```

### `[tiling]` - Tiling Settings

```toml
[tiling]
gap_size = 8                       # Gap between windows (pixels)
border_width = 2                   # Window border width (pixels)
border_color = "#4C7899"          # Default border color (hex)
active_border_color = "#57A0C9"   # Active window border color
master_ratio = 0.55               # Master area size ratio (0.0 - 1.0)
center_ratio = 0.6                # Center window size ratio (0.0 - 1.0)
window_width = 800                # Default window width for horizontal layout
window_height = 600               # Default window height for vertical layout
```

### `[workspaces]` - Workspace Settings

```toml
[workspaces]
count = 10                         # Number of workspaces
scroll_mode = "both"              # Scroll mode: horizontal, vertical, both
```

### `[animations]` - Animation Settings

```toml
[animations]
enabled = true                     # Enable/disable animations
duration_ms = 250                 # Animation duration (milliseconds)
curve = "ease-out-expo"           # Animation curve:
                                   #   - linear
                                   #   - ease-in
                                   #   - ease-out
                                   #   - ease-in-out
                                   #   - ease-out-expo
                                   #   - spring
```

### `[overview]` - Overview Mode Settings

```toml
[overview]
scale = 0.15                      # Overview scale (0.1 - 0.5)
gap = 20                          # Gap between workspaces in overview (pixels)
blur = true                       # Enable blur effect
scroll_direction = "both"         # Scroll direction: horizontal, vertical, both
```

### `[input]` - Input Settings

```toml
[input]
follow_mouse = true               # Focus follows mouse
natural_scroll = true             # Natural scrolling direction
repeat_rate = 25                  # Key repeat rate (keys/second)
repeat_delay = 300                # Key repeat delay (milliseconds)
```

### `[keybindings]` - Keybindings

```toml
[keybindings]
mod_key = "SUPER"                 # Modifier key: SUPER, CTRL, ALT

# Format: "key" = "action"
"Mod+Return" = "spawn terminal"
"Mod+D" = "spawn launcher"
"Mod+Tab" = "toggle overview"
"Mod+H" = "focus left"
"Mod+J" = "focus down"
"Mod+K" = "focus up"
"Mod+L" = "focus right"
"Mod+Shift+Q" = "close window"
"Mod+1" = "workspace 1"
"Mod+2" = "workspace 2"
# ... etc
```

### `[render]` - Rendering Settings

```toml
[render]
vsync = true                      # Enable VSync
hdr = false                       # Enable HDR (experimental)
backend = "opengl"                # Render backend: opengl, vulkan
```

---

## 🎹 Keybinding Syntax

### Modifier Keys

- `SUPER` - Windows/Command key
- `CTRL` - Control key
- `ALT` - Alt key
- `SHIFT` - Shift key

### Combining Modifiers

```toml
"Mod+Shift+Q" = "close window"     # SUPER + SHIFT + Q
"Ctrl+Alt+D" = "spawn launcher"    # CTRL + ALT + D
```

### Available Actions

#### Window Management
- `focus left/right/up/down` - Focus adjacent window
- `move left/right/up/down` - Move window in direction
- `close window` - Close focused window
- `toggle floating` - Toggle window floating state
- `toggle fullscreen` - Toggle window fullscreen

#### Layout
- `cycle layout` - Cycle through layout modes
- `cycle center pattern` - Cycle center layout patterns
- `increase master ratio` - Increase master area size
- `decrease master ratio` - Decrease master area size

#### Workspaces
- `workspace N` - Switch to workspace N (1-10)
- `movetoworkspace N` - Move window to workspace N
- `next workspace` - Go to next workspace
- `previous workspace` - Go to previous workspace

#### Scrolling
- `scroll left/right` - Scroll horizontally
- `scroll up/down` - Scroll vertically

#### System
- `spawn <command>` - Execute command
- `toggle overview` - Toggle overview mode
- `reload config` - Reload configuration
- `quit` - Exit compositor

---

## 🎨 Theme Configuration

Create theme files in `~/.config/flowwm/themes/`:

```toml
# ~/.config/flowwm/themes/dark.toml
name = "Dark Theme"

[colors]
background = "#1a1a1a"
foreground = "#ffffff"
border = "#4C7899"
active_border = "#57A0C9"
urgent_border = "#C94C4C"

[overview]
background = "#2a2a2a"
workspace_bg = "#3a3a3a"
workspace_active = "#57A0C9"
```

Load theme in main config:
```toml
[general]
theme = "~/.config/flowwm/themes/dark.toml"
```

---

## 📝 Configuration Examples

### Example 1: Minimal Configuration

```toml
[general]
layout_mode = "dynamic"

[tiling]
gap_size = 4
border_width = 1

[animations]
enabled = false
```

### Example 2: Productivity Setup

```toml
[tiling]
gap_size = 8
master_ratio = 0.65
border_width = 2

[workspaces]
count = 5

[overview]
scale = 0.2
blur = false

[keybindings]
"Mod+E" = "spawn editor"
"Mod+W" = "spawn browser"
"Mod+T" = "spawn terminal"
```

### Example 3: Gaming Setup

```toml
[general]
layout_mode = "dynamic"

[tiling]
gap_size = 0
border_width = 0

[animations]
enabled = false

[render]
vsync = false
```

---

## 🔄 Reloading Configuration

### Method 1: Keybinding

Press `Mod+Shift+R` (if configured) to reload config.

### Method 2: Command

```bash
flowwm reload-config
```

### Method 3: Restart Session

Log out and log back in.

---

## 🐛 Troubleshooting

### Config Not Loading

Check config file syntax:
```bash
flowwm --validate-config
```

### Keybindings Not Working

Check for conflicts:
```bash
flowwm list-keybindings
```

### Reset to Defaults

```bash
rm ~/.config/flowwm/config.toml
flowwm  # Will create default config
```

---

## 📚 Advanced Configuration

### Per-Workspace Settings

```toml
[[workspace_rules]]
workspace = 1
layout_mode = "dynamic"
master_ratio = 0.6

[[workspace_rules]]
workspace = 2
layout_mode = "horizontal"
```

### Application Rules

```toml
[[application_rules]]
app_id = "firefox"
floating = true
workspace = 2

[[application_rules]]
app_id = "steam"
fullscreen = true
workspace = 3
```

---

**Last Updated:** April 2026
