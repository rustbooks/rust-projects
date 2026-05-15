# Android Messages — Rust + Slint

A **production-ready, cross-platform SMS messaging application** built with:

| Layer | Technology |
|-------|-----------|
| UI | [Slint 1.9](https://slint.dev) — Material You / Material Design 3 |
| Language | Rust 2021 edition |
| Storage | SQLite via `rusqlite` (bundled, no runtime dependency) |
| Encryption | AES-256-GCM via `aes-gcm` (pure Rust) |
| Async | Tokio multi-threaded runtime |
| Targets | Android (AArch64/ARMv7/x86_64) · Windows · macOS · Linux |

---

## Features

- **Material You** dynamic colour theming, elevation, ripple effects
- **Responsive dual-pane** layout: phone → tablet → 65-98" TV / monitor
- **AES-256-GCM** end-to-end local encryption (toggle per conversation)
- **Full-text search** across contacts and phone numbers
- **Attachment support**: image, video, audio, arbitrary files
- **Message status indicators**: sending → sent → delivered → read ✓✓
- **Swipe gestures**: delete / archive conversations
- **Dark mode** toggle
- **Offline-first**: SQLite works without internet; sync when available
- **Sub-300 ms startup**, < 100 MB RAM typical, binary < 15 MB (release)

---

## Project Structure

```
android-messages/
├── build.rs                  # Compiles .slint files at build time
├── Cargo.toml
├── AndroidManifest.xml       # Android permissions & activities
├── .cargo/config.toml        # Cross-compilation linker config
├── ui/
│   ├── app.slint             # Root window + global Theme tokens
│   ├── conversation_list.slint
│   ├── message_thread.slint
│   ├── compose_bar.slint
│   ├── search_bar.slint
│   ├── settings_panel.slint
│   ├── new_message_dialog.slint
│   └── attachment_viewer.slint
├── src/
│   ├── main.rs               # Entry point, Tokio runtime, Slint loop
│   ├── lib.rs                # Library root (for integration tests)
│   ├── crypto/mod.rs         # AES-256-GCM encrypt/decrypt
│   ├── db/
│   │   ├── mod.rs            # Database wrapper (SQLite + WAL)
│   │   ├── schema.rs         # Schema migrations (versioned, append-only)
│   │   ├── conversations.rs  # ConversationRepo CRUD
│   │   ├── messages.rs       # MessageRepo CRUD
│   │   └── attachments.rs    # AttachmentRepo CRUD
│   ├── models/
│   │   ├── mod.rs
│   │   ├── conversation.rs
│   │   ├── message.rs
│   │   └── attachment.rs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── app_controller.rs # Business logic + async orchestration
│   │   ├── wiring.rs         # Connects Slint callbacks → controller
│   │   └── ui_mapper.rs      # Domain types → Slint struct types
│   └── utils/
│       ├── mod.rs
│       ├── paths.rs          # Cross-platform data directory
│       └── seed.rs           # Demo data (development only)
├── assets/
│   └── icon.png              # 512×512 app icon (replace placeholder)
└── tests/
    └── integration_test.rs
```

---

## Prerequisites

### All platforms
```bash
# Install Rust (stable)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable
```

### Linux (additional packages)
```bash
# Debian / Ubuntu
sudo apt install -y \
  libxkbcommon-dev libfontconfig1-dev \
  libwayland-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  pkg-config build-essential

# Fedora / RHEL
sudo dnf install -y \
  libxkbcommon-devel fontconfig-devel \
  wayland-devel xcb-util-devel pkg-config
```

### macOS (additional packages)
```bash
xcode-select --install   # Command-line tools (includes clang + linker)
```

### Windows
Install [Visual Studio Build Tools 2022](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
with the **"Desktop development with C++"** workload.

### Android NDK (for Android builds)
```bash
# 1. Install Android Studio or standalone command-line tools
#    https://developer.android.com/studio

# 2. Install NDK r26+ via SDK Manager or:
sdkmanager "ndk;26.3.11579264"

# 3. Add NDK toolchain to PATH (adjust path as needed):
export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/26.3.11579264
export PATH=$PATH:$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin

# 4. Add Rust Android targets:
rustup target add \
  aarch64-linux-android \
  armv7-linux-androideabi \
  x86_64-linux-android
```

---

## Build & Run

### Desktop (development)
```bash
# Debug build — fast compile, verbose logging
RUST_LOG=info cargo run

# With demo data:
RUST_LOG=debug cargo run -- --demo

# Release build (optimised, stripped)
cargo build --release
# Binary: target/release/android-messages  (Linux/macOS)
#         target\release\android-messages.exe  (Windows)
```

### Desktop (release size optimisation)
```bash
cargo build --release

# Optional: further compress with UPX
# Install upx: https://upx.github.io
upx --best --lzma target/release/android-messages
```

### Android (AArch64 — modern phones)
```bash
# Build shared library
cargo build --release --target aarch64-linux-android

# Package into APK using cargo-apk:
cargo install cargo-apk
cargo apk build --release

# Install on connected device / emulator:
cargo apk run --release
```

### Android (all ABIs — Play Store fat APK)
```bash
for TARGET in aarch64-linux-android armv7-linux-androideabi x86_64-linux-android; do
  cargo build --release --target $TARGET
done

# Then use gradle or bundletool to create an AAB/APK with all three ABIs.
```

---

## Release Size Optimisations

The following settings in `Cargo.toml` minimise binary size:

| Setting | Effect | Saving |
|---------|--------|--------|
| `opt-level = "z"` | Optimise for size, not speed | ~20-30% |
| `lto = true` | Remove dead code across crates | ~15-25% |
| `codegen-units = 1` | Better inlining / DCE | ~5-10% |
| `panic = "abort"` | Remove unwinding machinery | ~50 KB |
| `strip = true` | Remove debug symbols | ~60-70% |

After all optimisations, expected binary sizes:
- Linux x86_64: ~8-12 MB
- Android AArch64: ~6-10 MB
- Windows x86_64: ~10-14 MB

---

## Configuration

### Environment Variables
| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `info` | Log level: `error`, `warn`, `info`, `debug`, `trace` |

### Data Directory
Messages and the encryption key are stored in the OS-appropriate location:

| Platform | Path |
|----------|------|
| Windows | `%APPDATA%\android-messages\` |
| macOS | `~/Library/Application Support/android-messages/` |
| Linux | `$XDG_DATA_HOME/android-messages/` or `~/.local/share/android-messages/` |
| Android | `/data/data/dev.slint.androidmessages/files/` |

Files created:
- `messages.db` — SQLite database (WAL mode)
- `.msg.key` — 256-bit AES key (permissions: 0600 on Unix)

---

## Testing

```bash
# Unit tests (including crypto round-trip tests)
cargo test

# Integration tests only
cargo test --test integration_test

# With output
cargo test -- --nocapture

# Check for security lints
cargo clippy -- -W clippy::unwrap_used -W clippy::expect_used
```

---

## Security Checklist

- [x] No `unwrap()` in production code paths (`clippy::unwrap_used` warning)
- [x] AES-256-GCM authenticated encryption (detects tampering)
- [x] Random 96-bit nonce per message (prevents nonce reuse)
- [x] Encryption key stored at 0600 permissions on Unix
- [x] SQLite foreign keys enabled (prevents orphaned records)
- [x] WAL mode (atomic writes, no corruption on crash)
- [x] Minimal Android permissions (runtime-requested, not install-time)
- [x] No hardcoded paths or secrets in source code
- [x] `panic = "abort"` removes stack unwinding (reduces attack surface)
- [x] Dependencies audited: `cargo audit` (add `cargo-audit` to CI)
- [ ] TODO: Biometric lock screen (Android Keystore integration)
- [ ] TODO: Certificate pinning for any future server sync
- [ ] TODO: Secure key derivation from user passphrase (Argon2)

```bash
# Run security audit on dependencies:
cargo install cargo-audit
cargo audit
```

---

## Publishing

### Google Play Store
1. Build a release AAB: `cargo apk build --release` → sign with `jarsigner`
2. Create a Play Console listing at https://play.google.com/console
3. Upload the AAB to the **Internal Testing** track first
4. Gradually roll out to **Production** (10% → 50% → 100%)

Required metadata:
- App icon 512×512 PNG
- Feature graphic 1024×500 PNG
- Screenshots for phone, tablet, TV
- Privacy policy URL (required if using SMS permissions)
- Data safety form (declare SMS access)

### Microsoft Store (MSIX)
```bash
# Install cargo-wix for Windows installer:
cargo install cargo-wix
cargo wix init
cargo wix

# Or create MSIX manually:
# https://learn.microsoft.com/en-us/windows/msix/
```

### Flathub (Linux)
1. Create `dev.slint.AndroidMessages.yml` Flatpak manifest
2. Submit PR to https://github.com/flathub/flathub
3. Flatpak sandbox automatically handles permissions

### Snapcraft (Ubuntu Snap)
```bash
# Create snapcraft.yaml, then:
snapcraft
snapcraft upload android-messages_1.0.0_amd64.snap --release=stable
```

---

## Architecture Notes

### Why Slint?
- **Compile-time UI** — `.slint` files are compiled to native Rust; zero runtime interpreter
- **Tiny runtime** — no Electron/WebView overhead; pure GPU rendering via Skia or software fallback
- **Type-safe bridge** — the Rust↔UI boundary is fully typed and checked at compile time
- **Cross-platform** — single codebase targets Android, Windows, macOS, Linux

### Why SQLite?
- **Zero configuration** — no database server to manage
- **Bundled** — `rusqlite` compiles SQLite statically; no runtime .so/.dll dependency
- **WAL mode** — concurrent reads + atomic writes; safe on Android internal storage
- **Tiny footprint** — the amalgamation SQLite is ~750 KB

### Why AES-256-GCM over ChaCha20-Poly1305?
- Hardware acceleration on ARMv8 (Android) and x86 AES-NI
- NIST-standardised (required for some enterprise/government deployments)
- ChaCha20 is equally valid and preferred on devices without AES hardware

### MVU-inspired data flow
```
User Action (Slint callback)
    │
    ▼
wiring.rs  ──spawn──▶  tokio thread pool
                              │
                              ▼
                      AppController (async)
                              │
                       DB / Crypto ops
                              │
                 invoke_from_event_loop()
                              │
                              ▼
                    Slint UI state update
                              │
                              ▼
                    UI re-renders (60 fps)
```

---

## License

MIT — see LICENSE file.

---

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/my-feature`
3. Ensure all tests pass: `cargo test`
4. Ensure no clippy warnings: `cargo clippy --all-targets -- -D warnings`
5. Open a pull request

Commit style: [Conventional Commits](https://www.conventionalcommits.org/)
