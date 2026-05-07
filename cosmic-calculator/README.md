# COSMIC Calculator

A full-featured calculator application for the COSMIC desktop environment, built with **Rust** and **libcosmic**.

Inspired by KDE Kalk, it offers a standard calculator, scientific functions, unit converter, programmer calculator (with bitwise ops and bit-panel), full history, undo/redo, and clipboard support.

---

## Features

### 🧮 Calculator Mode
- Basic arithmetic: `+`, `-`, `×`, `÷`
- Scientific: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `log`, `ln`, `sqrt`, `x²`, `x³`, `xⁿ`, `1/x`, `|x|`, `x!`
- Constants: `π`, `e`
- Memory: `MS`, `MR`, `M+`, `M-`, `MC`
- Angle modes: Degrees / Radians / Gradians
- Parentheses, percentage, EXP notation
- Thousands-separator formatting

### 🔄 Converter Mode
12 conversion categories:
- Length, Area, Volume, Mass, Temperature, Speed, Time, Data Storage, Energy, Pressure, Angle, Fuel Economy
- Swap button to invert conversion instantly
- Quick reference table for all units in selected category

### 💻 Programmer Mode
- Bases: **HEX**, **DEC**, **OCT**, **BIN**
- Bit widths: 8-bit, 16-bit, 32-bit, 64-bit
- Bitwise ops: `AND`, `OR`, `XOR`, `NOT`, `<<`, `>>`
- Interactive bit panel — click any bit to toggle it
- All four bases shown simultaneously

### 📋 History
- Scrollable calculation history (up to 100 entries)
- Click any entry to restore expression
- Clear all button

### ⌨️ Keyboard Shortcuts
| Key | Action |
|-----|--------|
| `0–9` | Digit input |
| `+`, `-`, `*`, `/` | Operators |
| `.` | Decimal point |
| `Enter` / `Numpad Enter` | Evaluate |
| `Backspace` | Delete last character |
| `Delete` / `Escape` | Clear all |
| `%` | Percentage |
| `^` | Power |
| `(`, `)` | Parentheses |
| `!` | Factorial |
| `Ctrl+Z` | Undo |
| `Ctrl+Y` / `Ctrl+Shift+Z` | Redo |
| `Ctrl+C` | Copy result |
| `Ctrl+X` | Cut expression |

### ⚙️ Settings
- Color theme: System / Light / Dark
- Font size: Small / Medium / Large
- Angle unit default
- Thousands separator toggle
- Show/hide history panel
- Show/hide bit panel (programmer mode)

---

## Prerequisites

### System packages (Ubuntu/Pop!_OS/Debian)

```bash
sudo apt update
sudo apt install -y \
    build-essential \
    curl \
    pkg-config \
    libwayland-dev \
    libxkbcommon-dev \
    libvulkan-dev \
    libinput-dev \
    libudev-dev \
    libdbus-1-dev \
    libssl-dev \
    libfontconfig1-dev \
    libfreetype-dev \
    libseat-dev \
    libgbm-dev \
    libdrm-dev \
    cmake \
    clang \
    libclang-dev \
    mesa-vulkan-drivers \
    libgl1-mesa-dev
```

### System packages (Fedora)

```bash
sudo dnf install -y \
    gcc cmake clang clang-devel \
    wayland-devel libxkbcommon-devel \
    vulkan-loader-devel \
    dbus-devel openssl-devel \
    fontconfig-devel freetype-devel \
    libseat-devel mesa-libgbm-devel \
    libdrm-devel libinput-devel \
    systemd-devel
```

### Rust (version 1.75+ required, 1.93.1 recommended)

```bash
# Install rustup if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install or update to specific version
rustup toolchain install 1.93.1
rustup default 1.93.1

# Verify
rustc --version   # should show 1.93.1
cargo --version
```

---

## Build & Run

### Development (debug build)

```bash
# Clone or enter the project directory
cd cosmic-calculator

# Build
cargo build

# Run
cargo run
```

### Release (optimized)

```bash
cargo build --release

# Binary will be at:
./target/release/cosmic-calculator

# Run directly
./target/release/cosmic-calculator
```

### Install system-wide

```bash
cargo build --release

# Install binary
sudo install -Dm755 target/release/cosmic-calculator /usr/local/bin/cosmic-calculator

# Install desktop entry
sudo install -Dm644 com.cosmic.calculator.desktop \
    /usr/share/applications/com.cosmic.calculator.desktop

# (Optional) Install icon if you have one
# sudo install -Dm644 data/icons/com.cosmic.calculator.svg \
#     /usr/share/icons/hicolor/scalable/apps/com.cosmic.calculator.svg

# Update desktop database
sudo update-desktop-database /usr/share/applications/
```

