#!/bin/bash
# Shibui System Requirements Checker
# Verifies all necessary dependencies and permissions for running Shibui

set +e  # Don't exit on errors, we want to check everything

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'  # No Color

ERRORS=0
WARNINGS=0
SUCCESS=0

print_header() {
    echo -e "\n${BLUE}═══════════════════════════════════════${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}═══════════════════════════════════════${NC}\n"
}

check_success() {
    echo -e "${GREEN}✓${NC} $1"
    ((SUCCESS++))
}

check_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
    ((WARNINGS++))
}

check_error() {
    echo -e "${RED}✗${NC} $1"
    ((ERRORS++))
}

print_header "Shibui System Requirements Checker"

# ============================================================================
# 1. KERNEL & LINUX CHECK
# ============================================================================
print_header "1. Kernel & Linux"

# Check if running Linux
if grep -q "Linux" /proc/version; then
    check_success "Running Linux"
    
    # Check kernel version
    KERNEL_VERSION=$(uname -r | cut -d. -f1,2)
    KERNEL_MAJOR=$(echo $KERNEL_VERSION | cut -d. -f1)
    KERNEL_MINOR=$(echo $KERNEL_VERSION | cut -d. -f2)
    
    if [ "$KERNEL_MAJOR" -gt 5 ] || [ "$KERNEL_MAJOR" -eq 5 -a "$KERNEL_MINOR" -ge 0 ]; then
        check_success "Kernel version $KERNEL_VERSION (5.0+ required)"
    else
        check_error "Kernel version too old: $KERNEL_VERSION (need 5.0+)"
    fi
else
    check_error "Not running Linux"
    exit 1
fi

# Check systemd (for session management)
if systemctl --help &>/dev/null; then
    check_success "systemd/systemctl available"
else
    check_warning "systemd not found (may affect session management)"
fi

# ============================================================================
# 2. GRAPHICS & DRM
# ============================================================================
print_header "2. Graphics & DRM Support"

# Check for DRM devices
if [ -d /sys/class/drm ]; then
    check_success "DRM subsystem available"
    
    DRM_CARDS=$(ls /dev/dri/card* 2>/dev/null | wc -l)
    if [ "$DRM_CARDS" -gt 0 ]; then
        check_success "Found $DRM_CARDS DRM device(s): $(ls /dev/dri/card* 2>/dev/null | tr '\n' ' ')"
    else
        check_error "No /dev/dri/card* devices found"
        check_warning "KMS might not be enabled or drivers not loaded"
    fi
else
    check_error "DRM subsystem not available"
fi

# Check for render nodes
if ls /dev/dri/renderD* &>/dev/null; then
    check_success "DRI render nodes available: $(ls /dev/dri/renderD* | tr '\n' ' ')"
else
    check_warning "No DRI render nodes (some Wayland features may be limited)"
fi

# Check graphics drivers
echo ""
if grep -q "intel" /proc/cpuinfo; then
    if grep -q "i915" /proc/modules 2>/dev/null || modinfo i915 &>/dev/null; then
        check_success "Intel graphics module (i915) available"
    else
        check_warning "Intel GPU detected but i915 module might not be loaded"
    fi
fi

if grep -q "amd" /proc/cpuinfo; then
    if grep -q "amdgpu" /proc/modules 2>/dev/null || modinfo amdgpu &>/dev/null; then
        check_success "AMD graphics module (amdgpu) available"
    else
        check_warning "AMD GPU detected but amdgpu module might not be loaded"
    fi
fi

# Check for virtualization (different requirements)
if dmesg | grep -iE "kvm|vbox|vmware|qemu|hyperv" &>/dev/null || [ -f /proc/cpuinfo ] && grep -q "hypervisor" /proc/cpuinfo; then
    check_warning "Running in virtual machine (graphics support may be limited)"
fi

# ============================================================================
# 3. WAYLAND LIBRARIES
# ============================================================================
print_header "3. Wayland Libraries"

