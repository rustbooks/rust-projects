# 📱 RustDialer

> **Cross-platform Material You Dialer built with Rust + Slint**
>
> Single codebase → Android phone · Android TV · Windows · macOS · Linux

![License](https://img.shields.io/badge/license-MIT-blue)
![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange)
![Slint](https://img.shields.io/badge/Slint-1.9-green)

---

## ✨ Features

| Feature | Details |
|---|---|
| 📇 Contacts | Add · Edit · Delete · Sort (First Name / Last Name / Number) |
| ⭐ Favorites | Star any contact for quick access |
| 🕐 Call History | Incoming · Outgoing · Missed with timestamps |
| 🔢 Dialpad | Full numeric keypad with direct-dial |
| 🔍 Search | Real-time filter across name and number |
| 🎨 Material You | Dynamic color theming, elevation, smooth animations |
| 📐 Responsive | Phone → Tablet → TV dual-pane layout from one `.slint` file |
| 🔒 Encryption | Optional AES-256-GCM for contacts at rest |
| ⚡ Performance | <300ms startup · <15 MB binary · <100 MB RAM |

---

## 🗂 Project Structure

```
rust-dialer/
├── Cargo.toml              # Dependencies + release profile
├── build.rs                # Slint compiler invocation
├── .cargo/config.toml      # Cross-compilation linker config
│
├── src/
│   ├── main.rs             # Entry point, Slint callbacks, event loop
│   ├── lib.rs              # Android JNI entry point
│   ├── models.rs           # Domain structs (Contact, CallRecord, …)
│   ├── app_state.rs        # Business logic, sort/filter, CRUD
│   ├── storage.rs          # JSON persistence + AES-256-GCM encryption
│   └── call_engine.rs      # Call simulation (real device: Telecom API)
│
├── ui/
│   └── dialer.slint        # Complete Material You UI (all screen sizes)
│
├── android/
│   ├── app/
│   │   ├── build.gradle
│   │   └── src/main/
│   │       ├── AndroidManifest.xml
│   │       ├── java/com/rustdialer/MainActivity.java
│   │       ├── jniLibs/         # Place compiled .so files here
│   │       └── res/values/styles.xml
│   ├── build.gradle
│   └── settings.gradle
│
└── tests/
    └── integration_test.rs  # Full business-logic test suite
```

---

## 🚀 Quick Start — Desktop

### Prerequisites
```bash
# Install Rust (1.75+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Linux: install system dependencies for Slint
sudo apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
                 libxkbcommon-dev libssl-dev pkg-config cmake
```

### Build & Run
```bash
git clone https://github.com/yourorg/rust-dialer
cd rust-dialer

# Development run
cargo run

# Optimized release build (stripped, ~8–12 MB)
cargo build --release
./target/release/rust-dialer
```

---

## 🤖 Android Build

### 1. Install Android NDK
```bash
# Using sdkmanager (from Android Studio or command line tools)
sdkmanager "ndk;27.0.12077973"

# Or set NDK_HOME to your existing NDK installation
export NDK_HOME=$HOME/Android/Sdk/ndk/27.0.12077973
```

### 2. Install Rust Android Targets
```bash
rustup target add aarch64-linux-android    # ARM64 (modern phones)
rustup target add armv7-linux-androideabi  # ARMv7 (older phones)
rustup target add x86_64-linux-android     # x86_64 (emulator)
```

### 3. Configure NDK Linkers
Add to `~/.cargo/config.toml` (or use the project's `.cargo/config.toml`):
```toml
[target.aarch64-linux-android]
linker = "/path/to/ndk/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android34-clang"
```

Or set environment variables:
```bash
export ANDROID_NDK_HOME=$NDK_HOME
export PATH="$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH"
```

### 4. Compile Rust Library
```bash
# ARM64 (primary target — all phones since 2015)
cargo build --release --target aarch64-linux-android --features android

# ARMv7 (optional — legacy devices)
cargo build --release --target armv7-linux-androideabi --features android

# Copy .so to jniLibs
mkdir -p android/app/src/main/jniLibs/arm64-v8a
mkdir -p android/app/src/main/jniLibs/armeabi-v7a

cp target/aarch64-linux-android/release/librust_dialer.so \
   android/app/src/main/jniLibs/arm64-v8a/

cp target/armv7-linux-androideabi/release/librust_dialer.so \
   android/app/src/main/jniLibs/armeabi-v7a/
```

### 5. Build APK
```bash
cd android
./gradlew assembleDebug           # debug APK
./gradlew assembleRelease         # signed release APK

# Install on connected device / emulator
./gradlew installDebug
```

---

## 🪟 Windows Build

```powershell
# Install Rust for Windows
winget install Rustlang.Rustup

# Build release
cargo build --release

# Output: target\release\rust-dialer.exe  (~10 MB)
```

### Windows Store (MSIX packaging)
```powershell
# Install cargo-wix
cargo install cargo-wix

# Generate WiX installer config
cargo wix init

# Build .msi installer
cargo wix
```

---

## 🐧 Linux / Flatpak

```bash
# Native build
cargo build --release

# Flatpak packaging
flatpak install org.freedesktop.Sdk//23.08
flatpak-builder build-dir flatpak/com.rustdialer.app.yml --install --user
```

Sample Flatpak manifest (`flatpak/com.rustdialer.app.yml`):
```yaml
app-id: com.rustdialer.app
runtime: org.freedesktop.Platform
runtime-version: '23.08'
sdk: org.freedesktop.Sdk
command: rust-dialer
modules:
  - name: rust-dialer
    buildsystem: simple
    build-commands:
      - install -Dm755 rust-dialer /app/bin/rust-dialer
    sources:
      - type: file
        path: ../target/release/rust-dialer
```

---

## 🔒 Security Checklist

- [x] No `unwrap()` in production paths — all errors use `?` or explicit handling
- [x] AES-256-GCM encryption with random 96-bit nonce per write
- [x] Atomic file writes (write temp → rename) prevent corruption
- [x] UUID v4 for contact IDs (unpredictable, no enumeration)
- [x] Minimal Android permissions — only what's strictly needed
- [x] No network access — fully offline, no telemetry
- [x] No `unsafe` blocks (except the JNI `extern "C"` boundary)
- [x] `panic = "abort"` in release — no stack-unwinding exploitation
- [x] Input validation before any mutation (`Contact::validate()`)
- [x] Call history capped at 500 entries — no unbounded growth
- [ ] TODO: OS keychain integration for AES key storage
- [ ] TODO: Biometric authentication gate before decrypting contacts

---

## ⚡ Release Size Optimization

The `[profile.release]` in `Cargo.toml` already applies:

| Flag | Effect |
|---|---|
| `opt-level = "z"` | Optimize for binary size over speed |
| `lto = true` | Link-Time Optimization removes dead code |
| `codegen-units = 1` | Single unit gives best LTO |
| `strip = true` | Remove debug symbols |
| `panic = "abort"` | No unwinder code |

### Further size reduction (optional):
```bash
# Install UPX compressor
sudo apt install upx

# Compress binary (~40–60% reduction, slight startup overhead)
upx --best --lzma target/release/rust-dialer

# Check final size
ls -lh target/release/rust-dialer
```

Expected sizes (before UPX):
| Platform | Size |
|---|---|
| Linux x86_64 | ~10–14 MB |
| Windows x86_64 | ~11–15 MB |
| Android arm64 | ~8–12 MB |

---

## 🧪 Running Tests

```bash
# Unit tests (no display needed)
cargo test

# Integration tests only
cargo test --test integration_test

# With verbose output
cargo test -- --nocapture

# Android cross-compile test (host tests only — device tests need ADB)
cargo test --target aarch64-linux-android   # requires device/emulator
```

---

## 📦 Publishing

### Google Play Store
1. Build a signed release APK or AAB:
   ```bash
   cd android && ./gradlew bundleRelease   # produces .aab (preferred)
   ```
2. Create a [Play Console](https://play.google.com/console) account ($25 one-time)
3. Create app → fill Data Safety section (no data collected/shared)
4. Upload AAB to Internal Testing → promote to Production
5. Declare CALL_PHONE permission purpose in the manifest questionnaire

### Flathub (Linux)
1. Fork [flathub/com.rustdialer.app](https://github.com/flathub)
2. Submit your `.yml` manifest via PR
3. Flathub CI builds and publishes automatically

### Microsoft Store (MSIX)
1. Package with `cargo wix` or MSIX Packaging Tool
2. Register at [Partner Center](https://partner.microsoft.com) (free)
3. Upload MSIX → submit for certification

---

## 🗺 Roadmap

- [ ] Real Android Telecom API integration (replace simulation)
- [ ] OS keychain for AES key (Android Keystore / Windows DPAPI / macOS Keychain)
- [ ] Contact import/export (vCard .vcf)
- [ ] Dark mode / system theme follow
- [ ] Incoming call notification (Android foreground service)
- [ ] Contact photo support
- [ ] Call recording (platform-permitting)
- [ ] Widget / Quick Tile for direct-dial

---

## 📄 License

MIT © 2025 RustDialer Contributors
