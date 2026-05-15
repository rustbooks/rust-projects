# 📱 Contacts App

A **production-ready, cross-platform Contacts Manager** built with **Rust 1.75+** and **Slint 1.9** featuring Material You (Material 3) design.

Runs natively on **Android**, **Windows**, **macOS**, and **Linux** from a single codebase — no Electron, no WebView, no Tauri.

---

## ✨ Features

| Feature | Details |
|---------|---------|
| **CRUD Contacts** | Create, read, update, delete with full form validation |
| **Search** | Full-text search across all fields; Devanagari/Marathi Unicode support |
| **Sort** | Sort by First Name, Last Name, or Phone (Unicode-aware) |
| **Favorites** | Star contacts; dedicated Favorites tab with grid view |
| **Groups** | Create/delete groups; filter contacts by group |
| **Import** | vCard (.vcf), CSV, JSON — with automatic dedup by phone number |
| **Export** | vCard 3.0 (.vcf) and CSV; system file picker on desktop |
| **Encryption** | AES-256-GCM via `ring`; HKDF-SHA256 key derivation |
| **Responsive UI** | Phone → tablet → 65–98" TV; dual-pane master-detail on wide screens |
| **Material You** | Full M3 color tokens, shape system, motion, bottom nav, navigation rail |
| **Minimal binary** | ~8–12 MB stripped release binary; < 100 MB RAM |
| **Fast startup** | < 300 ms on modern hardware |

---

## 🏗️ Project Structure

```
contacts-app/
├── Cargo.toml              # Workspace manifest with size-optimized release profile
├── build.rs                # Slint UI compilation
├── src/
│   ├── main.rs             # Entry point + Slint ↔ Rust bridge (callbacks)
│   ├── lib.rs              # Public API for integration tests
│   ├── models.rs           # Contact, Group, SortKey data structures
│   ├── contacts.rs         # ContactManager — business logic, CRUD, sort
│   ├── search.rs           # Full-text search with Unicode/Devanagari support
│   ├── storage.rs          # Atomic JSON persistence, XDG/AppData paths
│   ├── crypto.rs           # AES-256-GCM encryption + HKDF key derivation
│   ├── vcard.rs            # vCard 2.1/3.0/4.0 import/export
│   └── csv_io.rs           # CSV import/export (Google Contacts format)
├── ui/
│   ├── main.slint          # Root window, navigation, page router
│   ├── themes/
│   │   └── theme.slint     # Material You color tokens, typography, spacing
│   ├── components/
│   │   ├── navigation.slint    # Bottom nav bar + navigation rail
│   │   ├── search_bar.slint    # Search input + sort chips
│   │   ├── fab.slint           # Floating Action Button
│   │   ├── dialogs.slint       # Confirm dialog
│   │   ├── toast.slint         # Snackbar/toast notification
│   │   └── contact_avatar.slint # Circular avatar with initials
│   └── pages/
│       ├── contacts_page.slint     # Main contacts list
│       ├── contact_detail_page.slint # Contact detail view
│       ├── edit_contact_page.slint  # Create/edit form
│       ├── favorites_page.slint     # Favorites grid
│       └── groups_page.slint       # Groups management
├── android/
│   ├── Cargo.toml          # cdylib crate for Android .so
│   ├── src/lib.rs          # android_main entry point
│   └── app/src/main/
│       ├── AndroidManifest.xml
│       ├── java/com/contacts/app/MainActivity.java
│       └── res/values/styles.xml
├── tests/
│   └── integration_test.rs # Full integration tests
├── scripts/
│   └── build_release.sh    # Release build script (desktop + Android)
├── .cargo/config.toml      # Linker config for size optimization
├── SECURITY.md             # Security checklist
└── README.md               # This file
```

---

## 🚀 Quick Start

### Prerequisites

```bash
# Install Rust (stable)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify version (1.75+ required for Slint 1.9)
rustc --version
```

### Linux / macOS

```bash
# Clone the project
git clone https://github.com/your-org/contacts-app
cd contacts-app

# Development run
cargo run

# Release build (optimized, stripped)
cargo build --release
./target/release/contacts-app
```

