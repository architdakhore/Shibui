#!/bin/bash
# FlowWM Build Script
# Builds FlowWM with various configurations

set -e

echo "🌊 FlowWM Build Script"
echo "======================"
echo ""

# Colors
GREEN='\033[0;32m'
NC='\033[0m'

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Please install Rust first."
    echo "   Visit: https://rustup.rs/"
    exit 1
fi

echo "Rust version: $(rustc --version)"
echo "Cargo version: $(cargo --version)"
echo ""

# Build type
case "${1:-release}" in
    debug)
        echo "Building debug version..."
        cargo build
        echo -e "${GREEN}✅ Debug build complete: target/debug/flowwm${NC}"
        ;;
    release)
        echo "Building release version..."
        cargo build --release
        echo -e "${GREEN}✅ Release build complete: target/release/flowwm${NC}"
        ;;
    test)
        echo "Running tests..."
        cargo test
        echo -e "${GREEN}✅ Tests complete${NC}"
        ;;
    bench)
        echo "Running benchmarks..."
        cargo bench
        echo -e "${GREEN}✅ Benchmarks complete${NC}"
        ;;
    clean)
        echo "Cleaning build artifacts..."
        cargo clean
        echo -e "${GREEN}✅ Clean complete${NC}"
        ;;
    *)
        echo "Usage: $0 [debug|release|test|bench|clean]"
        echo ""
        echo "  debug   - Build debug version (default)"
        echo "  release - Build optimized release version"
        echo "  test    - Run tests"
        echo "  bench   - Run benchmarks"
        echo "  clean   - Remove build artifacts"
        exit 1
        ;;
esac

echo ""
