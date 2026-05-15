#!/usr/bin/env bash
# scripts/build_release.sh
# ============================================================================
# Release Build Script — Optimizes binary size and prepares for distribution
# ============================================================================
# Usage:
#   ./scripts/build_release.sh [target]
#
# Targets:
#   (none)             → native desktop (auto-detected)
#   android-arm64      → Android ARM64 (phones/tablets)
#   android-arm        → Android ARM32 (older phones)
#   windows-x64        → Windows x86_64
#   linux-x64          → Linux x86_64 (Flatpak ready)
#   linux-arm64        → Linux ARM64 (Raspberry Pi / ARM boards)
#
# Prerequisites:
#   - Rust toolchain (rustup)
#   - cargo-ndk (for Android): cargo install cargo-ndk
#   - Android NDK r25+ in $ANDROID_NDK_HOME
#   - UPX (optional, for extra compression): https://upx.github.io
# ============================================================================

set -euo pipefail

TARGET="${1:-native}"
BINARY_NAME="android-clock"
OUT_DIR="dist"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log() { echo -e "${GREEN}[BUILD]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

mkdir -p "$OUT_DIR"

# ── Set Rust Optimization Flags ───────────────────────────────────────────────
# These env vars work alongside Cargo.toml [profile.release] settings.
export RUSTFLAGS="-C target-cpu=native -C codegen-units=1"

# ── Build by Target ───────────────────────────────────────────────────────────

case "$TARGET" in
    "native"|"")
        log "Building for native desktop platform..."
        cargo build --release
        BINARY_PATH="target/release/$BINARY_NAME"
        ;;

    "android-arm64")
        log "Building for Android ARM64..."
        # cargo-ndk handles the Android toolchain setup automatically.
        # Minimum SDK 26 = Android 8.0 (Oreo)
        cargo ndk \
            --target aarch64-linux-android \
            --android-platform 26 \
            -- build --release
        BINARY_PATH="target/aarch64-linux-android/release/$BINARY_NAME"
        ;;

    "android-arm")
        log "Building for Android ARM32..."
        cargo ndk \
            --target armv7-linux-androideabi \
            --android-platform 26 \
            -- build --release
        BINARY_PATH="target/armv7-linux-androideabi/release/$BINARY_NAME"
        ;;

    "windows-x64")
        log "Building for Windows x64..."
        cargo build --release --target x86_64-pc-windows-gnu
        BINARY_PATH="target/x86_64-pc-windows-gnu/release/${BINARY_NAME}.exe"
        ;;

    "linux-x64")
        log "Building for Linux x64..."
        cargo build --release --target x86_64-unknown-linux-gnu
        BINARY_PATH="target/x86_64-unknown-linux-gnu/release/$BINARY_NAME"
        ;;

    "linux-arm64")
        log "Building for Linux ARM64..."
        cargo build --release --target aarch64-unknown-linux-gnu
        BINARY_PATH="target/aarch64-unknown-linux-gnu/release/$BINARY_NAME"
        ;;

    *)
        error "Unknown target: $TARGET"
        ;;
esac

# ── Post-Processing ───────────────────────────────────────────────────────────

if [ -f "$BINARY_PATH" ]; then
    # Get size before optimization
    BEFORE=$(du -sh "$BINARY_PATH" | cut -f1)
    log "Binary size before optimization: $BEFORE"

    # Strip additional symbols (Cargo.toml already has strip=true,
    # but this catches anything the Rust stripper misses)
    if command -v strip &>/dev/null && [[ "$TARGET" != "windows-x64" ]]; then
        strip --strip-all "$BINARY_PATH" 2>/dev/null || true
    fi

    # Optional: UPX compression for even smaller size
    # WARNING: UPX'd binaries may be flagged by some antivirus on Windows.
    # Only use for Linux/Android distribution.
    if command -v upx &>/dev/null; then
        case "$TARGET" in
            "android-arm64"|"android-arm"|"linux-x64"|"linux-arm64")
                log "Applying UPX compression..."
                upx --best --lzma "$BINARY_PATH" 2>/dev/null || warn "UPX failed (continuing)"
                ;;
            *)
                warn "Skipping UPX for $TARGET (may cause issues)"
                ;;
        esac
    else
        warn "UPX not found. Install it for ~50% additional size reduction."
    fi

    AFTER=$(du -sh "$BINARY_PATH" | cut -f1)
    log "Binary size after optimization: $AFTER"

    # Copy to dist/
    DIST_BINARY="$OUT_DIR/${BINARY_NAME}-${TARGET}"
    [[ "$TARGET" == "windows-x64" ]] && DIST_BINARY="${DIST_BINARY}.exe"
    cp "$BINARY_PATH" "$DIST_BINARY"
    log "Output: $DIST_BINARY"
else
    error "Build failed — binary not found at $BINARY_PATH"
fi

log "✅ Build complete for target: $TARGET"
