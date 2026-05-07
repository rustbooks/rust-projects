use cosmic::app::{Command, Core};
use cosmic::iced::keyboard::{Key, Modifiers};
use cosmic::iced::{self, Alignment, Length, Subscription};
use cosmic::iced_core::SmolStr;
use cosmic::widget::nav_bar;
use cosmic::{cosmic_theme, theme, Application, ApplicationExt, Apply, Element};

use crate::calculator::{CalcEngine};
use crate::calculator::engine::{AngleMode, format_number};
use crate::calculator::display::format_display;
use crate::converter::{Converter, ConverterCategory};
use crate::programmer::{ProgrammerCalc, Base, BitWidth, BitwiseOp};
use crate::settings::{AppSettings, Theme, AngleUnit, FontSize};
use crate::history::{History, HistoryEntry};

// ── Pages ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Calculator,
    Converter,
    Programmer,
    Settings,
}

// ── Messages ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Message {
    // Navigation
    NavPage(Page),

    // Calculator
    CalcButton(CalcAction),
    CalcKeyPressed(Key, Modifiers),

    // Converter
    ConvCategoryChanged(usize),
    ConvFromUnitChanged(usize),
    ConvToUnitChanged(usize),
    ConvInputChanged(String),
    ConvSwap,

    // Programmer
    ProgButton(ProgAction),
    ProgBaseChanged(Base),
    ProgBitWidthChanged(BitWidth),
    ProgToggleBit(usize),

    // Settings
    SettingTheme(usize),
    SettingAngle(usize),
    SettingFontSize(usize),
    SettingThousandsSep(bool),
    SettingShowHistory(bool),
    SettingShowBitPanel(bool),
    SettingRound(bool),

    // History
    HistoryClear,
    HistoryUse(String),
    HistoryToggle,

    // Edit operations
    Undo,
    Redo,
    Cut,
    Copy,
    Paste(String),

    // Misc
    AboutDialog,
    ClipboardRead(String),
}

#[derive(Debug, Clone)]
pub enum CalcAction {
    Digit(char),
    Op(char),
    Dot,
    Clear,
    ClearEntry,
    Backspace,
    Equals,
    Negate,
    Percent,
    Sqrt,
    Square,
    Cube,
    Inverse,
    Power,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Log,
    Ln,
    Pi,
    E,
    Factorial,
    Abs,
    MemStore,
    MemRecall,
    MemAdd,
    MemSub,
    MemClear,
    LeftParen,
    RightParen,
    AngleMode,
    Exp,
}

#[derive(Debug, Clone)]
pub enum ProgAction {
    Digit(char),
    Op(char),
    Clear,
    Backspace,
    Equals,
    Bitwise(BitwiseOp),
    LeftParen,
    RightParen,
    Negate,
    Mod,
}

// ── Undo/Redo stack ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct UndoStack {
    past: Vec<String>,
    future: Vec<String>,
}

impl UndoStack {
    fn new() -> Self {
        Self { past: Vec::new(), future: Vec::new() }
    }

    fn push(&mut self, state: String) {
        self.past.push(state);
        self.future.clear();
    }

    fn undo(&mut self, current: &str) -> Option<String> {
        if let Some(prev) = self.past.pop() {
            self.future.push(current.to_string());
            Some(prev)
        } else {
            None
        }
    }

    fn redo(&mut self, current: &str) -> Option<String> {
        if let Some(next) = self.future.pop() {
            self.past.push(current.to_string());
            Some(next)
        } else {
            None
        }
    }
}

// ── App State ─────────────────────────────────────────────────────────────────

pub struct CosmicCalculator {
    core: Core,
    nav: nav_bar::Model,
    page: Page,

    // Calculator
    calc: CalcEngine,
    calc_undo: UndoStack,
    show_scientific: bool,

    // Converter
    converter: Converter,
    conv_categories: Vec<ConverterCategory>,

    // Programmer
    prog: ProgrammerCalc,

    // Settings
    settings: AppSettings,

    // History
    history: History,
    show_history: bool,

    // Clipboard
    clipboard_content: String,
}

impl Application for CosmicCalculator {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "com.cosmic.calculator";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: ()) -> (Self, Command<Message>) {
        let mut nav = nav_bar::Model::default();

