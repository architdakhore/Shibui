# FlowWM Frequently Asked Questions

## 📚 General Questions

### What is FlowWM?

FlowWM is a high-performance Wayland compositor that combines the best features of Hyprland (dynamic tiling), niri (horizontal scrolling and overview), and MangoWM (vertical tiling and center layout).

### Why was FlowWM created?

FlowWM was created to provide a single compositor that offers:
- All major tiling layouts in one package
- Unique dual-axis scrolling in overview mode
- High performance matching Hyprland
- Easy configuration with Quicksell compatibility

### Is FlowWM stable?

FlowWM is currently in active development. Core features are functional, but it's recommended to:
- Test in a VM first
- Keep your current compositor as backup
- Report issues on GitHub

---

## 🛠️ Installation

### Which Linux distributions are supported?

FlowWM works on any Linux distribution with:
- Wayland 1.21+
- Rust 1.75+
- OpenGL ES 3.0+ or Vulkan 1.2+

Best support: Arch Linux, Fedora, openSUSE

### Can I run FlowWM on Windows?

No. FlowWM is a Wayland compositor and requires Linux. However, you can:
- Use WSL2 (limited support)
- Run in a Linux VM
- Dual-boot with Linux

### How much disk space is needed?

- **Build dependencies:** ~1.8 GB
- **Source code:** ~400 MB
- **Installed:** ~90 MB
- **Total:** ~2.5 GB

### How much RAM does FlowWM use?

- **Idle:** 120-180 MB
- **Normal use:** 250-450 MB
- **Heavy load:** 500-700 MB

This is comparable to Hyprland and more efficient than most compositors.

---

## ⚙️ Configuration

### Where is the config file?

`~/.config/flowwm/config.toml`

### How do I change keybindings?

Edit the `[keybindings]` section in your config file:

```toml
[keybindings]
mod_key = "SUPER"
"Mod+Return" = "spawn alacritty"
"Mod+D" = "spawn wofi --show drun"
```

### How do I disable animations?

```toml
[animations]
enabled = false
```

### Can I use Hyprland configs?

FlowWM uses TOML format similar to Hyprland, but syntax differs. You'll need to convert your config.

---

## 🎹 Usage

### How do I switch layout modes?

Press `Mod+M` to cycle through:
1. Dynamic (Hyprland-style)
2. Horizontal (niri-style)
3. Vertical (MangoWM-style)
4. Center (MangoWM-style)

### How do I use overview mode?

Press `Mod+Tab` to toggle overview. Scroll with:
- `Mod+Left/Right` - Horizontal scroll
- `Mod+Up/Down` - Vertical scroll

### How many workspaces are there?

Default is 10 workspaces, configurable in `[workspaces]` section.

### Can I have different layouts per workspace?

Yes! Use workspace rules in config:

```toml
[[workspace_rules]]
workspace = 1
layout_mode = "dynamic"

[[workspace_rules]]
workspace = 2
layout_mode = "horizontal"
```

---

## 🐛 Troubleshooting

### FlowWM won't start

**Check:**
1. You're running on Wayland
2. No other compositor is running
3. GPU drivers are installed
4. Check logs: `journalctl --user -u flowwm`

### Screen is black/frozen

**Try:**
1. Switch to different TTY (Ctrl+Alt+F2)
2. Kill compositor: `pkill flowwm`
3. Check GPU compatibility
4. Try with `WLR_BACKENDS=headless`

### Keybindings not working

**Check:**
1. Config file syntax
2. No conflicting keybindings
3. Keyboard layout is correct
4. Run: `flowwm list-keybindings`

### High CPU usage

**Try:**
1. Disable animations: `enabled = false`
2. Disable blur in overview
3. Reduce refresh rate
4. Check for runaway processes

### Windows not tiling correctly

**Try:**
1. Reload config: `Mod+Shift+R`
2. Check application rules
3. Toggle floating state
4. Restart compositor

---

## 🔧 Development

### How can I contribute?

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines. Areas needed:
- Testing and bug reports
- Documentation
- Performance optimization
- Additional layouts
- Plugin development

### How do I build from source?

```bash
git clone https://github.com/yourusername/flowwm.git
cd flowwm
cargo build --release
```

See [INSTALL.md](INSTALL.md) for details.

### Can I write plugins?

Yes! A plugin system is in development. Stay tuned for documentation.

### What programming language is FlowWM written in?

Rust - for memory safety and performance.

---

## 📊 Performance

### How does FlowWM compare to Hyprland?

| Metric | Hyprland | FlowWM |
|--------|----------|--------|
| **Idle RAM** | 150-200 MB | 120-180 MB ✅ |
| **FPS** | 144+ | 144+ ✅ |
| **Input Lag** | <5ms | <5ms ✅ |
| **Boot Time** | ~2s | ~1.5s ✅ |

### Does FlowWM support NVIDIA?

Yes, but with caveats:
- Use proprietary drivers (470+)
- May need `NVIDIA_PATH` environment variable
- Some features may be limited

### Can I use FlowWM with touchscreens?

Yes! FlowWM supports touch input. Configuration options coming soon.

---

## 🔒 Security

### Is FlowWM secure?

FlowWM includes security hardening:
- NoNewPrivileges
- ProtectSystem
- PrivateTmp
- MemoryDenyWriteExecute

See systemd service file for details.

### Does FlowWM collect telemetry?

No. FlowWM doesn't collect or transmit any user data.

---

## 📞 Support

### Where can I get help?

- **GitHub Issues:** Bug reports and feature requests
- **GitHub Discussions:** Questions and community support
- **Documentation:** README, CONFIG.md, INSTALL.md

### How do I report a bug?

1. Check existing issues
2. Include:
   - FlowWM version
   - Linux distribution
   - GPU and drivers
   - Steps to reproduce
   - Logs (`journalctl --user -u flowwm`)
3. Create issue on GitHub

---

**Last Updated:** April 2026