---

## Project Structure

```
cosmic-calculator/
├── Cargo.toml
├── com.cosmic.calculator.desktop
├── README.md
└── src/
    ├── main.rs              # Entry point
    ├── app.rs               # App state, messages, update logic
    ├── app_views.rs         # All view/UI rendering functions
    ├── calculator/
    │   ├── mod.rs
    │   ├── engine.rs        # Expression parser & evaluator
    │   └── display.rs       # Number formatting utilities
    ├── converter/
    │   └── mod.rs           # Unit conversion engine (12 categories)
    ├── programmer/
    │   └── mod.rs           # Programmer calc, bitwise ops, base conversion
    ├── settings/
    │   └── mod.rs           # App settings & persistence
    └── history/
        └── mod.rs           # Calculation history
```

---

# COSMIC Calculator — Architecture (MVU / Elm Pattern)

The app follows the **Model-View-Update (MVU)** pattern, also known as the Elm Architecture.
All state lives in a single immutable model; the UI is a pure function of that state;
and all mutations flow through typed `Message` variants dispatched to `update()`.

---

## Module Responsibilities

| File | Responsibility |
|---|---|
| `main.rs` | Entry point, boots the COSMIC app runtime |
| `app.rs` | All state (`CosmicCalculator`), all `Message` variants, `update()` logic, keyboard handler, undo/redo stack |
| `app_views.rs` | All `view_*()` rendering functions for every mode |
| `calculator/engine.rs` | Recursive-descent expression parser & evaluator (full operator precedence, trig, log, constants) |
| `calculator/display.rs` | Thousands-separator number formatting |
| `converter/mod.rs` | 12-category unit converter with 100+ units |
| `programmer/mod.rs` | Multi-base calculator, bitwise operator parser, togglable bit panel |
| `settings/mod.rs` | Typed settings structs with load/save hooks |
| `history/mod.rs` | Circular history buffer with expressions and results |

---

## MVU Data Flow

```
User interaction / Keyboard event
           │
           ▼
      Message enum
           │
           ▼
      update() ──► mutates Model (CosmicCalculator)
           │
           ▼
       view() ──► renders UI from Model (pure function)
           │
           ▼
      COSMIC / iced renders frame
```

---

## Key Design Decisions

| Concern | Approach |
|---|---|
| **State management** | Single `CosmicCalculator` struct owns all sub-states |
| **Expression evaluation** | Recursive-descent parser with explicit operator precedence |
| **Undo / Redo** | Dual-stack `UndoStack` (past / future), saves expression string snapshots |
| **Keyboard input** | `iced::keyboard::on_key_press` subscription mapped to `Message::CalcKeyPressed` |
| **Settings persistence** | `AppSettings::load()` / `save()` — pluggable (file-backed in production) |
| **History** | Fixed-capacity `Vec` with front-eviction; click-to-restore bound to `Message::HistoryUse` |
| **Bit panel** | Bit slice rendered as toggle buttons; each fires `Message::ProgToggleBit(index)` |
| **Clipboard** | `iced::clipboard::write()` command for copy; paste via `Message::Paste(String)` |



The app uses the **COSMIC Application Framework** (libcosmic), which is built on top of **iced** (a cross-platform Rust GUI library). The architecture follows the **Elm/MVU pattern**:

1. **Model** — `CosmicCalculator` struct holds all state
2. **Update** — `Message` enum drives state transitions via `update()`  
3. **View** — `view()` renders the UI based on current state

The expression evaluator is a **recursive descent parser** that supports:
- Operator precedence (`^` > `*`/`/` > `+`/`-`)
- Unary prefix functions (sin, cos, sqrt, etc.)
- Parentheses for grouping
- Right-associative exponentiation

---

## Troubleshooting

**`libcosmic` not found / build fails:**
Make sure all system dependencies are installed. libcosmic is fetched automatically from GitHub via Cargo.

**Wayland compositor required:**
libcosmic targets Wayland by default. If running under X11, try:
```bash
WAYLAND_DISPLAY=wayland-0 cargo run
# or
XDG_SESSION_TYPE=wayland cargo run
```

**Slow first build:**
libcosmic and its dependencies (iced, wgpu, etc.) take 5–15 minutes to compile on first build. Subsequent builds are fast.

**Vulkan errors:**
Install Mesa Vulkan drivers: `sudo apt install mesa-vulkan-drivers vulkan-tools`

---

## License

MIT License — feel free to use, modify, and distribute.
