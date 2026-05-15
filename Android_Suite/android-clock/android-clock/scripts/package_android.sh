#!/usr/bin/env bash
# scripts/package_android.sh
# ============================================================================
# Android APK Packaging Script
# ============================================================================
# Builds a signed, optimized APK using xbuild (recommended for Slint) or
# cargo-apk. Both tools handle the JNI bridge automatically.
#
# Prerequisites:
#   Option A (Recommended): xbuild
#     cargo install xbuild
#     x devices  (to list connected devices)
#
#   Option B: cargo-apk
#     cargo install cargo-apk
#     Set ANDROID_NDK_HOME environment variable
#
# Usage:
#   ./scripts/package_android.sh [debug|release]
# ============================================================================

set -euo pipefail

MODE="${1:-debug}"
PACKAGE_NAME="dev.slint.androidclock"

GREEN='\033[0;32m'
NC='\033[0m'
log() { echo -e "${GREEN}[ANDROID]${NC} $1"; }

# ── Check for required tools ──────────────────────────────────────────────────
if command -v x &>/dev/null; then
    BUILDER="xbuild"
elif command -v cargo-apk &>/dev/null; then
    BUILDER="cargo-apk"
else
    echo "Error: Install xbuild (recommended) or cargo-apk"
    echo "  cargo install xbuild"
    exit 1
fi

log "Using builder: $BUILDER"

case "$BUILDER" in
    "xbuild")
        if [ "$MODE" = "release" ]; then
            log "Building release APK with xbuild..."
            x build --release --platform android --arch arm64
            APK_PATH="target/x/release/android/android-clock.apk"
        else
            log "Building debug APK with xbuild..."
            x build --platform android --arch arm64
            APK_PATH="target/x/debug/android/android-clock.apk"
        fi
        ;;

    "cargo-apk")
        if [ -z "${ANDROID_NDK_HOME:-}" ]; then
            echo "Error: Set ANDROID_NDK_HOME to your NDK path"
            exit 1
        fi
        if [ "$MODE" = "release" ]; then
            cargo apk build --release
            APK_PATH="target/debug/apk/android-clock.apk"
        else
            cargo apk build
            APK_PATH="target/release/apk/android-clock.apk"
        fi
        ;;
esac

if [ -f "${APK_PATH:-}" ]; then
    APK_SIZE=$(du -sh "$APK_PATH" | cut -f1)
    log "APK built successfully: $APK_PATH (${APK_SIZE})"

    # Install to connected device (optional)
    if command -v adb &>/dev/null && adb devices | grep -q "device$"; then
        log "Connected Android device detected. Installing..."
        adb install -r "$APK_PATH"
        log "Installed! Launch: adb shell am start -n ${PACKAGE_NAME}/.MainActivity"
    else
        log "No device connected. APK ready for manual installation."
    fi
else
    echo "APK not found at expected path. Check build output above."
    exit 1
fi

log "✅ Android packaging complete"
