#!/bin/bash
# FlowWM - Super Easy One-Command Installation
# For Arch Linux

set -e

echo ""
echo "🌊 FlowWM - One-Command Installer"
echo "================================="
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Check if root
if [ "$EUID" -eq 0 ]; then
    echo -e "${RED}❌ Don't run as root!${NC}"
    exit 1
fi

# Check if Arch
if [ ! -f /etc/arch-release ]; then
    echo -e "${YELLOW}⚠️  This is for Arch Linux${NC}"
    echo "Other distros need manual dependency installation"
    echo ""
    read -p "Continue? (y/N): " cont
    [[ ! $cont =~ ^[Yy]$ ]] && exit 1
fi

echo -e "${BLUE}📦 Installing dependencies...${NC}"
sudo pacman -S --needed --noconfirm base-devel rust git cmake ninja \
    wayland wayland-protocols mesa libglvnd libinput libxkbcommon \
    systemd libdrm pixman 2>&1 | grep -v ":: Running" || true

echo -e "${GREEN}✅ Dependencies installed${NC}"
echo ""

echo -e "${BLUE}📥 Getting FlowWM...${NC}"
if [ -d "flowwm" ]; then
    cd flowwm
    git pull
else
    git clone https://github.com/yourusername/flowwm.git
    cd flowwm
fi

echo -e "${GREEN}✅ Source ready${NC}"
echo ""

echo -e "${BLUE}🔨 Building...${NC}"
echo "This takes 5-10 minutes..."
cargo build --release 2>&1 | tail -3

echo -e "${GREEN}✅ Build complete${NC}"
echo ""

echo -e "${BLUE}📦 Installing ShibUI...${NC}"
sudo cp target/release/shibui /usr/local/bin/
sudo cp target/release/shibuictl /usr/local/bin/ 2>/dev/null || true
sudo chmod +x /usr/local/bin/shibui

mkdir -p ~/.config/shibui
[ ! -f ~/.config/shibui/config.toml ] && cp config/shibui.toml ~/.config/shibui/

mkdir -p ~/.local/share/wayland-sessions
cp shibui.desktop ~/.local/share/wayland-sessions/

mkdir -p ~/.config/systemd/user
cp systemd/shibui.service ~/.config/systemd/user/
systemctl --user daemon-reload 2>/dev/null || true

echo -e "${GREEN}✅ ShibUI installation complete!${NC}"
echo ""
echo "╔══════════════════════════════════════╗"
echo "║   🎉 ShibUI is ready to use! 🎉     ║"
echo "╚══════════════════════════════════════╝"
echo ""
echo "📍 Binary: /usr/local/bin/shibui"
echo "📝 Config: ~/.config/shibui/config.toml"
echo ""
echo "🎯 Next steps:"
echo "  1. Test: ./scripts/first-run.sh"
echo "  2. Or logout & select ShibUI"
echo ""
echo "⌨️  Keys: SUPER+Return, SUPER+Tab, SUPER+1-9"
echo ""