        nav.insert()
            .icon(cosmic::widget::icon::from_name("accessories-calculator-symbolic"))
            .text("Calculator")
            .data(Page::Calculator)
            .activate();

        nav.insert()
            .icon(cosmic::widget::icon::from_name("accessories-calculator-symbolic"))
            .text("Converter")
            .data(Page::Converter);

        nav.insert()
            .icon(cosmic::widget::icon::from_name("accessories-calculator-symbolic"))
            .text("Programmer")
            .data(Page::Programmer);

        nav.insert()
            .icon(cosmic::widget::icon::from_name("preferences-system-symbolic"))
            .text("Settings")
            .data(Page::Settings);

        let settings = AppSettings::load();

        let mut calc = CalcEngine::new();
        calc.angle_mode = match settings.angle_mode {
            AngleUnit::Degrees => AngleMode::Degrees,
            AngleUnit::Radians => AngleMode::Radians,
            AngleUnit::Gradians => AngleMode::Gradians,
        };

        let app = CosmicCalculator {
            core,
            nav,
            page: Page::Calculator,
            calc,
            calc_undo: UndoStack::new(),
            show_scientific: false,
            converter: Converter::new(),
            conv_categories: ConverterCategory::all(),
            prog: ProgrammerCalc::new(),
            settings,
            history: History::new(),
            show_history: true,
            clipboard_content: String::new(),
        };

        (app, Command::none())
    }

    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav)
    }

    fn on_nav_select(&mut self, id: nav_bar::Id) -> Command<Message> {
        self.nav.activate(id);
        if let Some(page) = self.nav.active_data::<Page>() {
            self.page = page.clone();
        }
        Command::none()
    }

    fn header_start(&self) -> Vec<Element<Message>> {
        vec![]
    }

    fn header_end(&self) -> Vec<Element<Message>> {
        use cosmic::widget::*;
        vec![
            button::icon(icon::from_name("edit-undo-symbolic"))
                .on_press(Message::Undo)
                .into(),
            button::icon(icon::from_name("edit-redo-symbolic"))
                .on_press(Message::Redo)
                .into(),
        ]
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::keyboard::on_key_press(|key, modifiers| {
            Some(Message::CalcKeyPressed(key, modifiers))
        })
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NavPage(page) => {
                self.page = page;
            }

            // ── Calculator Messages ──────────────────────────────────
            Message::CalcButton(action) => {
                self.handle_calc_action(action);
            }

            Message::CalcKeyPressed(key, modifiers) => {
                self.handle_key(key, modifiers);
            }

            // ── Converter Messages ───────────────────────────────────
            Message::ConvCategoryChanged(idx) => {
                self.converter.category = self.conv_categories[idx].clone();
                self.converter.from_unit_idx = 0;
                self.converter.to_unit_idx = 1;
                self.converter.convert();
            }
            Message::ConvFromUnitChanged(idx) => {
                self.converter.from_unit_idx = idx;
                self.converter.convert();
            }
            Message::ConvToUnitChanged(idx) => {
                self.converter.to_unit_idx = idx;
                self.converter.convert();
            }
            Message::ConvInputChanged(s) => {
                self.converter.input_value = s;
                self.converter.convert();
            }
            Message::ConvSwap => {
                self.converter.swap();
            }

            // ── Programmer Messages ──────────────────────────────────
            Message::ProgButton(action) => {
                self.handle_prog_action(action);
            }
            Message::ProgBaseChanged(base) => {
                self.prog.base = base;
                self.prog.clear();
            }
            Message::ProgBitWidthChanged(bw) => {
                self.prog.bit_width = bw;
                self.prog.evaluate();
            }
            Message::ProgToggleBit(bit) => {
                self.prog.toggle_bit(bit);
            }

            // ── Settings Messages ────────────────────────────────────
            Message::SettingTheme(idx) => {
                self.settings.theme = [Theme::System, Theme::Light, Theme::Dark][idx].clone();
                self.settings.save();
            }
            Message::SettingAngle(idx) => {
                self.settings.angle_mode = AngleUnit::all()[idx].clone();
                self.calc.angle_mode = match self.settings.angle_mode {
                    AngleUnit::Degrees => AngleMode::Degrees,
                    AngleUnit::Radians => AngleMode::Radians,
                    AngleUnit::Gradians => AngleMode::Gradians,
                };
                self.settings.save();
            }
            Message::SettingFontSize(idx) => {
                self.settings.font_size = FontSize::all()[idx].clone();
                self.settings.save();
            }
            Message::SettingThousandsSep(v) => {
                self.settings.thousands_separator = v;
                self.settings.save();
            }
            Message::SettingShowHistory(v) => {
                self.settings.show_history = v;
                self.show_history = v;
                self.settings.save();
            }
            Message::SettingShowBitPanel(v) => {
                self.settings.show_bit_panel = v;
                self.settings.save();
            }
            Message::SettingRound(v) => {
                self.settings.round_results = v;
                self.settings.save();
            }

            // ── History ──────────────────────────────────────────────
            Message::HistoryClear => {
                self.history.clear();
            }
            Message::HistoryUse(expr) => {
                self.calc.expression = expr;
            }
            Message::HistoryToggle => {
                self.show_history = !self.show_history;
            }

            // ── Edit Operations ──────────────────────────────────────
            Message::Undo => {
                if let Some(prev) = self.calc_undo.undo(&self.calc.expression) {
                    self.calc.expression = prev;
                }
            }
            Message::Redo => {
                if let Some(next) = self.calc_undo.redo(&self.calc.expression) {
                    self.calc.expression = next;
                }
            }
            Message::Cut => {
                self.clipboard_content = self.calc.expression.clone();
                self.calc.clear();
            }
            Message::Copy => {
                self.clipboard_content = self.calc.result.clone();
                return iced::clipboard::write(self.calc.result.clone());
            }
            Message::Paste(s) => {
                self.calc_undo.push(self.calc.expression.clone());
                self.calc.expression.push_str(&s);
            }
            Message::ClipboardRead(s) => {
                self.clipboard_content = s;
            }

            Message::AboutDialog => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        match self.page {
            Page::Calculator => self.view_calculator(),
            Page::Converter => self.view_converter(),
            Page::Programmer => self.view_programmer(),
            Page::Settings => self.view_settings(),
        }
    }
}

