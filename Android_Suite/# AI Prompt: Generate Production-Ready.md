
**You are an expert senior Rust developer specializing in modern, minimal-footprint, cross-platform applications that run on Android Smartphones/Windows/Linux + Slint architect.** Create **complete, production-ready, cross-platform Android applications** using:

- **Rust 1.93+** (2024 edition where applicable)
- **Slint 1.16.1+**
- **Slint Material Components** (Material 3 / Material You) from `material.slint.dev`
- Pure native Rust backend (no Tauri, no Electron)

## Target Applications (Generate One at a Time or as a Suite)

1. **Android Clock** (Alarm, Timer, Stopwatch, World Clock)
2. **Android Calculator** (Basic + Scientific + History + Unit Converter)
3. **Android Phone** (Dialer with call simulation / history / favorites)
4. **Contacts** (CRUD, search, groups, favorites, import/export)
5. **Android Messages** (SMS-like messaging with threads, search, attachments simulation)

---

## Core Goals (Strictly Follow)

- **Modern & UX-Friendly**:
  - Full **Material You** design (dynamic color theming, elevation, ripple effects).
  - Smooth 60fps animations, gestures (swipe to delete/archive, pull-to-refresh).
  - Responsive layout: single code base that adapts from **Android mobile** (phone-like) to **Android large screens** (65–98 inch TVs) with dual-pane, master-detail, bottom sheets, navigation rail/drawer.
  - Bottom navigation + FAB where appropriate.
  - Powerful search with filters, chips, recent suggestions.

- **Highly Secure**:
  - Follow Rust security best practices (no `unwrap()` in prod paths, proper error handling).
  - Safe file I/O with `std::fs` + user consent.
  - Optional AES-256-GCM encryption for sensitive data (contacts, messages) using `aes-gcm` or `ring`.
  - Minimal permissions model. Clear consent dialogs.

- **Write & Forget / Future-Proof**:
  - Works on **Windows, macOS, Linux** (and Android where Slint supports).
  - Use only stable, forward-compatible APIs.
  - Excellent documentation, modular architecture (MVU or similar), comprehensive tests.

- **Broad Compatibility**:
  - Support internal storage + external drives + USB + user-selected folders (`rfd` for folder picker).
  - No root/admin required.

- **Minimal Footprint**:
  - Stripped binary **< 15 MB** preferred (optimize with `strip`, `upx` if needed, minimal deps).
  - Fast startup (< 300ms), low memory (< 100 MB RAM typical).
  - Prioritize performance and small dependency graph.

- **Responsive UI from Single Design**:
  - Use Slint layouts (`GridLayout`, `HorizontalLayout`, `VerticalLayout`, `Box`, `TouchArea`).
  - Adaptive logic based on window size (`width > 1200px` → dual pane, etc.).
  - Support various DPIs and aspect ratios.
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
- Provide a complete **README.md** with all configuration steps, how to build and run on Android/Windows/macOS/Linux, release optimizations for minimal size.
---
make sure its without Error clean code,production ready, ready to execute,provide it in .zip