**Linux dependencies** (for Slint's winit backend):

```bash
# Ubuntu/Debian
sudo apt install libxkbcommon-dev libwayland-dev libgl1-mesa-dev pkg-config

# Fedora
sudo dnf install libxkbcommon-devel wayland-devel mesa-libGL-devel
```

### Windows

```bash
# No extra dependencies needed — Slint uses Win32/D3D11 natively
cargo build --release
.\target\release\contacts-app.exe
```

### macOS

```bash
cargo build --release
./target/release/contacts-app
```

---

## 📱 Android Build

### One-time Setup

```bash
# 1. Install Android SDK + NDK
#    Download Android Studio or command-line tools from https://developer.android.com

# 2. Set environment variables
export ANDROID_HOME=$HOME/Android/Sdk
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/25.2.9519653
export PATH=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH

# 3. Add Rust Android targets
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android

# 4. Install cargo-ndk
cargo install cargo-ndk

# 5. Install Java 17 (for Gradle)
sudo apt install openjdk-17-jdk  # Ubuntu
```

### Build & Install

```bash
# Build .so libraries for all ABIs
cd android
cargo ndk \
  -t arm64-v8a \
  -t armeabi-v7a \
  -t x86_64 \
  -o app/src/main/jniLibs \
  build --release

# Build APK (requires Gradle wrapper — add gradlew if using Android Studio)
./gradlew assembleRelease

# Install on connected device/emulator
adb install app/build/outputs/apk/release/app-release.apk
```

### Android Emulator Testing

```bash
# Create AVD with API 33 (Android 13)
$ANDROID_HOME/cmdline-tools/latest/bin/avdmanager create avd \
  -n contacts_test \
  -k "system-images;android-33;google_apis;x86_64"

# Start emulator
$ANDROID_HOME/emulator/emulator -avd contacts_test &

# Build x86_64 only (faster for emulator)
cargo ndk -t x86_64 -o app/src/main/jniLibs build
```

---

## 🧪 Running Tests

```bash
# All tests (unit + integration)
cargo test

# With output (useful for debugging)
cargo test -- --nocapture

# Specific test
cargo test test_devanagari_search

# Security audit (requires cargo-audit)
cargo install cargo-audit
cargo audit
```

---

## 📦 Release Optimization

The `Cargo.toml` release profile already applies these optimizations:

| Setting | Value | Effect |
|---------|-------|--------|
| `opt-level` | `"z"` | Optimize for size (vs speed) |
| `lto` | `true` | Link-time optimization removes dead code |
| `codegen-units` | `1` | Single unit enables better LTO |
| `panic` | `"abort"` | No unwinding code in binary |
| `strip` | `true` | Remove debug symbols automatically |

### Additional Size Reduction

```bash
# Check binary size breakdown
cargo install cargo-bloat
cargo bloat --release --crates

# Optional: UPX compression (60% smaller, +100ms startup)
# Install: https://upx.github.io/
upx --best --lzma target/release/contacts-app

# Expected sizes (approximate):
# Debug build:     ~80 MB
# Release (strip): ~10-14 MB
# Release + UPX:   ~4-6 MB
```

---

## 🚢 Publishing

### Android (Google Play Store)

1. **Sign the APK**:
   ```bash
   keytool -genkey -v -keystore release-key.jks -alias contacts-app \
     -keyalg RSA -keysize 2048 -validity 10000
   ```

2. **Configure signing in `android/app/build.gradle`**:
   ```gradle
   signingConfigs {
       release {
           storeFile file("release-key.jks")
           storePassword System.env.KEYSTORE_PASSWORD
           keyAlias "contacts-app"
           keyPassword System.env.KEY_PASSWORD
       }
   }
   ```

3. Build signed AAB:
   ```bash
   ./gradlew bundleRelease
   ```

4. Upload to [Google Play Console](https://play.google.com/console)

### Windows (Microsoft Store)

1. Build MSIX package:
   ```bash
   # Install WiX toolset or use cargo-wix
   cargo install cargo-wix
   cargo wix
   ```

2. Submit via [Partner Center](https://partner.microsoft.com/dashboard)

### Linux (Flathub)

1. Create `com.contacts.App.yml` Flatpak manifest
2. Build:
   ```bash
   flatpak-builder build-dir com.contacts.App.yml --force-clean
   flatpak-builder --run build-dir com.contacts.App.yml contacts-app
   ```
3. Submit PR to [flathub/flathub](https://github.com/flathub/flathub)

### Linux (Snap Store)

```bash
# Create snapcraft.yaml (provided in releases)
snapcraft
snapcraft upload --release=stable contacts-app_1.0.0_amd64.snap
```

---

## 🔐 Security

See [SECURITY.md](SECURITY.md) for the full security checklist.

**Key points:**
- No network access — fully offline
- AES-256-GCM encryption for sensitive data
- Atomic file writes prevent data corruption
- `#![forbid(unsafe_code)]` — no raw memory operations
- Minimal Android permissions (no system contacts access)

---

## 🌐 Internationalization

The app supports **Devanagari (Marathi/Hindi)** natively:

- Slint's `TextInput` handles all Unicode scripts via the OS text input stack
- Search uses Unicode NFC normalization for correct Devanagari matching
- Sort uses Unicode-aware `str::to_lowercase()` which handles Devanagari collation
- Avatar initials correctly extract the first Unicode codepoint (not first byte)

To add more locales: update placeholder strings in `.slint` files and add a localization system (e.g., `fluent-rs` crate).

---

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Write tests for new functionality
4. Run `cargo test && cargo clippy` — zero warnings required
5. Submit a pull request

---

## 📄 License

MIT License — see [LICENSE](LICENSE) file.
