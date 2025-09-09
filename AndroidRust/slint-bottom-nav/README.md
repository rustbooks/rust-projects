Android app with a bottom navigation menu using Rust and Slint

### Prerequisites
Before starting, ensure you have the following installed:
1. **Rust**: Install via `rustup` (https://rustup.rs/):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```
   Verify with `rustc --version` (e.g., 1.80.1 or later).

2. **Android SDK and NDK**:
   - Install Android Studio (https://developer.android.com/studio) or the command-line SDK tools.
   - Set up the Android SDK and NDK (version r25b or compatible):
     - SDK: Install via Android Studio’s SDK Manager (target API 33 for Android 13).
     - NDK: Install via SDK Manager (e.g., NDK 25.2.9519653).
     - Set environment variables:
       ```bash
       export ANDROID_HOME=$HOME/Android/Sdk
       export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/<version>
       ```
       Add to `~/.bashrc` or equivalent for persistence.

3. **Java JDK**: Required for Android builds (e.g., OpenJDK 17). Install via your package manager or Android Studio.
   ```bash
   # Example for Ubuntu
   sudo apt install openjdk-17-jdk
   ```
4. **cargo-apk**: Install this Rust tool to build Android APKs:
   ```bash
   cargo install cargo-apk
   Or
   cargo install cargo-apk --force
   ```
5. **Android Device/Emulator**: A physical Android device (API 21 or higher) or an emulator set up via Android Studio.
6. **ADB**: Ensure Android Debug Bridge is installed (part of Android SDK’s platform-tools):
   ```bash
   export PATH=$PATH:$ANDROID_HOME/platform-tools

   ```
   Verify with `adb --version`.

### Step 1: Create the Project
1. **Create a New Rust Project**:
   ```bash
   cargo new slint-android-bottom-nav --lib
   cd slint-android-bottom-nav
   ```
   This creates a library project (required for Android’s `cdylib` output).

2. **Configure `Cargo.toml`**:
   Edit `Cargo.toml` to include dependencies and Android metadata. 

   - **Dependencies**:
     - `slint`: For the UI framework, with Android support and Material Design colors.
     - `android-activity`: For Android lifecycle integration.
     - `jni` and `log`: For Android interop and logging.
   - `[lib]`: Sets the crate type to `cdylib` for Android’s native library.
   - `[package.metadata.android]`: Points to a custom manifest (created later).

3. **Add Android Target**:
   Add the Android target for your architecture (e.g., `aarch64` for most modern devices):
   ```bash
   rustup target add aarch64-linux-android
   ```
   Optionally add `armv7-linux-androideabi`, `x86_64-linux-android`, or `i686-linux-android` for other devices/emulators.

### Step 2: Set Up Project Structure
Create the following directory structure:
```
slint-android-bottom-nav/
├── Cargo.toml
├── build.rs
├── src/
│   └── lib.rs
├── app.slint
├── android/
│   └── AndroidManifest.xml
├── assets/
│   ├── home.png
│   ├── search.png
│   └── profile.png
```

- **build.rs**: For compiling the Slint UI file.
- **src/lib.rs**: The Rust entry point for Android.
- **app.slint**: The Slint UI definition.
- **android/AndroidManifest.xml**: Custom manifest for Android.
- **assets/**: Placeholder for navigation icons (optional; can be omitted if using text or colors).

### Step 3: Write the Code

### Step 4: Add Icons (Optional)
The `app.slint` file references icons (`home.png`, `search.png`, `profile.png`) in `assets/`. You can:
- **Use Placeholder Images**: Download or create 24x24 PNG icons and place them in `assets/`.
- **Skip Icons**: Replace `@image-url("assets/home.png")` with placeholders (e.g., `Rectangle { background: blue; }`) for testing:
  ```slint
  Image {
      source: icon;
      width: 24px;
      height: 24px;
      colorize: is-selected ? blue : gray;
      // Fallback if no icon
      Rectangle { background: is-selected ? blue : gray; visible: icon == @image-url(""); }
  }
  ```
- For production, include icons in `android/res/mipmap/` for the app icon (`ic_launcher.png`) referenced in the manifest.

### Step 5: Build the Project
1. **Set Up Environment**:
   Ensure `ANDROID_HOME` and `ANDROID_NDK_HOME` are set:
   ```bash
   export ANDROID_HOME=$HOME/Android/Sdk
   export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/<version>
   ```
2. **Build the APK**:
   ```bash
   cargo apk build --target aarch64-linux-android --release
                    Or
   cargo build --release --target aarch64-linux-android --verbose 
   ```
                            
   This generates `target/aarch64-linux-android/release/app.apk`.
3. **Troubleshooting**:
   - If `cargo-apk` fails, verify SDK/NDK paths and ensure `ndk-build` and `sdkmanager` are accessible.
   - Check Slint’s Android docs (https://slint.dev/docs/android) for version compatibility.

### Step 6: Run the App
1. **Connect a Device/Emulator**:
   - Start an emulator via Android Studio (API 21+).
   - Or connect a physical device with USB debugging enabled (`adb devices` to verify).
2. **Install and Run**:
   ```bash
   cargo apk run --target aarch64-linux-android --release
   
   ```
   This installs `app.apk` and launches the app. Alternatively:
   ```bash
   adb install target/aarch64-linux-android/release/app.apk
   adb shell am start -n org.xai.slint_android_bottom_nav/android.app.NativeActivity
   ```

### Step 7: Customize (Optional)
- **Icons**: Add real icons to `assets/` and `android/res/mipmap/` for the app icon.
- **Styling**: Adjust `app.slint` for custom colors, animations, or fonts (e.g., use Slint’s Material theme properties).
- **Content**: Replace the `Text` in the content area with actual screens (e.g., a list view or form).
- **Permissions**: Add `<uses-permission>` to the manifest if needed (e.g., `<uses-permission android:name="android.permission.INTERNET"/>`).


# Troubleshooting

## 1) create .cargo\config.toml
   - Update `.cargo\config.toml`:
    ```toml
[target.aarch64-linux-android]
ar = "E:\\Softwares\\Android_8Spt25\\sdk\\ndk\\29.0.14033849\\toolchains\\llvm\\prebuilt\\windows-x86_64\\bin\\aarch64-linux-android-ar.exe"
linker = "E:\\Softwares\\Android_8Spt25\\sdk\\ndk\\29.0.14033849\\toolchains\\llvm\\prebuilt\\windows-x86_64\\bin\\aarch64-linux-android34-clang.cmd"

[target.armv7-linux-androideabi]
ar = "E:\\Softwares\\Android_8Spt25\\sdk\\ndk\\29.0.14033849\\toolchains\\llvm\\prebuilt\\windows-x86_64\\bin\\arm-linux-androideabi-ar.exe"
linker = "E:\\Softwares\\Android_8Spt25\\sdk\\ndk\\29.0.14033849\\toolchains\\llvm\\prebuilt\\windows-x86_64\\bin\\armv7a-linux-androideabi34-clang.cmd"

[target.i686-linux-android]
ar = "E:\\Softwares\\Android_8Spt25\\sdk\\ndk\\29.0.14033849\\toolchains\\llvm\\prebuilt\\windows-x86_64\\bin\\i686-linux-android-ar.exe"
linker = "E:\\Softwares\\Android_8Spt25\\sdk\\ndk\\29.0.14033849\\toolchains\\llvm\\prebuilt\\windows-x86_64\\bin\\i686-linux-android34-clang.cmd"

[target.x86_64-linux-android]
ar = "E:\\Softwares\\Android_8Spt25\\sdk\\ndk\\29.0.14033849\\toolchains\\llvm\\prebuilt\\windows-x86_64\\bin\\x86_64-linux-android-ar.exe"
linker = "E:\\Softwares\\Android_8Spt25\\sdk\\ndk\\29.0.14033849\\toolchains\\llvm\\prebuilt\\windows-x86_64\\bin\\x86_64-linux-android34-clang.cmd"
    ```

## 2) Cargo requires a keystore to sign the APK, which is necessary for installation on Android devices. The error points to the missing configuration in the `[package.metadata.android.signing.release]` section of your `Cargo.toml`.

### Steps to Fix the Keystore Error

#### 1. **Understand the Requirement**
Android requires APKs to be signed with a keystore for release builds. You need to either:
- Create a new keystore and configure it in `Cargo.toml`.
- Use an existing keystore.
- Alternatively, build in debug mode (which uses a default debug keystore) for testing.

#### Create and Configure a Release Keystore**
Follow these steps to generate a keystore and update your `Cargo.toml`:

1. **Generate a Keystore**:
   Use the `keytool` command (included with the JDK, which you have at `D:\openjdk-17+35\jdk-17\bin` since it’s in your PATH) to create a keystore:

   ```powershell
   keytool -genkey -v -keystore release-keystore.jks -keyalg RSA -keysize 2048 -validity 10000 -alias my-alias
   ```
   - **Prompts**: You’ll be prompted for a keystore password, key password, and details like name, organization, etc. Note down the passwords and alias (e.g., `my-alias`).
   - **Output**: This creates `release-keystore.jks` in the current directory. Move it to your project folder for convenience:
   
     ```powershell
     move release-keystore.jks E:\ProgrammingLang\Rust\rust-projects\AndroidRust\slint-bottom-nav
     ```

2. **Update `Cargo.toml`**:
   
   Add or modify the `[package.metadata.android.signing.release]` section:
   ```toml
   
[package.metadata.android]
manifest = "android/AndroidManifest.xml"

[package.metadata.android.signing.release]
path = "E:\\ProgrammingLang\\Rust\\rust-projects\\AndroidRust\\slint-bottom-nav\\release-keystore.jks"
keystore_password = "<your-keystore-password>"
key_alias = "my-alias"
key_password = "<your-key-password>"
   ```
   - Replace `<your-keystore-password>` and `<your-key-password>` with the passwords you set during keystore creation.
   - Ensure the `path` uses double backslashes (`\\`) for Windows compatibility.

3. **Re-run the Command**:
   Try running the command again:
   ```powershell
   cd E:\ProgrammingLang\Rust\rust-projects\AndroidRust\slint-bottom-nav
   cargo apk run --target aarch64-linux-android --release
   ```

## 3) PATH Verification: Confirm it’s accessible:
```powershell
echo %PATH%
```

```bash
D:\openjdk-17+35\jdk-17;D:\openjdk-17+35\jdk-17\bin\;
D:\openjdk-17+35\jdk-17\lib\;
E:\Softwares\Android_8Spt25\sdk;
E:\Softwares\Android_8Spt25\sdk\ndk\29.0.14033849;E:\Softwares\Android_8Spt25\sdk\platform-tools;E:\Softwares\Android_8Spt25\sdk\platforms\android-36;E:\Softwares\Android_8Spt25\sdk\cmdline-tools\latest\bin;E:\Softwares\Android_8Spt25\sdk\build-tools\36.0.0;
E:\Softwares\Android_8Spt25\sdk\ndk\29.0.14033849\toolchains\llvm\prebuilt\windows-x86_64\bin;
```

## 4) Rust Log
```bash
set RUST_LOG=debug
cargo apk build --target aarch64-linux-android --release
```
