# 🚀 Shibui Installation Guide

## Complete Step-by-Step Guide for Arch Linux

---

## 📋 Prerequisites

- Arch Linux (minimal desktop installation)
- Internet connection
- sudo privileges
- At least 2GB free disk space
- 4GB RAM recommended

---

## 🔧 Step 1: Update Your System

Boot into Arch Linux (TTY or desktop), then:

```bash
sudo pacman -Syu
```

Press `Y` when prompted to confirm updates.

**Wait for completion** - This ensures all packages are up to date.

---

## 📦 Step 2: Install Build Dependencies

```bash
sudo pacman -S --needed base-devel rust cargo git cmake ninja \
    wayland wayland-protocols mesa libglvnd libinput libxkbcommon \
    systemd libdrm pixman vulkan-headers
```

**What this installs:**
- `rust` and `cargo` - Rust programming language
- `wayland` - Wayland display server libraries
- `mesa` and `libglvnd` - OpenGL graphics
- `libinput` and `libxkbcommon` - Input handling
- Other required libraries

**Wait for completion** - This may take 5-10 minutes.

---

## 👥 Step 3: Add User to Video Group

```bash
sudo usermod -a -G video $USER
```

**Important:** You must **log out and log back in** (or reboot) for this to take effect.

To verify:
```bash
groups
```

You should see `video` in the list.

---

## 📥 Step 4: Clone Shibui Repository

```bash
git clone https://github.com/architdakhore/Shibui.git
cd Shibui
```

This downloads the Shibui source code to your home directory.

---

## 🔨 Step 5: Build Shibui

```bash
cargo build --release
```

**What happens:**
- Downloads Rust dependencies (~200 crates)
- Compiles Shibui (~5-10 minutes on modern hardware)
- Creates optimized binary in `target/release/shibui`

**Wait for completion** - First build takes longest.

**Expected output:**
```
   Compiling shibui v0.1.0
   Finished release [optimized] target(s) in 2m 30s
```

---

## 📤 Step 6: Install Binary

```bash
sudo cp target/release/shibui /usr/local/bin/
sudo chmod +x /usr/local/bin/shibui
```

This copies the binary to a system-wide location.

**Verify installation:**
```bash
which shibui
shibui --version
```

---

## ⚙️ Step 7: Setup Configuration

```bash
mkdir -p ~/.config/shibui
cp config/shibui.toml ~/.config/shibui/
```

This creates your personal configuration directory.

**Verify:**
```bash
ls -la ~/.config/shibui/
```

You should see `config.toml`.

---

## 🖥️ Step 8: Configure VirtualBox (If Using VM)

**Shut down your VM**, then in VirtualBox Manager:

1. **Right-click VM → Settings → Display**
2. Set **Video Memory** to **128 MB** (or higher)
3. Set **Graphics Controller** to **VMSVGA**
4. ✅ **Enable 3D Acceleration** (REQUIRED!)
5. Click **OK**

**Start your VM** after making these changes.

---

## 🚀 Step 9: Test Shibui

### Option A: Run from TTY (Recommended for Testing)

1. **Switch to TTY1:**
   - Press `Ctrl + Alt + F1`
   - You'll see a black screen with login prompt

2. **Login:**
   - Enter your username
   - Enter your password

3. **Run Shibui:**
   ```bash
   shibui
   ```

**Expected result:** Shibui starts, you'll see a blank screen with a cursor.

### Option B: Use as Default Wayland Session

1. **Create desktop entry:**
   ```bash
   sudo nano /usr/share/wayland-sessions/shibui.desktop
   ```

2. **Paste this content:**
   ```ini
   [Desktop Entry]
   Name=Shibui
   Comment=Shibui Wayland Compositor
   Exec=/usr/local/bin/shibui
   Type=Application
   DesktopNames=Shibui
   ```

3. **Save and exit:**
   - Press `Ctrl + X`
   - Press `Y`
   - Press `Enter`

4. **Use display manager:**
   - If you have GDM/LightDM/SDDM installed
   - Select "Shibui" from session menu at login

### Option C: Test with Winit Backend (Windowed Mode)

If you're already running another Wayland/X11 session:

```bash
shibui --backend winit
```

This runs Shibui in a window for testing.

---

## ⌨️ Step 10: Test Keybindings

Once Shibui is running, test these:

| Key | Action |
|-----|--------|
| `Super + Return` | Open terminal (kitty/alacritty) |
| `Super + D` | Open launcher (wofi/rofi) |
| `Super + Tab` | Toggle overview |
| `Super + M` | Cycle layouts |
| `Super + H/J/K/L` | Focus windows |
| `Super + Shift + Q` | Close window |
| `Super + 1-9` | Switch workspaces |

**Note:** You need a terminal emulator and launcher installed:
```bash
sudo pacman -S kitty wofi
```

---

## 🔍 Step 11: Verify Installation

