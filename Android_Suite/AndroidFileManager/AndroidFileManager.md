
You are an expert senior Rust developer specializing in modern, minimal-footprint, cross-platform applications. Create a complete, production-ready **File Browser (File Manager)** using **Rust + latest Slint** with the following requirements:

### Core Goals

- **Modern & UX-friendly**: Use latest **Slint 1.16+** with Material You-inspired design, smooth animations, gestures, responsive layout, dual-pane support on large screens, bottom sheet / action bar, powerful search with filters.
- **Highly Secure**: Follow Rust security best practices, safe file operations, optional AES file/folder encryption, minimal permissions, clear user consent.
- **Write & Forget / Future-Proof**: Must work on Windows, macOS, Linux (and Android where possible). Use stable, forward-compatible APIs with minimal maintenance.
- **Broad Compatibility**: Support internal storage, external drives, USB, user-selected folders. No root required.
- **Minimal Footprint**: Extremely small binary size (<15MB stripped preferred), fast startup, low memory usage. Prioritize performance and small dependencies.

### Key Technical Requirements

**Storage Access Strategy**:
- Primary: Use `std::fs`, `walkdir`, `notify` (for watching), and `rfd` for native folder picker.
- Support user-selected directories with persistent access where possible.
- Graceful handling of permission errors and platform differences (`cfg(target_os)`).
- Use `dirs` crate for standard paths (Documents, Downloads, Pictures, etc.).
- Best-effort access to user-accessible locations.

**Features**:
- Browse all user-accessible directories.
- View, copy, move, delete, rename, compress (ZIP), extract files.
- File previews (images, video thumbnails, text, PDF basic support).
- Powerful search (name, content, type, date, size) with filtering.
- Multi-selection with action mode.
- Sort & filter (name, size, date, type, extension).
- Dark/Light theme with dynamic accent colors.
- Storage usage analysis.
- Recent files and favorites/bookmarks (persistent).
- Support hidden files (.dotfiles) with toggle.
- Basic archive support (ZIP).
- Optional: Network shares (SMB/FTP) via feature flags.

**Architecture**:
- Clean Architecture with clear separation (core, domain, fs, ui).
- Use latest stable Rust, Tokio for async, Flume or channels for communication.
- Repository pattern for file system operations.
- Robust error handling with user-friendly messages.
- Modular structure with minimal dependencies.

**Permissions & Security**:
- Use native dialogs (`rfd`) for folder selection.
- Safe delete/move with undo support (trash or temp backup).
- Optional AES-256 encryption for sensitive files/folders.
- Path validation and canonicalization to prevent traversal attacks.

**UI/UX Excellence**:
- Beautiful, responsive Slint UI with excellent performance.
- Breadcrumb navigation.
- Tabbed view (Home / Favorites / Recent).
- Keyboard shortcuts and accessibility support.
- Fast loading with virtualized lists.
- Drag & drop support where possible.

**Additional Requirements**:
- Provide full project structure with clean, optimized, well-commented code.
- Include detailed comments explaining why certain approaches are minimal and future-proof.
- Suggest how to publish on platforms (Android Play store,Windows Store, Flathub, etc.).
- Add basic tests and security checklist.
- Provide a complete **README.md** with all configuration steps, how to build and run on Windows/macOS/Linux/Android, release optimizations for minimal size.

**Tech Stack Constraints (Minimal Footprint)**:
- Rust edition 1.93 - Slint 1.16.1 with Material Design Components as UI framework
- Build a responsive UI from a single design.
- Target different screen resolution and sizes with flexible layouts.(mobile to 65, 75, 85, and 98-inch display models.)
- Minimal dependencies only
- No Electron/Tauri/WebView
- Prefer zero-cost abstractions and compile-Run-time optimizations
- Enable maximum binary size reduction in release profile

Generate the complete production ready project including `README.md` with detail steps how to Run.


GNU GPLv3 Lic