// ── Calculator Action Handler ──────────────────────────────────────────────────

impl CosmicCalculator {
    fn handle_calc_action(&mut self, action: CalcAction) {
        // Save undo state before mutation
        let prev = self.calc.expression.clone();

        match action {
            CalcAction::Digit(c) => self.calc.append(&c.to_string()),
            CalcAction::Dot => {
                // Only add dot if last segment doesn't have one
                let parts: Vec<&str> = self.calc.expression.split(|c: char| !c.is_ascii_digit() && c != '.' && c != '-').collect();
                if !parts.last().unwrap_or(&"").contains('.') {
                    if self.calc.expression.is_empty() || !self.calc.expression.chars().last().map(|c| c.is_ascii_digit() || c == '.').unwrap_or(false) {
                        self.calc.append("0");
                    }
                    self.calc.append(".");
                }
            }
            CalcAction::Op(op) => self.calc.append(&op.to_string()),
            CalcAction::Clear => { self.calc.clear(); return; }
            CalcAction::ClearEntry => self.calc.clear_entry(),
            CalcAction::Backspace => self.calc.backspace(),
            CalcAction::Equals => {
                let expr = self.calc.expression.clone();
                if let Some(result) = self.calc.evaluate() {
                    self.history.push(expr.clone(), format_number(result), "Calculator".to_string());
                    self.calc.expression = format_number(result);
                }
                return;
            }
            CalcAction::Negate => self.calc.negate(),
            CalcAction::Percent => self.calc.percentage(),
            CalcAction::Sqrt => self.calc.append("sqrt("),
            CalcAction::Square => self.calc.append("^2"),
            CalcAction::Cube => self.calc.append("^3"),
            CalcAction::Inverse => {
                let expr = self.calc.expression.clone();
                self.calc.expression = format!("1/({})", expr);
            }
            CalcAction::Power => self.calc.append("^"),
            CalcAction::Sin => self.calc.append("sin("),
            CalcAction::Cos => self.calc.append("cos("),
            CalcAction::Tan => self.calc.append("tan("),
            CalcAction::Asin => self.calc.append("asin("),
            CalcAction::Acos => self.calc.append("acos("),
            CalcAction::Atan => self.calc.append("atan("),
            CalcAction::Log => self.calc.append("log("),
            CalcAction::Ln => self.calc.append("ln("),
            CalcAction::Pi => self.calc.append("π"),
            CalcAction::E => self.calc.append("e"),
            CalcAction::Factorial => self.calc.append("!"),
            CalcAction::Abs => self.calc.append("abs("),
            CalcAction::MemStore => { self.calc.memory_store(); return; }
            CalcAction::MemRecall => { self.calc.memory_recall(); }
            CalcAction::MemAdd => { self.calc.memory_add(); return; }
            CalcAction::MemSub => { self.calc.memory_subtract(); return; }
            CalcAction::MemClear => { self.calc.memory_clear(); return; }
            CalcAction::LeftParen => self.calc.append("("),
            CalcAction::RightParen => self.calc.append(")"),
            CalcAction::AngleMode => {
                self.calc.angle_mode = match self.calc.angle_mode {
                    AngleMode::Degrees => AngleMode::Radians,
                    AngleMode::Radians => AngleMode::Gradians,
                    AngleMode::Gradians => AngleMode::Degrees,
                };
                return;
            }
            CalcAction::Exp => self.calc.append("×10^"),
        }

        if self.calc.expression != prev {
            self.calc_undo.push(prev);
        }
    }

