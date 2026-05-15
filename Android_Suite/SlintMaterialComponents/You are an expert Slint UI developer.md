You are an expert Slint UI developer with deep knowledge of Material Design 3 (M3) specifications, responsive layouts, and production-grade embedded/desktop touch UIs.

Create a complete, modular, and extensible Slint component library that implements **Material 3 Design** components using official Slint Material Components where available (import @material/...), and custom implementations for others. Focus on **responsive, touch-first design** that works from mobile phones to large 65–98 inch touch displays from a **single codebase/design**.

### Core Requirements:
- **Responsive from a single design**: Use Slint's `VerticalLayout`, `HorizontalLayout`, `GridLayout`, `FlexLayout` (if available), relative units (`%`, `rem`, logical pixels), `min-width`/`max-width`, `stretch` properties, and adaptive logic (e.g., via `width` bindings or states). Avoid fixed pixel sizes for containers.
- **Scalability**: Support automatic adaptation to different resolutions, DPIs, and physical sizes (mobile portrait/landscape → tablet → large kiosk/TV). Use logical sizing, minimum touch targets (≥48dp/ logical pixels recommended), and fluid typography/spacing.
- **Touch-first**: Large, accessible touch targets (min 48x48 logical px, preferably 56+ for large screens). Good hit areas, ripple effects (or Slint equivalent animations), appropriate spacing for fingers on big displays.
- **Production-ready & Extendable**:
  - Clean, well-commented `.slint` code with public properties, callbacks, and states.
  - Theme support (light/dark, custom colors via M3 color roles/schemes).
  - Variants (e.g., Button: filled, outlined, text, elevated, tonal).
  - Accessibility: focus, keyboard navigation where applicable, proper contrast.
  - Performance: efficient layouts, minimal nesting, reusable primitives.
  - Error handling and default sensible values.
  - Modular: Base primitives (Card, Surface, Typography scales) + higher components.

### Components to Implement (start with these, make extensible):
1. **Core Primitives**: Surface (with elevation/shadow), Typography (M3 scales that adapt), Spacing tokens, Icon (with Material Symbols support if possible).
2. **Button** family (Filled, Outlined, Text, FAB, Extended FAB) with states (enabled, disabled, pressed, hovered).
3. **Card** (with variants: elevated, outlined, clickable).
4. **Top App Bar / Navigation Bar** (responsive: collapses or switches layout on small screens).
5. **Navigation Drawer / Rail / Bar** (adaptive based on screen width).
6. **Text Field / Input** (filled, outlined, with labels, errors, prefixes).
7. **List / List Item** (single-line, two-line, with leading/trailing).
8. **Chip / Filter Chip / Suggestion Chip**.
9. **Bottom Sheet / Dialog** (modal, responsive sizing).
10. **Snackbar / Toast**.
11. gradieant shapes and colors
12. Bottom App Bar

### Layout Strategy:
- Root uses a responsive container (e.g., `ResponsiveLayout` component you define) that switches between mobile (column-heavy), tablet, and large-screen (grid/multi-column) modes based on `width > xx px` thresholds or aspect ratio.
- Example adaptive logic: Use `if width < 600px { mobile layout } else if width < 1200px { tablet } else { desktop/large }`.
- Nest layouts properly; use `padding`, `spacing`, `alignment`, and `stretch` factors.

### Additional Details:
- Use modern Slint syntax (latest version).
- Include example `AppWindow` that demonstrates all components in a realistic responsive dashboard/app shell.
- Support theming via exported properties or a global `Theme` struct.
- Add comments for customization points.
- Make components importable (e.g., via libraries).
- Optimize for touch on large screens: increase padding/touch targets proportionally or via a global `density` / `scale` property.
