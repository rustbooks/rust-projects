# Android Calculator — Rust + Slint Material You

A **production-ready, cross-platform calculator** built with Rust 1.75+ and Slint 1.7.
Features Material You (Material 3) design, four tabs (Basic, Scientific, History, Unit Converter),
and ships as a single stripped native binary.

---

## Features

| Tab | Capabilities |
|-----|-------------|
| **Basic** | +, −, ×, ÷, %, ±, backspace, live preview |
| **Scientific** | sin/cos/tan, log/ln, √, ∛, x², xʸ, 1/x, !, π, e, DEG/RAD |
| **History** | Persistent JSON history, recall, delete, clear all |
| **Converter** | 10 categories, 80+ units, live conversion, swap button |

### Design
- Material You tonal palette (purple / violet, M3 tokens)
- Smooth press/ripple animations on every button
- Adaptive font sizing on the display
- Bottom navigation with animated indicator pill
- Responsive layout — works from 320 px phone to 1920 px desktop

---

## Quick Start (Desktop)

### Prerequisites
```bash
# Rust stable 1.75+
rustup update stable

# Linux: install graphics dependencies
sudo apt install libxkbcommon-dev libwayland-dev   # Wayland
# or
sudo apt install libx11-dev libxcb1-dev            # X11
```

### Build & Run
```bash
git clone <repo>
cd android-calculator

# Debug (fast compile)
cargo run

# Release (optimised, ~3–5 MB stripped binary)
cargo build --release
./target/release/android-calculator
```

---

## Android Build

Slint supports Android via the `i-slint-backend-android-activity` crate.

### 1. Install Android toolchain
```bash
# Install Android NDK (via Android Studio or sdkmanager)
# Recommended: NDK r26+, API level 24+

rustup target add aarch64-linux-android
cargo install cargo-apk
```

### 2. Add Android backend to Cargo.toml
```toml
[target.'cfg(target_os = "android")'.dependencies]
slint = { version = "1.7", features = ["backend-android-activity"] }

[lib]
crate-type = ["cdylib"]
```

### 3. Create Android activity wrapper (src/android.rs)
```rust
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    main().unwrap();
}
```

### 4. android/AndroidManifest.xml (minimal)
```xml
<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    package="com.example.calculator">
    <application android:label="Calculator" android:theme="@android:style/Theme.NoTitleBar">
        <activity android:name="android.app.NativeActivity"
                  android:exported="true"
                  android:configChanges="orientation|keyboardHidden|screenSize">
            <meta-data android:name="android.app.lib_name" android:value="android_calculator"/>
            <intent-filter>
                <action android:name="android.intent.action.MAIN"/>
                <category android:name="android.intent.category.LAUNCHER"/>
            </intent-filter>
        </activity>
    </application>
</manifest>
```

### 5. Build APK
```bash
# Set environment variables
export ANDROID_HOME=$HOME/Android/Sdk
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/26.1.10909125

cargo apk build --release
# APK output: target/release/apks/android-calculator.apk

# Install on connected device
adb install target/release/apks/android-calculator.apk
```

---

## Project Structure

```
android-calculator/
├── Cargo.toml          # Dependencies & build profiles
├── build.rs            # Compiles Slint UI
├── README.md
├── ui/
│   └── app.slint       # Complete Material You UI (all tabs)
└── src/
    ├── main.rs         # Entry point, Slint ↔ Rust bridge
    ├── calculator.rs   # Recursive-descent expression parser
    ├── converter.rs    # Unit conversion engine (10 categories)
    ├── history.rs      # JSON persistence manager
    └── error.rs        # Typed error enum
```

---

## Architecture

```
AppWindow (Slint)
    ├── BasicTab      ──────────→  AppState::handle_calc_key()
    ├── ScientificTab ──────────→  AppState::handle_calc_key()
    ├── HistoryTab    ──────────→  HistoryManager (JSON on disk)
    └── ConverterTab  ──────────→  AppState::handle_conv_key()
                                   smart_convert()

Calculator (recursive descent)
    Lexer → [Token] → Parser → f64
    Supported: +−×÷%^()
               sin cos tan asin acos atan
               log ln sqrt cbrt sq inv abs exp
               ! (factorial) π e
```

---

## Binary Size Optimisation

The release profile is already configured for minimum size:
```toml
[profile.release]
opt-level = "z"   # optimise for size
lto = true        # link-time optimisation
codegen-units = 1 # single codegen unit for better LTO
panic = "abort"   # no unwinding machinery
strip = true      # strip debug symbols
```

Optional: further compress with UPX
```bash
upx --best target/release/android-calculator
```

---

## Running Tests

```bash
cargo test
```

Tests cover:
- Basic arithmetic and operator precedence
- Division by zero and domain errors
- Scientific functions (sin, cos, sqrt, factorial)
- Unit conversions (length, weight, temperature)
- History entry creation and serialisation

---

## History Storage

History is saved to:
- **Linux/macOS**: `~/.local/share/android-calculator/history.json`
- **Windows**: `%APPDATA%\android-calculator\history.json`
- **Android**: app internal storage

Maximum 500 entries; oldest are pruned automatically.

---

## License

MIT