# Check pkg-config (tool for checking libraries)
if pkg-config --version &>/dev/null; then
    check_success "pkg-config available"
    
    # Check Wayland libraries
    WAYLAND_LIBS=("wayland-client" "wayland-server" "wayland-protocols" "libxkbcommon" "xkbcommon-keysyms")
    for lib in "${WAYLAND_LIBS[@]}"; do
        if pkg-config --exists "$lib" 2>/dev/null; then
            VERSION=$(pkg-config --modversion "$lib" 2>/dev/null)
            check_success "$lib ($VERSION)"
        else
            check_error "Missing: $lib"
        fi
    done
    
    # Check optional libraries
    echo ""
    OPTIONAL_LIBS=("libinput" "libudev" "libgbm" "libdrm" "libgl" "libegl")
    for lib in "${OPTIONAL_LIBS[@]}"; do
        if pkg-config --exists "$lib" 2>/dev/null; then
            check_success "[Optional] $lib available"
        else
            check_warning "Optional library not found: $lib"
        fi
    done
else
    check_warning "pkg-config not found (cannot verify library versions)"
fi

# ============================================================================
# 4. XWAYLAND (for X11 compatibility)
# ============================================================================
print_header "4. Xwayland (X11 Compatibility - Optional)"

if which Xwayland &>/dev/null; then
    XWAYLAND_VERSION=$(Xwayland -version 2>&1 | head -1)
    check_success "Xwayland available: $XWAYLAND_VERSION"
else
    check_warning "Xwayland not found (X11 apps won't work, but optional)"
fi

# ============================================================================
# 5. RUST & BUILD TOOLS
# ============================================================================
print_header "5. Rust & Build Tools"

# Check Rust
if rustc --version &>/dev/null; then
    RUST_VERSION=$(rustc --version)
    RUST_MAJOR=$(echo $RUST_VERSION | grep -oE '([0-9]+)' | head -1)
    check_success "$RUST_VERSION"
    
    if [ "$RUST_MAJOR" -ge 1 ]; then
        check_success "Rust version compatible (1.70+ required)"
    fi
else
    check_error "Rust not installed"
fi

# Check Cargo
if cargo --version &>/dev/null; then
    check_success "$(cargo --version)"
else
    check_error "Cargo not installed"
fi

# Check build tools
if which gcc &>/dev/null || which cc &>/dev/null; then
    check_success "C compiler available"
else
    check_warning "C compiler not found (may be needed for some dependencies)"
fi

if which pkg-config &>/dev/null; then
    check_success "pkg-config available"
else
    check_warning "pkg-config not found (needed for library detection)"
fi

# ============================================================================
# 6. SESSION MANAGEMENT (systemd-logind or seatd)
# ============================================================================
print_header "6. Session Management"

# Check systemd-logind
if systemctl is-active --quiet systemd-logind; then
    check_success "systemd-logind is running"
    LOGIND_AVAILABLE="yes"
else
    if systemctl is-enabled --quiet systemd-logind 2>/dev/null; then
        check_warning "systemd-logind is installed but not running (start with: sudo systemctl start systemd-logind)"
    else
        check_warning "systemd-logind not available"
    fi
    LOGIND_AVAILABLE="no"
fi

# Check seatd (alternative to logind)
if systemctl is-active --quiet seatd; then
    check_success "seatd is running"
else
    if which seatd &>/dev/null; then
        check_warning "seatd is installed but not running (start with: sudo systemctl start seatd)"
    else
        if [ "$LOGIND_AVAILABLE" = "no" ]; then
            check_error "Neither systemd-logind nor seatd available"
            check_warning "Install one: 'sudo pacman -S seatd' or enable logind"
        fi
    fi
fi

# ============================================================================
# 7. USER PERMISSIONS
# ============================================================================
print_header "7. User Permissions"

CURRENT_USER=$(whoami)
CURRENT_UID=$(id -u)

# Check input group
if groups | grep -q "input"; then
    check_success "User is in 'input' group"
else
    check_error "User NOT in 'input' group (needed for hardware input)"
    check_warning "Fix with: sudo usermod -aG input $CURRENT_USER"
