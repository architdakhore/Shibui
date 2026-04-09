#!/bin/bash
# FlowWM Installation Script for Arch Linux
# This script installs FlowWM and all required dependencies

set -e

echo "🌊 FlowWM Installation Script"
echo "=============================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if running as root
if [ "$EUID" -eq 0 ]; then 
    echo -e "${RED}Please do not run this script as root${NC}"
    exit 1
fi

# Check if running on Arch Linux
if [ ! -f /etc/arch-release ]; then
    echo -e "${YELLOW}Warning: This script is designed for Arch Linux${NC}"
    echo "Continuing anyway..."
fi

echo "Step 1: Installing build dependencies..."
echo "----------------------------------------"
sudo pacman -S --needed --noconfirm base-devel rust cargo git cmake ninja

echo ""
echo "Step 2: Installing Wayland and graphics dependencies..."
echo "-------------------------------------------------------"
sudo pacman -S --noconfirm wayland wayland-protocols \
    mesa libglvnd libinput libxkbcommon \
    systemd libdrm pixman vulkan-headers

echo ""
echo "Step 3: Cloning FlowWM repository..."
echo "-------------------------------------"
if [ -d "flowwm" ]; then
    echo "FlowWM directory already exists. Updating..."
    cd flowwm
    git pull
else
    git clone https://github.com/yourusername/flowwm.git
    cd flowwm
fi

echo ""
echo "Step 4: Building FlowWM..."
echo "--------------------------"
cargo build --release

echo ""
echo "Step 5: Installing FlowWM..."
echo "----------------------------"
sudo cp target/release/flowwm /usr/local/bin/
sudo chmod +x /usr/local/bin/flowwm

echo ""
echo "Step 6: Installing systemd service..."
echo "--------------------------------------"
mkdir -p ~/.config/systemd/user
cp systemd/flowwm.service ~/.config/systemd/user/
systemctl --user daemon-reload

echo ""
echo "Step 7: Setting up configuration..."
echo "------------------------------------"
mkdir -p ~/.config/flowwm
if [ ! -f ~/.config/flowwm/config.toml ]; then
    cp config/flowwm.toml ~/.config/flowwm/config.toml
    echo "✅ Default configuration created"
else
    echo "ℹ️  Configuration already exists"
fi

echo ""
echo "Step 8: Creating desktop entry..."
echo "----------------------------------"
mkdir -p ~/.local/share/wayland-sessions
cat > ~/.local/share/wayland-sessions/flowwm.desktop << 'EOF'
[Desktop Entry]
Name=FlowWM
Comment=High-performance Wayland compositor
Exec=flowwm
Type=Application
EOF
echo "✅ Desktop entry created"

echo ""
echo -e "${GREEN}✅ Installation complete!${NC}"
echo ""
echo "Next steps:"
echo "1. Log out of your current session"
echo "2. Select 'FlowWM' from your display manager"
echo "3. Log in to start using FlowWM"
echo ""
echo "Or run manually: flowwm"
echo ""
echo "Configuration file: ~/.config/flowwm/config.toml"
echo "Documentation: https://github.com/yourusername/flowwm"
echo ""
