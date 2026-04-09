# FlowWM Testing Guide

**Comprehensive guide to testing FlowWM**

---

## 🎯 Testing Strategy

### **Phase 1: Windowed Testing** (Safest)
Test FlowWM in a window on your current desktop - no risk!

### **Phase 2: Virtual Machine** (Safe)
Test on real Wayland in a VM - isolated from main system

### **Phase 3: Real Hardware** (Advanced)
Test on bare metal - full experience but requires backup

---

## 🧪 PHASE 1: Windowed Testing

### **Prerequisites**

```bash
# Install dependencies
sudo pacman -S rust cargo wayland wayland-protocols \
    mesa libglvnd libinput libxkbcommon \
    systemd libdrm pixman winit
```

### **Build with Winit Feature**

```bash
cd "Wayland Compositor/flowwm"

# Build with winit backend
cargo build --release --features winit
```

### **Run in Windowed Mode**

```bash
# From existing Wayland session
./target/release/flowwm

# Or with environment variable
WLR_BACKENDS=winit ./target/release/flowwm
```

### **What to Test**

#### Basic Functionality
- [ ] Compositor starts without crashes
- [ ] Window opens showing FlowWM
- [ ] Can open terminal (SUPER+Return)
- [ ] Can switch workspaces (SUPER+1-9)
- [ ] Can toggle overview (SUPER+Tab)

#### Layout Testing
- [ ] Dynamic layout tiles windows correctly
- [ ] Horizontal layout arranges horizontally
- [ ] Vertical layout arranges vertically
- [ ] Center layout centers main window
- [ ] Can cycle layouts (SUPER+M)

#### Window Management
- [ ] Windows appear in correct positions
- [ ] Borders render around windows
- [ ] Focus follows expected behavior
- [ ] Can close windows (SUPER+Shift+Q)

#### Overview Mode
- [ ] Overview shows workspace thumbnails
- [ ] Can scroll horizontally
- [ ] Can scroll vertically (unique feature!)
- [ ] Can select workspace from overview

#### Performance
- [ ] FPS stays above 60
- [ ] No stuttering or lag
- [ ] Memory usage reasonable (<200MB idle)
- [ ] Animations smooth

### **Expected Issues (and Fixes)**

**Issue:** "Failed to create Wayland display"
```bash
# Solution: Make sure you're in a Wayland session
echo $WAYLAND_DISPLAY
# Should show wayland-0 or similar
```

**Issue:** High CPU usage
```bash
# Solution: Disable animations in config
nano ~/.config/flowwm/config.toml
# Set: enabled = false
```

**Issue:** Windows don't appear
```bash
# Solution: Check logs
RUST_LOG=debug ./target/release/flowwm
```

---

## 🖥️ PHASE 2: Virtual Machine Testing

### **Setup VM**

```bash
# Install QEMU/KVM
sudo pacman -S qemu virt-manager virt-viewer

# Create VM with Arch Linux
# Download Arch ISO: https://archlinux.org/download/

# Create VM with these settings:
# - 4GB RAM minimum
# - 2+ CPU cores
# - Enable 3D acceleration
# - Use VirtIO GPU for best performance
```

### **Install FlowWM in VM**

```bash
# In the VM, install dependencies
sudo pacman -S rust cargo wayland wayland-protocols \
    mesa libglvnd libinput libxkbcommon \
    systemd libdrm pixman

# Clone or copy FlowWM
git clone https://github.com/yourusername/flowwm.git
cd flowwm

# Build
cargo build --release

# Test
./target/release/flowwm
```

### **Test Scenarios**

#### Multi-Monitor (VM with multiple displays)
- [ ] Both monitors detected
- [ ] Can move windows between monitors
- [ ] Workspaces work on each monitor
- [ ] Overview shows all monitors

#### Application Compatibility
- [ ] Terminal apps work (Alacritty, Kitty)
- [ ] Browser works (Firefox, Chromium)
- [ ] File manager works (Nautilus, Dolphin)
- [ ] X11 apps work via XWayland (if enabled)

#### Input Devices
- [ ] Keyboard works fully
- [ ] Mouse/touchpad works
- [ ] Natural scrolling works
- [ ] Touch gestures (if available)

---

## 💻 PHASE 3: Real Hardware Testing

### **⚠️ IMPORTANT: Prepare Backup**

**Before testing on real hardware:**

1. **Keep current compositor installed**
   ```bash
   # Don't uninstall Hyprland/niri/etc yet
   ```

2. **Create recovery plan**
   ```bash
   # Know how to switch to TTY
   # Ctrl+Alt+F2 (or F3-F6)
   
   # Know how to kill FlowWM
   pkill flowwm
   ```

3. **Have fallback session**
   ```bash
   # Make sure your old compositor still works
   # Test you can switch back in display manager
   ```

### **Installation**

```bash
# Install FlowWM
cd "Wayland Compositor/flowwm"
cargo build --release
sudo cp target/release/flowwm /usr/local/bin/

# Install systemd service
sudo cp systemd/flowwm.service /etc/systemd/user/
systemctl --user daemon-reload

# Install desktop entry
cp flowwm.desktop ~/.local/share/wayland-sessions/
```

### **First Boot**

1. **Log out of current session**
2. **Select "FlowWM" from login screen**
3. **Log in**

### **What to Test**

#### Hardware Compatibility
- [ ] GPU detected correctly
- [ ] All outputs (monitors) working
- [ ] Correct resolution and refresh rate
- [ ] No screen tearing

#### Performance
- [ ] Desktop feels responsive
- [ ] Animations smooth (60+ FPS)
- [ ] No input lag
- [ ] Memory usage reasonable

#### Daily Driver Tasks
- [ ] Open terminal and work
- [ ] Browse web
- [ ] Watch videos
- [ ] Code/edit files
- [ ] Multi-tasking smooth