fi

# Check DRM device permissions
if [ -c /dev/dri/card0 ]; then
    if [ -r /dev/dri/card0 ] && [ -w /dev/dri/card0 ]; then
        check_success "Can read/write /dev/dri/card0"
    else
        CARD_OWNER=$(ls -l /dev/dri/card0 | awk '{print $3":"$4}')
        check_error "Cannot access /dev/dri/card0 (owner: $CARD_OWNER)"
        check_warning "Fix by adding user to correct group or checking udev rules"
    fi
fi

# Check render nodes
if [ -c /dev/dri/renderD128 ]; then
    if [ -r /dev/dri/renderD128 ] && [ -w /dev/dri/renderD128 ]; then
        check_success "Can access /dev/dri/renderD128"
    else
        check_warning "Cannot write to /dev/dri/renderD128 (may cause issues)"
    fi
fi

# ============================================================================
# 8. ENVIRONMENT VARIABLES
# ============================================================================
print_header "8. Environment Variables"

# Check XDG_RUNTIME_DIR
if [ -n "$XDG_RUNTIME_DIR" ]; then
    if [ -d "$XDG_RUNTIME_DIR" ] && [ -w "$XDG_RUNTIME_DIR" ]; then
        check_success "XDG_RUNTIME_DIR is set and writable: $XDG_RUNTIME_DIR"
    else
        check_error "XDG_RUNTIME_DIR set but not writable: $XDG_RUNTIME_DIR"
    fi
else
    check_error "XDG_RUNTIME_DIR not set"
    check_warning "Add to ~/.bashrc: export XDG_RUNTIME_DIR=/run/user/\$(id -u)"
fi

# Check other Wayland variables
if [ -n "$WAYLAND_DISPLAY" ]; then
    check_success "WAYLAND_DISPLAY set: $WAYLAND_DISPLAY"
else
    check_warning "WAYLAND_DISPLAY not set (normal before compositor starts)"
fi

# ============================================================================
# 9. OPTIONAL COMPONENTS
# ============================================================================
print_header "9. Optional Components"

# Vulkan support (optional)
if which vulkaninfo &>/dev/null; then
    check_success "Vulkan support available"
else
    check_warning "Vulkan not available (optional for Shibui)"
fi

# OpenGL support
if which glxinfo &>/dev/null; then
    GL_VERSION=$(glxinfo 2>/dev/null | grep -i "OpenGL version" | head -1)
    if [ -n "$GL_VERSION" ]; then
        check_success "OpenGL available: $GL_VERSION"
    fi
else
    check_warning "glxinfo not found (install mesa-utils for graphics diagnostics)"
fi

# ============================================================================
# SUMMARY
# ============================================================================
print_header "Summary"

echo -e "\n${GREEN}✓ Passed: $SUCCESS${NC}"
echo -e "${YELLOW}⚠ Warnings: $WARNINGS${NC}"
echo -e "${RED}✗ Errors: $ERRORS${NC}\n"

if [ "$ERRORS" -eq 0 ]; then
    if [ "$WARNINGS" -eq 0 ]; then
        echo -e "${GREEN}✅ System ready for Shibui installation!${NC}\n"
        exit 0
    else
        echo -e "${YELLOW}⚠  System can run Shibui but some features may be limited${NC}\n"
        echo "Review warnings above and see ARCH_MINIMAL_FIXES.md for solutions."
        exit 0
    fi
else
    echo -e "${RED}❌ System requirements not met${NC}\n"
    echo "Critical errors must be fixed before installing Shibui."
    echo "See ARCH_MINIMAL_FIXES.md for solutions to common problems.\n"
    
    echo "Quick fixes for most common issues:"
    echo "  1. Missing graphics: sudo pacman -S mesa"
    echo "  2. Missing logind:  sudo systemctl start systemd-logind"
    echo "  3. Input group:     sudo usermod -aG input \$USER"
    echo "  4. XDG_RUNTIME_DIR: export XDG_RUNTIME_DIR=/run/user/\$(id -u)"
    
    exit 1
fi
