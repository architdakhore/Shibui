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
