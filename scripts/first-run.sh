#!/bin/bash
# FlowWM - First Run Script
# This script helps you test FlowWM for the first time

set -e

echo "🌊 FlowWM First Run Script"
echo "=========================="
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Check if on Arch Linux
if [ ! -f /etc/arch-release ]; then
    echo -e "${YELLOW}Warning: This script is for Arch Linux${NC}"
    echo "You may need to adjust for your distribution"
    echo ""
fi

# Step 1: Check dependencies
echo "Step 1: Checking dependencies..."
echo "---------------------------------"

check_dep() {
    if command -v $1 &> /dev/null; then
        echo -e "✅ $1"
    else
        echo -e "❌ $1 (missing)"
        MISSING_DEPS="$MISSING_DEPS $1"
    fi
}

check_dep rustc
check_dep cargo
check_dep wayland
check_dep pkg-config

if [ ! -z "$MISSING_DEPS" ]; then
    echo ""
    echo -e "${RED}Missing dependencies:$MISSING_DEPS${NC}"
    echo "Install with: sudo pacman -S rust cargo wayland"
    exit 1
fi

echo ""
echo "Step 2: Building FlowWM..."
echo "--------------------------"

# Build
cargo build --release

echo ""
echo -e "${GREEN}✅ Build successful!${NC}"
echo ""

# Step 3: Create config directory
echo "Step 3: Setting up configuration..."
echo "------------------------------------"

mkdir -p ~/.config/flowwm

if [ ! -f ~/.config/flowwm/config.toml ]; then
    cp config/flowwm.toml ~/.config/flowwm/config.toml
    echo "✅ Created default configuration"
else
    echo "ℹ️  Configuration already exists"
fi

echo ""

# Step 4: Test mode selection
echo "Step 4: Choose test mode..."
echo "---------------------------"
echo ""
echo "1. Windowed mode (safest - test in a window)"
echo "2. Headless mode (no display, for testing)"
echo "3. Real hardware (full Wayland session)"
echo ""
read -p "Enter choice [1-3]: " choice

echo ""
case $choice in
    1)
        echo "🪟 Starting FlowWM in windowed mode..."
        echo "This will open FlowWM in a window on your current desktop"
        echo ""
        echo "Controls:"
        echo "  SUPER+Return - Open terminal"
        echo "  SUPER+Tab    - Toggle overview"
        echo "  SUPER+1-9    - Switch workspaces"
        echo "  SUPER+M      - Cycle layouts"
        echo "  SUPER+Shift+Q - Close window"
        echo ""
        echo "Press Ctrl+C to exit"
        echo ""
        WLR_BACKENDS=winit ./target/release/flowwm
        ;;
    2)
        echo "🔮 Starting FlowWM in headless mode..."
        echo "This runs without display (for automated testing)"
        echo ""
        WLR_BACKENDS=headless ./target/release/flowwm
        ;;
    3)
        echo "💻 Starting FlowWM on real hardware..."
        echo ""
        echo "⚠️  WARNING: This will take over your screen!"
        echo "⚠️  Make sure you have a backup compositor installed"
        echo ""
        read -p "Are you sure? (y/N): " confirm
        if [ "$confirm" = "y" ] || [ "$confirm" = "Y" ]; then
            echo ""
            echo "Starting FlowWM..."
            echo "Press Ctrl+Alt+F2 to switch to TTY if you need to exit"
            echo ""
            ./target/release/flowwm
        else
            echo "Aborted"
            exit 0
        fi
        ;;
    *)
        echo "Invalid choice"
        exit 1
        ;;
esac

echo ""
echo "✅ FlowWM session ended"
echo ""
echo "How did it go?"
echo "Report issues at: https://github.com/yourusername/flowwm/issues"
echo ""