### Check Binary
```bash
which shibui
# Should output: /usr/local/bin/shibui

shibui --help
# Should show help message
```

### Check Config
```bash
ls -la ~/.config/shibui/config.toml
# Should show config file
```

### Check Logs
```bash
journalctl -xe | grep shibui
# Should show Shibui startup logs
```

---

## 🐛 Troubleshooting

### Issue: "No DRM device found"

**Solution:**
```bash
# Check if DRM devices exist
ls -la /dev/dri/

# Load GPU drivers (Intel)
sudo modprobe i915

# Load GPU drivers (AMD)
sudo modprobe amdgpu

# Load GPU drivers (NVIDIA - requires proprietary drivers)
sudo modprobe nvidia
```

### Issue: Permission denied errors

**Solution:**
```bash
# Verify you're in video group
groups

# If not, add yourself and reboot
sudo usermod -a -G video $USER
sudo reboot
```

### Issue: Black screen or crash

**Solutions:**
1. **Enable 3D acceleration** in VirtualBox (see Step 8)
2. **Increase video memory** to 128MB+
3. **Try Winit backend:**
   ```bash
   shibui --backend winit
   ```
4. **Check logs:**
   ```bash
   journalctl -xe | grep shibui
   ```

### Issue: Config not reloading

**Solution:**
```bash
# Check file watcher permissions
ls -la ~/.config/shibui/config.toml

# Ensure readable
chmod 644 ~/.config/shibui/config.toml

# Check logs for errors
journalctl -f | grep shibui
```

### Issue: Input not working

**Solution:**
```bash
# Check input devices
libinput list-devices

# Ensure libinput is installed
sudo pacman -S libinput
```

---

## 📝 Post-Installation

### Install Recommended Packages

```bash
# Terminal emulator
sudo pacman -S kitty

# Application launcher
sudo pacman -S wofi

# Screenshot tool
sudo pacman -S grim slurp

# System monitor
sudo pacman -S htop
```

### Customize Configuration

Edit `~/.config/shibui/config.toml`:

```bash
nano ~/.config/shibui/config.toml
```

See CONFIG.md for configuration options.

### Enable Systemd Service (Optional)

```bash
# Copy service file
sudo cp systemd/shibui.service /etc/systemd/user/

# Enable service
systemctl --user enable shibui.service

# Start service
systemctl --user start shibui.service
```

---

## 🎯 Next Steps

1. **Test all layouts:**
   - Press `Super + M` to cycle through Dwindle, Horizontal, Vertical, Center, Floating

2. **Test overview:**
   - Press `Super + Tab` to see all workspaces

3. **Test floating windows:**
   - Switch to floating layout
   - Use `Super + Left Click` to drag windows
   - Use `Super + Right Click` to resize

4. **Customize:**
   - Edit `~/.config/shibui/config.toml`
   - Changes apply automatically (hot reload!)

5. **Report issues:**
   - GitHub Issues: https://github.com/architdakhore/Shibui/issues

---

## ✅ Verification Checklist

- [ ] System updated (`sudo pacman -Syu`)
- [ ] Dependencies installed
- [ ] User in video group
- [ ] Repository cloned
- [ ] Build successful (`cargo build --release`)
- [ ] Binary installed (`/usr/local/bin/shibui`)
- [ ] Config created (`~/.config/shibui/config.toml`)
- [ ] VirtualBox configured (if using VM)
- [ ] Shibui starts successfully
- [ ] Keybindings work
- [ ] All 5 layouts functional
- [ ] Overview mode works
- [ ] Config hot reload works

---

## 📞 Support

- **Documentation:** See README.md and FAQ.md
- **Issues:** https://github.com/architdakhore/Shibui/issues
- **Discussions:** https://github.com/architdakhore/Shibui/discussions

---

## 🎉 Success!

If you've completed all steps, **Shibui is now installed and running!**

Enjoy your new Wayland compositor with **5 layout engines**, **hot config reload**, and **dual-axis overview**!

---

**Shibui (渋い)** - Simple, Subtle, and Unobtrusive Beauty ✨
# FlowWM Installation Guide

This guide covers installing FlowWM on various Linux distributions.

## 📋 Prerequisites

### Required Dependencies

- **Rust** (1.75 or later)
- **Wayland** (1.21 or later)
- **OpenGL ES 3.0+** or **Vulkan 1.2+**
- **libinput**
- **libxkbcommon**
- **systemd** (for session management)
- **pixman**
- **libdrm**

### Optional Dependencies

- **Vulkan** (for Vulkan renderer)
- **pipewire** (for screen capture)
- **polkit** (for authentication dialogs)

---

## 🏗️ Arch Linux (Recommended)

### Method 1: AUR Package (Coming Soon)

```bash
# Using yay
yay -S flowwm

# Using paru
paru -S flowwm
```

### Method 2: Manual Installation

