// src/main.rs
//
// Android Calculator – Main entry point
// Wires the Slint UI to the Rust calculator, history, and converter backends.

mod calculator;
mod converter;
mod error;
mod history;
mod scientific;

use calculator::{format_result, Calculator};
use converter::{default_units, smart_convert};
use history::HistoryManager;

use std::cell::RefCell;
use std::rc::Rc;

slint::include_modules!();

// ── App State ────────────────────────────────────────────────────────────────

struct AppState {
    /// Current expression string being built
    expression: String,
    /// Whether the display is showing a final result (next digit starts fresh)
    result_mode: bool,
    /// Last evaluated result (for chaining)
    last_result: Option<f64>,
    calc: Calculator,
    history: HistoryManager,
    // Converter state
    conv_input: String,
    conv_category: String,
    conv_from: String,
    conv_to: String,
}

impl AppState {
    fn new() -> Self {
        Self {
            expression: String::new(),
            result_mode: false,
            last_result: None,
            calc: Calculator::new(),
            history: HistoryManager::new(),
            conv_input: "0".to_string(),
            conv_category: "Length".to_string(),
            conv_from: "Meters".to_string(),
            conv_to: "Feet".to_string(),
        }
    }

    fn current_display(&self) -> (&str, &str) {
        // Returns (expression, result) for display
        (&self.expression, "")
    }

    /// Handle a button press from Basic or Scientific tab
    fn handle_calc_key(&mut self, key: &str) -> (String, String, bool) {
        match key {
            // Clear
            "C" => {
                self.expression.clear();
                self.result_mode = false;
                self.last_result = None;
                return ("".to_string(), "0".to_string(), false);
            }

            // Backspace
            "⌫" => {
                if self.result_mode {
                    self.expression.clear();
                    self.result_mode = false;
                } else {
                    // Remove last character(s) – handle multi-byte
                    let mut chars = self.expression.chars().collect::<Vec<_>>();
                    if !chars.is_empty() {
                        chars.pop();
                        self.expression = chars.into_iter().collect();
                    }
                }
            }

            // Evaluate
            "=" => {
                let expr = self.expression.clone();
                if expr.is_empty() { return ("".to_string(), "0".to_string(), false); }
                match self.calc.evaluate(&expr) {
                    Ok(val) => {
                        let result_str = format_result(val);
                        self.history.add(&expr, &result_str);
                        self.last_result = Some(val);
                        self.result_mode = true;
                        return (expr, result_str, false);
                    }
                    Err(e) => {
                        let err_str = e.to_string();
                        self.result_mode = true;
                        return (expr, err_str, true);
                    }
                }
            }

            // Negate
            "±" => {
                if self.expression.is_empty() {
                    self.expression.push('-');
                } else if self.expression.starts_with('-') {
                    self.expression = self.expression[1..].to_string();
                } else {
                    self.expression.insert(0, '-');
                }
                self.result_mode = false;
            }

            // Percent – append as operator
            "%" => {
                if self.result_mode { self.result_mode = false; }
                self.expression.push('%');
            }

            // Operators
            "÷" => { self.append_op('/'); }
            "×" => { self.append_op('*'); }
            "−" => { self.append_op('-'); }
            "+" => { self.append_op('+'); }
            "^" | "xʸ" => { self.append_op('^'); }

            // Decimal
            "." => {
                if self.result_mode { self.expression.clear(); self.result_mode = false; }
                // Avoid double-dot in current number segment
                if !self.last_segment_has_dot() {
                    if self.expression.is_empty() || self.is_last_op() {
                        self.expression.push('0');
                    }
                    self.expression.push('.');
                }
            }

            // Functions (scientific)
            "sin"  => self.append_func("sin"),
            "cos"  => self.append_func("cos"),
            "tan"  => self.append_func("tan"),
            "log"  => self.append_func("log"),
            "ln"   => self.append_func("ln"),
            "√x"   => self.append_func("sqrt"),
            "∛x"   => self.append_func("cbrt"),
            "x²"   => self.expression.push_str("^2"),
            "1/x"  => self.append_func("inv"),
            "!"    => self.expression.push('!'),
            "π"    => {
                if self.result_mode { self.expression.clear(); self.result_mode = false; }
                self.expression.push_str("pi");
            }
            "e"    => {
                if self.result_mode { self.expression.clear(); self.result_mode = false; }
                self.expression.push('e');
            }
            "("    => {
                if self.result_mode { self.expression.clear(); self.result_mode = false; }
                self.expression.push('(');
            }
            ")"    => self.expression.push(')'),

            // Digits 0-9
            digit if digit.len() == 1 && digit.chars().all(|c| c.is_ascii_digit()) => {
                if self.result_mode {
                    self.expression.clear();
                    self.result_mode = false;
                }
                self.expression.push_str(digit);
            }

            _ => {}
        }

        // Live evaluation for preview
        let preview = if !self.expression.is_empty() && !self.result_mode {
            match self.calc.evaluate(&self.expression) {
                Ok(v)  => format_result(v),
                Err(_) => String::new(),
            }
        } else {
            String::new()
        };

        let display_result = if preview.is_empty() {
            if self.expression.is_empty() { "0".to_string() } else { self.expression.clone() }
        } else {
            preview
        };

        (self.expression.clone(), display_result, false)
    }

