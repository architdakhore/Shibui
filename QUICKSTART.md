# FlowWM Quick Start Guide

**Get FlowWM running in 5 minutes!**

---

## ⚡ Quick Installation

### For Arch Linux

```bash
# 1. Install dependencies
sudo pacman -S rust cargo wayland wayland-protocols \
    mesa libglvnd libinput libxkbcommon \
    systemd libdrm pixman

# 2. Clone and build
cd "Wayland Compositor/flowwm"
cargo build --release

# 3. Test in windowed mode (recommended first!)
WLR_BACKENDS=headless cargo run --release
```

---

## 🧪 First Test (Windowed Mode)

**Safest way to test** - run FlowWM in a window on your current desktop:

```bash
# Build with winit feature
cargo build --release --features winit

# Run in windowed mode
./target/release/flowwm
```

This opens FlowWM in a window, letting you test without logging out!

---

## 🖥️ Full Session Test

**When ready for real hardware:**

### Option 1: From Display Manager

1. Log out of current session
2. Select "FlowWM" from login screen
3. Log in

### Option 2: From TTY

```bash
# Switch to TTY (Ctrl+Alt+F2)
# Login, then:
flowwm
```

### Option 3: Using systemd

```bash
# Enable FlowWM service
systemctl --user enable flowwm.service

# Start FlowWM
systemctl --user start flowwm.service
```

---

## ⌨️ Default Keybindings

| Key | Action |
|-----|--------|
| `SUPER+Return` | Open terminal |
| `SUPER+D` | Open launcher |
| `SUPER+Tab` | Toggle overview |
| `SUPER+HJKL` | Focus windows |
| `SUPER+Shift+Q` | Close window |
| `SUPER+1-9` | Switch workspace |
| `SUPER+M` | Cycle layouts |

---

## 🎯 Test Checklist

### Basic Functionality

- [ ] Can start compositor
- [ ] Can open terminal (SUPER+Return)
- [ ] Can switch workspaces (SUPER+1-9)
- [ ] Can toggle overview (SUPER+Tab)
- [ ] Can close windows (SUPER+Shift+Q)

### Layout Testing

- [ ] Dynamic layout works
- [ ] Horizontal layout works
- [ ] Vertical layout works
- [ ] Center layout works
- [ ] Can cycle layouts (SUPER+M)

### Advanced Features

- [ ] Overview mode shows all workspaces
- [ ] Can scroll overview horizontally
- [ ] Can scroll overview vertically
- [ ] Animations are smooth
- [ ] Borders render correctly

---

## 🐛 Troubleshooting

### "Failed to create Wayland display"

**Solution:** You're probably running from within another Wayland session. Use:

```bash
WLR_BACKENDS=headless cargo run --release
```

### "No DRM device found"

**Solution:** Install proper GPU drivers:

```bash
# Intel
sudo pacman -S mesa vulkan-intel

# AMD
sudo pacman -S mesa vulkan-radeon

# NVIDIA
sudo pacman -S nvidia nvidia-utils
```

### Black screen

**Solution:** Try different backend:

```bash
# Use Winit backend for testing
cargo run --release --features winit
```

### High CPU usage

**Solution:** Disable animations in config:

```toml
[animations]
enabled = false
```

---

## 📝 Configuration Quick Edit

```bash
# Edit config
nano ~/.config/flowwm/config.toml

# Reload config
SUPER+Shift+R  # (if configured)
# or restart session
```

### Quick Settings

```toml
# Change layout mode
[general]
layout_mode = "dynamic"  # or horizontal, vertical, center

# Change gaps
[tiling]
gap_size = 8

# Disable animations for performance
[animations]
enabled = false
```

---

## 📊 Performance Check

```bash
# Check FPS (if IPC is working)
flowmsg status

# Check memory usage
ps aux | grep flowwm
```

**Expected:**
- Idle: 120-180 MB
- FPS: 60+ (144+ on good hardware)

---

## 🆘 Getting Help

### Logs

```bash
# View logs
journalctl --user -u flowwm -f

# Or run with debug logging
RUST_LOG=debug flowwm
```

### Report Issues

1. Check existing issues on GitHub
2. Include:
   - FlowWM version
   - Linux distro
   - GPU and drivers
   - Steps to reproduce
   - Logs

---

## 🎉 Success!

If you got FlowWM running, **congratulations!** 🎊

You're now testing a compositor that took serious development effort!

### Next Steps:

1. **Customize** - Edit `~/.config/flowwm/config.toml`
2. **Test** - Try all features and layouts
3. **Report** - File bugs or feedback on GitHub
4. **Share** - Show screenshots on r/unixporn!

---

## 🔄 Updates

```bash
# Pull latest changes
git pull origin main

# Rebuild
cargo build --release

# Restart
# (log out and back in, or restart session)
```

---

**Happy Tiling!** 🌊

*Last Updated: April 2026*
