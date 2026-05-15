# 🕐 Android Clock — Rust + Slint

> A modern, Material You clock application built with Rust and Slint.  
> **One codebase. Runs natively on Android, Windows, macOS, and Linux.**

![Rust 1.75+](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)
![Slint 1.7](https://img.shields.io/badge/Slint-1.7-blue)
![License MIT](https://img.shields.io/badge/License-MIT-green)
![Binary Size](https://img.shields.io/badge/Binary-~8MB-brightgreen)

---

## ✨ Features

| Feature | Details |
|---------|---------|
| **Digital Clock** | Large display with smooth animations, AM/PM indicator |
| **Alarm Manager** | Add/toggle/delete alarms, swipe-to-delete, repeat days |
| **Countdown Timer** | Circular progress, number pickers, pause/resume |
| **Stopwatch** | Centisecond accuracy, unlimited laps, split times |
| **World Clock** | 16 pre-set cities, day/night icons, UTC offset display |
| **Material You** | Material 3 design tokens, dynamic color theming |
| **Responsive** | Phone → Tablet → TV adapts automatically (bottom nav → rail) |
| **Offline** | Zero network access required. Timezone DB compiled in. |
| **Minimal** | Binary ~8 MB, RAM ~30 MB, startup < 200ms |

---

## 🏗 Architecture

```
android-clock/
├── src/
│   ├── main.rs          # Entry point + MVU bridge (Rust ↔ Slint)
│   ├── model.rs         # Pure data types (no UI dependency)
│   ├── alarm.rs         # Alarm engine + scheduling logic
│   ├── timer.rs         # Countdown timer engine
│   ├── world_clock.rs   # Timezone conversion (chrono-tz)
│   ├── storage.rs       # Cross-platform persistence (JSON)
│   └── platform.rs      # Platform abstractions (vibration, audio)
├── ui/
│   ├── app.slint        # Root component + shared types + navigation
│   └── components/
│       ├── clock_tab.slint       # Digital clock display
│       ├── alarm_tab.slint       # Alarm list + swipe-to-delete
│       ├── timer_tab.slint       # Countdown timer + circular progress
│       ├── stopwatch_tab.slint   # High-precision stopwatch + laps
│       ├── world_clock_tab.slint # Multi-timezone display
│       └── add_alarm_dialog.slint # Material 3 bottom sheet dialog
├── tests/
│   └── integration_tests.rs     # Alarm, timer, stopwatch, security tests
├── scripts/
│   ├── build_release.sh         # Cross-platform release builder
│   └── package_android.sh       # APK packaging
├── build.rs                     # Compiles .slint files at build time
└── Cargo.toml
```

**Design Pattern: MVU (Model-View-Update)**
- **Model** (`src/model.rs`): Pure Rust structs, completely UI-independent
- **View** (`ui/*.slint`): Declarative UI, compiled to native code at build time
- **Update** (`src/main.rs`): Slint callbacks → Rust functions → update Slint properties

---

## 🚀 Quick Start

### Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version  # Should be 1.75+
cargo --version
```

### Run on Desktop (Linux / macOS / Windows)

```bash
git clone https://github.com/your-org/android-clock
cd android-clock

# Development build (fast compile, debug info)
cargo run

# Optimized release build
cargo run --release
```

That's it! The clock window will open immediately.

---

## 📱 Build for Android

### Method A: xbuild (Recommended)

xbuild is the official Slint-recommended build tool for Android. It handles
the JNI bridge, APK signing, and NDK toolchain setup automatically.

```bash
# Install xbuild
cargo install xbuild

# List connected Android devices/emulators
x devices

# Build and run on connected device (debug)
x run --device <device-id>

# Build release APK
x build --release --platform android --arch arm64
# Output: target/x/release/android/android-clock.apk
```

### Method B: cargo-ndk + cargo-apk

```bash
# Install tools
cargo install cargo-ndk cargo-apk

# Set NDK path (download from https://developer.android.com/ndk/downloads)
export ANDROID_NDK_HOME=/path/to/android-ndk-r26

# Add Android targets to Rust
rustup target add aarch64-linux-android    # ARM64 phones
rustup target add armv7-linux-androideabi  # ARM32 phones

# Build APK
cargo apk build --release
# Output: target/release/apk/android-clock.apk

# Install to connected device
adb install target/release/apk/android-clock.apk
```

### Android Minimum Requirements
- Android 8.0 (API 26) or higher
- Supports: ARM64, ARM32, x86_64 (emulator)
- Tested on: Pixel 6, Samsung Galaxy S21, Xiaomi Mi 11

---

## 🖥 Build for Windows

```bash
# On Linux (cross-compile):
rustup target add x86_64-pc-windows-gnu
sudo apt install mingw-w64  # Ubuntu/Debian
cargo build --release --target x86_64-pc-windows-gnu

# On Windows (native):
cargo build --release
# Output: target/release/android-clock.exe
```

**Windows Distribution:**
- Single `.exe` file, no installer needed
- Submit to [Microsoft Store](https://partner.microsoft.com/en-us/dashboard) via MSIX packaging
- Or distribute as a portable `.exe`

---

## 🐧 Build for Linux

```bash
cargo build --release
# Output: target/release/android-clock (~8 MB)

# Optional: Maximum size optimization
./scripts/build_release.sh linux-x64
# Applies strip + UPX (~4-5 MB result)
```

### Flatpak Distribution (Flathub)

```bash
# Install Flatpak SDK
sudo apt install flatpak flatpak-builder
flatpak install flathub org.freedesktop.Sdk//23.08

# Build Flatpak
flatpak-builder build-dir dev.slint.androidclock.yml --force-clean

# Test locally
flatpak-builder --run build-dir dev.slint.androidclock.yml android-clock

# Submit to Flathub:
# Fork https://github.com/flathub/flathub
# Add your manifest, open a PR
```

---

## 🍎 Build for macOS

```bash
cargo build --release
# Output: target/release/android-clock

# For App Store distribution (requires Apple Developer account):
# 1. Sign the binary: codesign --sign "Developer ID" target/release/android-clock
# 2. Create .app bundle (see scripts/package_macos.sh — coming soon)
# 3. Notarize: xcrun notarytool submit
```

---

## ⚡ Release Optimization

The Cargo.toml is pre-configured for minimal binary size:

```toml
[profile.release]
opt-level = "z"      # Size optimization
lto = true           # Dead code elimination
codegen-units = 1    # Better optimization
panic = "abort"      # No unwinding code
strip = true         # Strip debug symbols
```

Additional size reduction steps:

```bash
# 1. Check what's in your binary
cargo install cargo-bloat
cargo bloat --release --crates

# 2. Find large dependencies
cargo install cargo-tree
cargo tree --depth 2

# 3. Apply UPX compression (Linux/Android only)
upx --best --lzma target/release/android-clock
# Typically reduces from ~8MB to ~3MB

# 4. Profile binary with symbols
cargo install cargo-size
cargo size --release
```

**Expected sizes (stripped, before UPX):**
| Platform | Size |
|----------|------|
| Linux x64 | ~7 MB |
| Windows x64 | ~9 MB |
| Android ARM64 | ~6 MB |
| macOS ARM64 | ~8 MB |

---

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output (for debugging test failures)
cargo test -- --nocapture

# Run specific test module
cargo test alarm_tests
cargo test timer_tests
cargo test world_clock_tests

# Code coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
open tarpaulin-report.html

# Linting (zero warnings policy)
cargo clippy -- -D warnings

# Security audit
cargo install cargo-audit
cargo audit
```

---

## 🔐 Security Checklist

- [x] **No `unwrap()` in production paths** — all errors propagated via `Result<T, E>`
- [x] **Atomic file writes** — temp file + rename prevents data corruption on crash
- [x] **Corrupt state recovery** — bad JSON logs a warning and uses defaults (no crash)
- [x] **No hardcoded secrets** — zero API keys, passwords, or tokens in source
- [x] **Minimal permissions** — only `VIBRATE`, `SCHEDULE_EXACT_ALARM`, `POST_NOTIFICATIONS`
- [x] **No network access** — timezone DB compiled in, zero HTTP calls
- [x] **Memory safety** — 100% safe Rust, no `unsafe` blocks
- [x] **Dependency audit** — `cargo audit` in CI catches known CVEs
- [x] **Integer overflow** — `overflow-checks = true` in dev profile
- [x] **UUID-based IDs** — alarm IDs are v4 UUIDs (not sequential, not guessable)
- [ ] **Code signing** — configure for your distribution platform
- [ ] **Keystore security** — Android keystore should use strong password + offline storage

---

## 🎨 Customizing the Theme

The Material You color tokens are defined in `ui/app.slint`:

```slint
export global MaterialTheme {
    out property <color> primary: #6750A4;  // Change this!
    out property <color> primary-container: #EADDFF;
    // ... other tokens
}
```

To implement true Material You dynamic color (from wallpaper):
1. Detect the wallpaper's dominant color on Android via `WallpaperManager`
2. Pass it to Rust via a Slint property
3. Generate the full tonal palette using the Material Color Utilities algorithm

A community Rust implementation: `material-color-utilities-rs` on crates.io.

---

## 📋 Adding New Features

### Adding a new tab

1. Create `ui/components/my_tab.slint`
2. Import it in `ui/app.slint`
3. Add a property `in-out property <bool> my-tab-data` to `AppWindow`
4. Add an `if root.active-tab == 5:` block in the content area
5. Add a `NavItem` in both the bottom bar and navigation rail
6. Handle the tab's callbacks in `src/main.rs`

### Adding a new alarm sound

1. Place `.mp3` or `.wav` in `assets/sounds/`
2. Include with `include_bytes!("../assets/sounds/my_sound.mp3")`
3. Play with `rodio`: create a `Cursor<&[u8]>` from the bytes

---

## 🚢 Publishing

### Google Play Store
1. Build signed release APK (see Android section above)
2. Create Play Console account ($25 one-time fee)
3. Follow [Play Console upload guide](https://support.google.com/googleplay/android-developer/answer/9859152)
4. Required: Privacy policy, screenshots, 512px icon

### Microsoft Store  
1. Build Windows release binary
2. Package as MSIX using [MSIX Packaging Tool](https://docs.microsoft.com/en-us/windows/msix/packaging-tool/tool-overview)
3. Create [Partner Center account](https://partner.microsoft.com)
4. Submit for certification (usually 1-3 days)

### Flathub (Linux)
1. Fork https://github.com/flathub/flathub
2. Create `dev.slint.androidclock/` directory with your manifest
3. Open a pull request — Flathub team reviews within 1-2 weeks
4. Free hosting, automatic updates

### F-Droid (Android, Open Source)
1. Register at https://f-droid.org/en/contribute/
2. Submit your app's source repository
3. F-Droid builds from source (reproducible builds required)

---

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/world-clock-search`
3. Ensure `cargo clippy -- -D warnings` passes with zero warnings
4. Ensure `cargo test` passes
5. Open a pull request

**Code Style:**
- Follow `rustfmt` defaults: `cargo fmt`
- No `unwrap()` in non-test code
- Document public functions with `///` doc comments
- Keep `.slint` files focused — one component per file

---

## 📄 License

MIT License — see [LICENSE](LICENSE) for details.

Built with ❤️ using [Rust](https://www.rust-lang.org/) and [Slint](https://slint.dev/).