    fn append_op(&mut self, op: char) {
        if self.result_mode { self.result_mode = false; }
        // Replace trailing operator if present
        if !self.expression.is_empty() && self.is_last_op() {
            self.expression.pop();
        }
        self.expression.push(op);
    }

    fn append_func(&mut self, func: &str) {
        if self.result_mode { self.result_mode = false; }
        self.expression.push_str(&format!("{func}("));
    }

    fn is_last_op(&self) -> bool {
        self.expression.ends_with(['+', '-', '*', '/', '^', '('])
    }

    fn last_segment_has_dot(&self) -> bool {
        let ops = ['+', '-', '*', '/', '^', '(', ')'];
        let seg: String = self.expression
            .chars()
            .rev()
            .take_while(|c| !ops.contains(c))
            .collect();
        seg.contains('.')
    }

    // ── Converter helpers ────────────────────────────────────────────────────

    fn handle_conv_key(&mut self, key: &str) -> String {
        match key {
            "C" => { self.conv_input = "0".to_string(); }
            "⌫" => {
                if self.conv_input.len() > 1 {
                    self.conv_input.pop();
                } else {
                    self.conv_input = "0".to_string();
                }
            }
            "." => {
                if !self.conv_input.contains('.') {
                    self.conv_input.push('.');
                }
            }
            "±" => {
                if self.conv_input.starts_with('-') {
                    self.conv_input = self.conv_input[1..].to_string();
                } else {
                    self.conv_input.insert(0, '-');
                }
            }
            "=" => {} // Nothing extra for converter
            digit if digit.len() == 1 && digit.chars().all(|c| c.is_ascii_digit()) => {
                if self.conv_input == "0" {
                    self.conv_input = digit.to_string();
                } else {
                    self.conv_input.push_str(digit);
                }
            }
            _ => {}
        }
        self.run_conversion()
    }

    fn run_conversion(&self) -> String {
        let val: f64 = self.conv_input.parse().unwrap_or(0.0);
        match smart_convert(val, &self.conv_from, &self.conv_to, &self.conv_category) {
            Some(result) => format_result(result),
            None => "—".to_string(),
        }
    }

    fn swap_units(&mut self) -> String {
        std::mem::swap(&mut self.conv_from, &mut self.conv_to);
        self.run_conversion()
    }

    fn set_category(&mut self, cat: String) -> (String, String, String) {
        self.conv_category = cat.clone();
        let (from, to) = default_units(&cat);
        self.conv_from = from.to_string();
        self.conv_to   = to.to_string();
        self.conv_input = "0".to_string();
        let result = self.run_conversion();
        (self.conv_from.clone(), self.conv_to.clone(), result)
    }
}

// ── Main ─────────────────────────────────────────────────────────────────────