```bash
# 1. Install build dependencies
sudo pacman -S --needed base-devel rust cargo git cmake ninja

# 2. Install Wayland and graphics dependencies
sudo pacman -S wayland wayland-protocols \
    mesa libglvnd libinput libxkbcommon \
    systemd libdrm pixman vulkan-headers

# 3. Clone the repository
git clone https://github.com/yourusername/flowwm.git
cd flowwm

# 4. Build the compositor
cargo build --release

# 5. Install (optional)
sudo cp target/release/flowwm /usr/local/bin/
sudo chmod +x /usr/local/bin/flowwm

# 6. Install systemd service
sudo cp systemd/flowwm.service /etc/systemd/user/
systemctl --user daemon-reload
```

### Method 3: Using Install Script

```bash
# Run the installation script
./scripts/install.sh

# This will:
# - Install dependencies
# - Build FlowWM
# - Install binaries and services
# - Configure environment
```

---

## 🐧 Fedora

```bash
# Install dependencies
sudo dnf install rust cargo wayland-devel wayland-protocols-devel \
    mesa-libGL-devel libinput-devel libxkbcommon-devel \
    systemd-devel pixman-devel libdrm-devel

# Build
git clone https://github.com/yourusername/flowwm.git
cd flowwm
cargo build --release

# Install
sudo cp target/release/flowwm /usr/local/bin/
```

---

## 🐧 Ubuntu/Debian

```bash
# Install dependencies
sudo apt install rustc cargo pkg-config cmake ninja-build \
    libwayland-dev wayland-protocols \
    libgl1-mesa-dev libgles2-mesa-dev \
    libinput-dev libxkbcommon-dev \
    libsystemd-dev libpixman-1-dev libdrm-dev

# Build
git clone https://github.com/yourusername/flowwm.git
cd flowwm
cargo build --release

# Install
sudo cp target/release/flowwm /usr/local/bin/
```

---

## 🐧 openSUSE

```bash
# Install dependencies
sudo zypper install rust cargo wayland-devel wayland-protocols-devel \
    Mesa-libGL-devel libinput-devel libxkbcommon-devel \
    systemd-devel pixman-devel libdrm-devel

# Build
git clone https://github.com/yourusername/flowwm.git
cd flowwm
cargo build --release

# Install
sudo cp target/release/flowwm /usr/local/bin/
```

---

## 🔧 Post-Installation

### 1. Create Configuration Directory

```bash
mkdir -p ~/.config/flowwm
cp config/flowwm.toml ~/.config/flowwm/config.toml
```

### 2. Configure Your Session

Add to your display manager or `~/.xinitrc`:

```bash
exec flowwm
```

Or create a systemd user service:

```bash
systemctl --user enable flowwm.service
systemctl --user start flowwm.service
```

### 3. Verify Installation

```bash
# Check version
flowwm --version

# Check help
flowwm --help

# Run with debug logging
RUST_LOG=debug flowwm
```

---

## 🧪 Testing

### Run in Windowed Mode (for testing)

FlowWM can run in a window using winit backend for testing:

```bash
# Build with winit feature
cargo build --release --features winit

# Run
./target/release/flowwm
```

### Performance Testing

```bash
# Run performance benchmarks
cargo bench

# Memory profiling
valgrind --tool=massif ./target/release/flowwm
```

---

## ❓ Troubleshooting

### Build Errors

**Error: `wayland-server.h: No such file or directory`**
```bash
# Install Wayland development files
sudo pacman -S wayland  # Arch
sudo dnf install wayland-devel  # Fedora
sudo apt install libwayland-dev  # Ubuntu
```

**Error: `cannot find -lGLESv2`**
```bash
# Install OpenGL ES
sudo pacman -S libglvnd  # Arch
sudo dnf install libglvnd-devel  # Fedora
sudo apt install libgles2-mesa-dev  # Ubuntu
```

### Runtime Errors

**Error: `Failed to create Wayland display`**
- Ensure you're running on a Wayland session
- Check that no other compositor is running
- Try running with `WLR_BACKENDS=headless`

**Error: `Failed to initialize input`**
- Ensure you have permissions to access input devices
- Add your user to the `input` group: `sudo usermod -aG input $USER`

---

## 📦 Uninstallation

### Manual Installation

```bash
sudo rm /usr/local/bin/flowwm
sudo rm /etc/systemd/user/flowwm.service
systemctl --user daemon-reload
rm -rf ~/.config/flowwm
```

### AUR Installation

```bash
yay -R flowwm  # or paru -R flowwm
```

---

## 🔄 Updating

```bash
# Pull latest changes
git pull origin main

# Rebuild
cargo build --release

# Reinstall
sudo cp target/release/flowwm /usr/local/bin/
```

---

## 📞 Need Help?

- Check the [FAQ](FAQ.md)
- Open an [issue](https://github.com/yourusername/flowwm/issues)
- Join [discussions](https://github.com/yourusername/flowwm/discussions)

---

**Last Updated:** April 2026
