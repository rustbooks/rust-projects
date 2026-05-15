#!/usr/bin/env bash
# scripts/build_release.sh
# Builds release binaries for all desktop platforms and Android.
# Run from the project root.
# Usage:
#   ./scripts/build_release.sh [desktop|android|all]

set -euo pipefail

TARGET="${1:-desktop}"
BINARY_NAME="contacts-app"
OUT_DIR="dist"

mkdir -p "$OUT_DIR"

# ── Color output ───────────────────────────────────────────────────────────────
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; NC='\033[0m'
info()    { echo -e "${GREEN}[INFO]${NC} $*"; }
warning() { echo -e "${YELLOW}[WARN]${NC} $*"; }
error()   { echo -e "${RED}[ERROR]${NC} $*"; exit 1; }

# ── Desktop Build ─────────────────────────────────────────────────────────────
build_desktop() {
    info "Building desktop release..."

    # Standard release build with size optimizations (set in Cargo.toml)
    cargo build --release

    BINARY="target/release/${BINARY_NAME}"
    SIZE_BEFORE=$(du -sh "$BINARY" | cut -f1)
    info "Binary size before strip: ${SIZE_BEFORE}"

    # Strip debug symbols (already set in Cargo.toml profile.release.strip=true)
    # but strip again explicitly for maximum effect on Linux
    if command -v strip &>/dev/null; then
        strip --strip-all "$BINARY" 2>/dev/null || true
    fi

    SIZE_AFTER=$(du -sh "$BINARY" | cut -f1)
    info "Binary size after strip:  ${SIZE_AFTER}"

    # Optional: UPX compression (uncomment if UPX is installed)
    # Reduces binary size by ~60% at cost of ~100ms startup time
    # if command -v upx &>/dev/null; then
    #     upx --best --lzma "$BINARY"
    #     SIZE_UPX=$(du -sh "$BINARY" | cut -f1)
    #     info "Binary size after UPX:   ${SIZE_UPX}"
    # fi

    cp "$BINARY" "$OUT_DIR/"
    info "Desktop binary: ${OUT_DIR}/${BINARY_NAME}"
}

# ── Windows Cross-compile ──────────────────────────────────────────────────────
build_windows() {
    info "Building Windows release (cross-compile from Linux)..."

    if ! rustup target list --installed | grep -q "x86_64-pc-windows-gnu"; then
        info "Adding Windows target..."
        rustup target add x86_64-pc-windows-gnu
    fi

    if ! command -v x86_64-w64-mingw32-gcc &>/dev/null; then
        error "mingw-w64 not found. Install with: sudo apt install mingw-w64"
    fi

    cargo build --release --target x86_64-pc-windows-gnu
    cp "target/x86_64-pc-windows-gnu/release/${BINARY_NAME}.exe" "$OUT_DIR/"
    info "Windows binary: ${OUT_DIR}/${BINARY_NAME}.exe"
}

# ── Android Build ─────────────────────────────────────────────────────────────
build_android() {
    info "Building Android release..."

    # Prerequisites check
    command -v cargo-ndk &>/dev/null || error "cargo-ndk not found. Install: cargo install cargo-ndk"
    [ -n "${ANDROID_NDK_HOME:-}" ] || error "ANDROID_NDK_HOME not set"
    [ -n "${ANDROID_HOME:-}" ] || error "ANDROID_HOME not set"

    # Add Android targets if not present
    for TARGET in aarch64-linux-android x86_64-linux-android armv7-linux-androideabi; do
        rustup target add "$TARGET" 2>/dev/null || true
    done

    # Build .so for all ABI targets
    cd android
    cargo ndk \
        -t arm64-v8a \
        -t armeabi-v7a \
        -t x86_64 \
        -o app/src/main/jniLibs \
        build --release

    info "Native libraries built in android/app/src/main/jniLibs/"

    # Build APK using Gradle
    if [ -f "gradlew" ]; then
        ./gradlew assembleRelease
        APK_PATH=$(find . -name "*.apk" -path "*/release/*" | head -1)
        if [ -n "$APK_PATH" ]; then
            cp "$APK_PATH" "../${OUT_DIR}/contacts-app.apk"
            info "APK: ${OUT_DIR}/contacts-app.apk"
        fi
    else
        warning "gradlew not found; skipping APK assembly. Copy the .so files manually."
    fi

    cd ..
}

# ── Dispatch ──────────────────────────────────────────────────────────────────
case "$TARGET" in
    desktop) build_desktop ;;
    windows) build_windows ;;
    android) build_android ;;
    all)
        build_desktop
        build_android
        ;;
    *) error "Unknown target: $TARGET. Use: desktop | windows | android | all" ;;
esac

info "Build complete! Outputs in ${OUT_DIR}/"
ls -lh "$OUT_DIR/"