fn main() -> Result<(), slint::PlatformError> {
    env_logger::init();

    let ui = AppWindow::new()?;
    let state = Rc::new(RefCell::new(AppState::new()));

    // ── Populate history on startup ──────────────────────────────────────────
    {
        let entries = state.borrow().history.entries().to_vec();
        let slint_entries: Vec<HistoryEntry> = entries
            .into_iter()
            .map(|e| HistoryEntry {
                expression: e.expression.into(),
                result:     e.result.into(),
                timestamp:  e.timestamp.into(),
            })
            .collect();
        ui.set_history_entries(slint_entries.as_slice().into());
    }

    // ── Set initial converter state ──────────────────────────────────────────
    {
        let s = state.borrow();
        ui.set_conv_from(s.conv_from.as_str().into());
        ui.set_conv_to(s.conv_to.as_str().into());
        ui.set_conv_category(s.conv_category.as_str().into());
        ui.set_conv_input(s.conv_input.as_str().into());
        let result = s.run_conversion();
        ui.set_conv_result(result.into());
    }

    // ── Basic tab ────────────────────────────────────────────────────────────
    {
        let ui_handle = ui.as_weak();
        let state_clone = state.clone();
        ui.on_basic_button_pressed(move |key| {
            let (expr, result, err) = state_clone.borrow_mut().handle_calc_key(key.as_str());
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_display_expr(expr.into());
                ui.set_display_result(result.into());
                ui.set_display_error(err);
                // Refresh history
                let entries = state_clone.borrow().history.entries().to_vec();
                let slint_entries: Vec<HistoryEntry> = entries
                    .into_iter()
                    .map(|e| HistoryEntry {
                        expression: e.expression.into(),
                        result:     e.result.into(),
                        timestamp:  e.timestamp.into(),
                    })
                    .collect();
                ui.set_history_entries(slint_entries.as_slice().into());
            }
        });
    }

    // ── Scientific tab ───────────────────────────────────────────────────────
    {
        let ui_handle = ui.as_weak();
        let state_clone = state.clone();
        ui.on_sci_button_pressed(move |key| {
            let (expr, result, err) = state_clone.borrow_mut().handle_calc_key(key.as_str());
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_display_expr(expr.into());
                ui.set_display_result(result.into());
                ui.set_display_error(err);
                let entries = state_clone.borrow().history.entries().to_vec();
                let slint_entries: Vec<HistoryEntry> = entries
                    .into_iter()
                    .map(|e| HistoryEntry {
                        expression: e.expression.into(),
                        result:     e.result.into(),
                        timestamp:  e.timestamp.into(),
                    })
                    .collect();
                ui.set_history_entries(slint_entries.as_slice().into());
            }
        });

        let ui_handle2 = ui.as_weak();
        let state_clone2 = state.clone();
        ui.on_sci_toggle_deg_rad(move || {
            let mut s = state_clone2.borrow_mut();
            s.calc.use_degrees = !s.calc.use_degrees;
            if let Some(ui) = ui_handle2.upgrade() {
                ui.set_is_deg(s.calc.use_degrees);
            }
        });
    }

    // ── History tab ──────────────────────────────────────────────────────────
    {
        let ui_handle = ui.as_weak();
        let state_clone = state.clone();
        ui.on_history_recall(move |idx| {
            let entries = state_clone.borrow().history.entries().to_vec();
            if let Some(entry) = entries.get(idx as usize) {
                let expr = entry.expression.clone();
                let result = entry.result.clone();
                if let Some(ui) = ui_handle.upgrade() {
                    ui.set_display_expr(expr.into());
                    ui.set_display_result(result.into());
                    ui.set_display_error(false);
                    ui.set_active_tab(0); // Switch to Basic tab
                }
                state_clone.borrow_mut().expression = entries[idx as usize].expression.clone();
                state_clone.borrow_mut().result_mode = true;
            }
        });

        let ui_handle2 = ui.as_weak();
        let state_clone2 = state.clone();
        ui.on_history_delete(move |idx| {
            state_clone2.borrow_mut().history.delete(idx as usize);
            let entries = state_clone2.borrow().history.entries().to_vec();
            let slint_entries: Vec<HistoryEntry> = entries
                .into_iter()
                .map(|e| HistoryEntry {
                    expression: e.expression.into(),
                    result:     e.result.into(),
                    timestamp:  e.timestamp.into(),
                })
                .collect();
            if let Some(ui) = ui_handle2.upgrade() {
                ui.set_history_entries(slint_entries.as_slice().into());
            }
        });

        let ui_handle3 = ui.as_weak();
        let state_clone3 = state.clone();
        ui.on_history_clear_all(move || {
            state_clone3.borrow_mut().history.clear();
            if let Some(ui) = ui_handle3.upgrade() {
                ui.set_history_entries(Default::default());
            }
        });
    }

    // ── Converter tab ────────────────────────────────────────────────────────
    {
        let ui_handle = ui.as_weak();
        let state_clone = state.clone();
        ui.on_conv_value(move |key| {
            let result = state_clone.borrow_mut().handle_conv_key(key.as_str());
            if let Some(ui) = ui_handle.upgrade() {
                let input = state_clone.borrow().conv_input.clone();
                ui.set_conv_input(input.into());
                ui.set_conv_result(result.into());
            }
        });

        let ui_handle2 = ui.as_weak();
        let state_clone2 = state.clone();
        ui.on_conv_swap(move || {
            let result = state_clone2.borrow_mut().swap_units();
            if let Some(ui) = ui_handle2.upgrade() {
                let (from, to) = {
                    let s = state_clone2.borrow();
                    (s.conv_from.clone(), s.conv_to.clone())
                };
                ui.set_conv_from(from.into());
                ui.set_conv_to(to.into());
                ui.set_conv_result(result.into());
            }
        });

        let ui_handle3 = ui.as_weak();
        let state_clone3 = state.clone();
        ui.on_conv_category_changed(move |cat| {
            let (from, to, result) = state_clone3.borrow_mut().set_category(cat.to_string());
            if let Some(ui) = ui_handle3.upgrade() {
                ui.set_conv_from(from.into());
                ui.set_conv_to(to.into());
                ui.set_conv_input("0".into());
                ui.set_conv_result(result.into());
                ui.set_conv_category(cat);
            }
        });
    }

    ui.run()
}
