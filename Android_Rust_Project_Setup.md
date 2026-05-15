**Step-by-step guide for setting up Android SDK on Pop!_OS 24.04 for C++ (NDK) + Rust development.**

You already have the command-line tools extracted. I'll assume you placed them in a standard location like `~/android-sdk` (adjust paths as needed).

### 1. Prepare Directory Structure & Environment

```bash
# Create SDK root if not done
mkdir -p ~/android-sdk/cmdline-tools

# Move/extract your commandlinetools into cmdline-tools/latest (important structure)
# Example (adjust source path):
unzip commandlinetools-linux-14742923_latest.zip -d ~/android-sdk/cmdline-tools/
mv ~/android-sdk/cmdline-tools/cmdline-tools/* ~/android-sdk/cmdline-tools/latest/
rmdir ~/android-sdk/cmdline-tools/cmdline-tools  # if empty

# Set environment variables (add to ~/.bashrc or ~/.zshrc)
export ANDROID_HOME=$HOME/android-sdk
export ANDROID_SDK_ROOT=$ANDROID_HOME
export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin
export PATH=$PATH:$ANDROID_HOME/platform-tools
export PATH=$PATH:$ANDROID_HOME/build-tools/<version>  # will be available after install

# Reload
source ~/.bashrc
```

### 2. Accept Licenses & Update

```bash
# Accept all licenses
yes | sdkmanager --licenses

# Update existing packages
sdkmanager --update
```

### 3. Install Required SDK Components via sdkmanager

Run these one by one or together. Use `sdkmanager --list` first to see exact available versions (recommended).

**Core packages:**

```bash
sdkmanager "platform-tools" "platforms;android-35" "build-tools;35.0.0"  # Adjust versions to latest available
```

**For C++ / Native development (NDK + CMake):**

```bash
sdkmanager "ndk;27.0.12077973" "cmake;3.22.1"   # Use latest stable NDK; check with --list
# OR for side-by-side latest: sdkmanager "ndk;latest" but specific version is safer
```

**Recommended extras (useful for development):**

```bash
sdkmanager "emulator" "patcher;v4" "extras;google;google_play_services" "extras;google;instantapps"
```

**Full recommended command (customize versions):**

```bash
sdkmanager --verbose \
  "platform-tools" \
  "platforms;android-35" \
  "build-tools;35.0.0" \
  "ndk;27.0.12077973" \
  "cmake;3.22.1" \
  "emulator"
```

**Google USB Driver note**: This is **Windows-only**. On Linux (Pop!_OS), you do **not** need it. Instead, set up udev rules for ADB (see below).

### 4. Linux USB / ADB Setup (for real devices)

```bash
# Install udev rules
sudo apt update
sudo apt install android-tools-adb android-tools-fastboot

# Common udev rules for most devices
sudo tee /etc/udev/rules.d/51-android.rules << EOF
SUBSYSTEM=="usb", ATTR{idVendor}=="18d1", MODE="0666", GROUP="plugdev"
SUBSYSTEM=="usb", ATTR{idVendor}=="0bb4", MODE="0666", GROUP="plugdev"
# Add more vendor IDs as needed (Google, Samsung, etc.)
EOF

sudo chmod a+r /etc/udev/rules.d/51-android.rules
sudo udevadm control --reload-rules
sudo udevadm trigger
```

Add your user to plugdev group: `sudo usermod -aG plugdev $USER` (log out/in).

### 5. For Rust Development

After SDK/NDK is ready:

```bash
# Install Rust if not installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add Android targets
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android

# Install cargo-ndk (recommended)
cargo install cargo-ndk
```

Set NDK path in `~/.cargo/config.toml` or environment:

```toml
[target.aarch64-linux-android]
linker = "aarch64-linux-android34-clang"  # adjust based on NDK version
```

### 6. Verify Installation

```bash
sdkmanager --list_installed
adb version
ndk-build --version   # or check $ANDROID_HOME/ndk/
cmake --version
```

### 7. Create `android-sdk-setup.md`

Create this file in your project or home directory:

```bash
cat > ~/android-sdk-setup.md << 'EOF'
# Android SDK Setup for Pop!_OS 24.04 (C++ + Rust NDK Development)

## Environment Variables (add to ~/.bashrc)
```bash
export ANDROID_HOME=$HOME/android-sdk
export ANDROID_SDK_ROOT=$ANDROID_HOME
export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools
```

## Installed SDK Components (via sdkmanager)

- `platform-tools`
- `platforms;android-35` (or latest)
- `build-tools;35.0.0` (or matching latest)
- `ndk;27.0.12077973` (or latest stable)
- `cmake;3.22.1`
- `emulator` (optional)

## Installation Commands

```bash
yes | sdkmanager --licenses
sdkmanager --update
sdkmanager "platform-tools" "platforms;android-35" "build-tools;35.0.0" "ndk;27.0.12077973" "cmake;3.22.1"
```

## USB/ADB on Linux
- No Google USB Driver needed.
- Use udev rules (see setup guide).
- `sudo apt install android-tools-adb`

## Rust Setup
```bash
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
cargo install cargo-ndk
```

## Verification
- `adb devices`
- `sdkmanager --list_installed`

**Last updated:** May 2026
EOF
```

Open it with `cat ~/android-sdk-setup.md` or a markdown viewer.

### Tips
- Run `sdkmanager --list` to get exact current package names/versions.
- For specific NDK versions (reproducible builds), use `ndk;<version>`.
- If using Android Studio later, it can manage some of this, but CLI is fine for pure native/Rust workflows.
- Restart terminal or `source ~/.bashrc` after env changes.

This covers everything for C++/Rust Android native development. Let me know if you hit any errors!