#### Stress Testing
- [ ] Open 10+ windows
- [ ] Multiple workspaces active
- [ ] Heavy applications (browser with many tabs)
- [ ] Video playback while working

### **Monitoring**

```bash
# Check memory usage
ps aux | grep flowwm

# Check logs
journalctl --user -u flowwm -f

# Check FPS (if IPC working)
flowmsg status
```

---

## 📊 Performance Benchmarks

### **FPS Testing**

```bash
# Install mangohud
sudo pacman -S mangohud

# Run with FPS counter
mangohud flowwm
```

**Expected Results:**
- Idle: 144+ FPS (or match monitor refresh rate)
- Normal use: 144+ FPS
- Heavy load: 60+ FPS minimum

### **Memory Testing**

```bash
# Check memory usage
watch -n 1 'ps -o pid,rss,command -p $(pgrep flowwm)'
```

**Expected Results:**
- Idle: 120-180 MB
- 5 windows: 200-300 MB
- 10+ windows: 300-500 MB

### **Input Latency Testing**

```bash
# Install input-latency-tools
sudo pacman -S libinput

# Check input device latency
libinput measure
```

**Expected:** <5ms latency

---

## 🐛 Bug Reporting

### **How to Report Bugs**

1. **Check existing issues** on GitHub
2. **Collect information:**
   ```bash
   # FlowWM version
   flowwm --version
   
   # System info
   uname -a
   
   # GPU info
   glxinfo | grep "OpenGL"
   
   # Logs
   journalctl --user -u flowwm -n 100
   ```

3. **Create detailed report:**
   - What you were doing
   - What happened
   - What you expected
   - Steps to reproduce
   - Logs attached

### **Common Bugs & Fixes**

**Bug:** Crash on startup
```
Likely cause: Missing dependencies
Fix: Install all required packages
```

**Bug:** Black screen
```
Likely cause: DRM backend issue
Fix: Try WLR_BACKENDS=winit
```

**Bug:** Windows not appearing
```
Likely cause: XDG Shell protocol issue
Fix: Check app is Wayland native
```

**Bug:** Keyboard not working
```
Likely cause: libinput/xkbcommon issue
Fix: Reinstall input libraries
```

---

## ✅ Testing Checklist

### **Alpha Testing (Windowed)**

- [ ] Can start in winit window
- [ ] All 4 layouts work
- [ ] Workspace switching works
- [ ] Overview mode works
- [ ] Basic keybindings work
- [ ] No crashes in 1 hour use

### **Beta Testing (VM)**

- [ ] Can run on real Wayland
- [ ] Multi-monitor works
- [ ] Application compatibility good
- [ ] Performance acceptable
- [ ] No major bugs in 1 day use

### **Release Candidate (Hardware)**

- [ ] Can use as daily driver
- [ ] All features work
- [ ] Performance matches Hyprland
- [ ] Stable for 1+ week
- [ ] No data loss or crashes

---

## 📈 Test Results Template

### **System Information**

```
FlowWM Version: 0.1.0
Date: YYYY-MM-DD
Hardware: CPU, GPU, RAM
Distribution: Arch Linux
Kernel: X.X.X
```

### **Test Results**

```
✅ Working:
- Feature 1
- Feature 2

⚠️ Issues:
- Issue 1 (severity: low/medium/high)
- Issue 2

❌ Not Working:
- Feature 3
```

### **Performance**

```
FPS (Idle): XXX
FPS (Load): XXX
Memory (Idle): XXX MB
Memory (Load): XXX MB
Input Latency: XXX ms
```

### **Overall Rating**

```
Stability: ⭐⭐⭐⭐⭐ (5/5)
Performance: ⭐⭐⭐⭐⭐ (5/5)
Features: ⭐⭐⭐⭐⭐ (5/5)
Documentation: ⭐⭐⭐⭐⭐ (5/5)
```

---

## 🎯 Success Criteria

### **Alpha Success**
- ✅ Runs in winit window
- ✅ No crashes in 30 minutes
- ✅ All layouts functional
- ✅ 5+ test scenarios passed

### **Beta Success**
- ✅ Runs on real hardware
- ✅ Daily driver usable
- ✅ 1+ hour without issues
- ✅ 10+ test scenarios passed

### **1.0 Success**
- ✅ Production stable
- ✅ 1+ week daily use
- ✅ Matches Hyprland performance
- ✅ All test scenarios passed

---

## 🆘 Emergency Procedures

### **If FlowWM Freezes**

```bash
# Switch to TTY
Ctrl+Alt+F2

# Login

# Kill FlowWM
pkill flowwm

# Or kill all Wayland sessions
pkill -9 wayland

# Return to GUI
Ctrl+Alt+F1 (or F7)
```

### **If FlowWM Won't Start**

```bash
# Check logs
journalctl --user -u flowwm

# Try with debug logging
RUST_LOG=debug flowwm

# Try winit backend
WLR_BACKENDS=winit flowwm

# Try headless
WLR_BACKENDS=headless flowwm
```

### **If You Can't Log Back In**

```bash
# From TTY, reinstall old compositor
sudo pacman -S hyprland  # or your old WM

# Select old session in display manager
```

---

## 📞 Getting Help

### **Resources**

- **Documentation:** README, CONFIG.md, FAQ.md
- **Logs:** `journalctl --user -u flowwm`
- **GitHub Issues:** Report bugs
- **GitHub Discussions:** Ask questions
- **Discord:** Community support (when available)

### **Contact**

For urgent issues or questions during testing, document everything and report on GitHub with detailed logs.

---

**Happy Testing! 🌊**

*Remember: Start with windowed testing, progress slowly, and always have a backup plan!*

*Last Updated: April 2026*