    fn handle_prog_action(&mut self, action: ProgAction) {
        match action {
            ProgAction::Digit(c) => self.prog.append(&c.to_string()),
            ProgAction::Op(op) => self.prog.append(&op.to_string()),
            ProgAction::Clear => self.prog.clear(),
            ProgAction::Backspace => self.prog.backspace(),
            ProgAction::Equals => self.prog.evaluate(),
            ProgAction::Bitwise(op) => self.prog.apply_bitwise(op),
            ProgAction::LeftParen => self.prog.append("("),
            ProgAction::RightParen => self.prog.append(")"),
            ProgAction::Negate => self.prog.append("-"),
            ProgAction::Mod => self.prog.append(" % "),
        }
    }

    fn handle_key(&mut self, key: Key, modifiers: Modifiers) {
        match &key {
            Key::Character(s) if modifiers.control() => {
                match s.as_str() {
                    "z" => { let _ = self.update(Message::Undo); }
                    "Z" | "y" => { let _ = self.update(Message::Redo); }
                    "c" => { let _ = self.update(Message::Copy); }
                    "x" => { let _ = self.update(Message::Cut); }
                    _ => {}
                }
            }
            Key::Character(s) => {
                match s.as_str() {
                    "0" | "1" | "2" | "3" | "4" |
                    "5" | "6" | "7" | "8" | "9" => {
                        let c = s.chars().next().unwrap();
                        self.handle_calc_action(CalcAction::Digit(c));
                    }
                    "+" => self.handle_calc_action(CalcAction::Op('+')),
                    "-" => self.handle_calc_action(CalcAction::Op('-')),
                    "*" => self.handle_calc_action(CalcAction::Op('×')),
                    "/" => self.handle_calc_action(CalcAction::Op('÷')),
                    "." | "," => self.handle_calc_action(CalcAction::Dot),
                    "%" => self.handle_calc_action(CalcAction::Percent),
                    "^" => self.handle_calc_action(CalcAction::Power),
                    "(" => self.handle_calc_action(CalcAction::LeftParen),
                    ")" => self.handle_calc_action(CalcAction::RightParen),
                    "!" => self.handle_calc_action(CalcAction::Factorial),
                    _ => {}
                }
            }
            Key::Named(named) => {
                use cosmic::iced::keyboard::key::Named;
                match named {
                    Named::Enter | Named::NumpadEnter => {
                        self.handle_calc_action(CalcAction::Equals);
                    }
                    Named::Backspace => {
                        self.handle_calc_action(CalcAction::Backspace);
                    }
                    Named::Delete => {
                        self.handle_calc_action(CalcAction::Clear);
                    }
                    Named::Escape => {
                        self.handle_calc_action(CalcAction::Clear);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
