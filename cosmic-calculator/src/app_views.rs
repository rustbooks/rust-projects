use cosmic::widget::*;
use cosmic::{Apply, Element};
use cosmic::iced::{Alignment, Length, Padding};
use cosmic::iced::widget::horizontal_rule;

use crate::app::{CosmicCalculator, Message, CalcAction, ProgAction, Page};
use crate::calculator::engine::{AngleMode, format_number};
use crate::calculator::display::format_display;
use crate::converter::ConverterCategory;
use crate::programmer::{Base, BitWidth, BitwiseOp};
use crate::settings::{Theme, AngleUnit, FontSize};

impl CosmicCalculator {
    // ── Calculator View ──────────────────────────────────────────────────────

    pub fn view_calculator(&self) -> Element<Message> {
        let angle_label = match self.calc.angle_mode {
            AngleMode::Degrees => "DEG",
            AngleMode::Radians => "RAD",
            AngleMode::Gradians => "GRA",
        };

        // Display area
        let display_expr = text(&self.calc.expression)
            .size(16.0)
            .apply(container)
            .width(Length::Fill)
            .align_x(cosmic::iced::alignment::Horizontal::Right)
            .padding(Padding::new(4.0));

        let result_str = if let Some(err) = &self.calc.error {
            format!("Error: {}", err)
        } else {
            let raw = &self.calc.result;
            if self.settings.thousands_separator {
                format_display(raw)
            } else {
                raw.clone()
            }
        };

        let display_result = text(result_str)
            .size(self.settings.font_size.size() * 2.0)
            .apply(container)
            .width(Length::Fill)
            .align_x(cosmic::iced::alignment::Horizontal::Right)
            .padding(Padding::new(8.0));

        let mem_indicator = if self.calc.memory != 0.0 {
            text(format!("M: {}", format_number(self.calc.memory))).size(12.0)
        } else {
            text("").size(12.0)
        };

        let angle_btn = button::text(angle_label)
            .on_press(Message::CalcButton(CalcAction::AngleMode));

        let hist_btn = button::icon(icon::from_name("document-open-recent-symbolic"))
            .on_press(Message::HistoryToggle);

        let display_bar = row()
            .push(mem_indicator)
            .push(horizontal_space())
            .push(angle_btn)
            .push(hist_btn)
            .align_items(Alignment::Center)
            .padding(4);

        let display_section = column()
            .push(display_bar)
            .push(display_expr)
            .push(display_result)
            .spacing(2)
            .apply(container)
            .width(Length::Fill)
            .style(cosmic::theme::Container::Card);

        // Memory row
        let mem_row = row()
            .push(self.calc_btn_sm("MC", CalcAction::MemClear))
            .push(self.calc_btn_sm("MR", CalcAction::MemRecall))
            .push(self.calc_btn_sm("M+", CalcAction::MemAdd))
            .push(self.calc_btn_sm("M-", CalcAction::MemSub))
            .push(self.calc_btn_sm("MS", CalcAction::MemStore))
            .spacing(4)
            .width(Length::Fill);

        // Scientific toggle
        let sci_btn = button::text(if self.show_scientific { "Basic" } else { "Scientific" })
            .on_press(Message::NavPage(Page::Calculator)); // placeholder toggle

        // Scientific row 1: inv trig + log
        let sci_row1 = row()
            .push(self.calc_btn_fn("sin", CalcAction::Sin))
            .push(self.calc_btn_fn("cos", CalcAction::Cos))
            .push(self.calc_btn_fn("tan", CalcAction::Tan))
            .push(self.calc_btn_fn("log", CalcAction::Log))
            .push(self.calc_btn_fn("ln", CalcAction::Ln))
            .spacing(4)
            .width(Length::Fill);

        let sci_row2 = row()
            .push(self.calc_btn_fn("asin", CalcAction::Asin))
            .push(self.calc_btn_fn("acos", CalcAction::Acos))
            .push(self.calc_btn_fn("atan", CalcAction::Atan))
            .push(self.calc_btn_fn("xⁿ", CalcAction::Power))
            .push(self.calc_btn_fn("√", CalcAction::Sqrt))
            .spacing(4)
            .width(Length::Fill);

        let sci_row3 = row()
            .push(self.calc_btn_fn("π", CalcAction::Pi))
            .push(self.calc_btn_fn("e", CalcAction::E))
            .push(self.calc_btn_fn("x²", CalcAction::Square))
            .push(self.calc_btn_fn("x³", CalcAction::Cube))
            .push(self.calc_btn_fn("1/x", CalcAction::Inverse))
            .spacing(4)
            .width(Length::Fill);

        let sci_row4 = row()
            .push(self.calc_btn_fn("(", CalcAction::LeftParen))
            .push(self.calc_btn_fn(")", CalcAction::RightParen))
            .push(self.calc_btn_fn("|x|", CalcAction::Abs))
            .push(self.calc_btn_fn("x!", CalcAction::Factorial))
            .push(self.calc_btn_fn("EXP", CalcAction::Exp))
            .spacing(4)
            .width(Length::Fill);

        // Main numpad
        let numpad = column()
            .push(
                row()
                    .push(self.calc_btn_accent("C", CalcAction::Clear))
                    .push(self.calc_btn_sm("CE", CalcAction::ClearEntry))
                    .push(self.calc_btn_sm("⌫", CalcAction::Backspace))
                    .push(self.calc_btn_op("÷", CalcAction::Op('÷')))
                    .spacing(4).width(Length::Fill)
            )
            .push(
                row()
                    .push(self.calc_btn_num("7", CalcAction::Digit('7')))
                    .push(self.calc_btn_num("8", CalcAction::Digit('8')))
                    .push(self.calc_btn_num("9", CalcAction::Digit('9')))
                    .push(self.calc_btn_op("×", CalcAction::Op('×')))
                    .spacing(4).width(Length::Fill)
            )
            .push(
                row()
                    .push(self.calc_btn_num("4", CalcAction::Digit('4')))
                    .push(self.calc_btn_num("5", CalcAction::Digit('5')))
                    .push(self.calc_btn_num("6", CalcAction::Digit('6')))
                    .push(self.calc_btn_op("-", CalcAction::Op('-')))
                    .spacing(4).width(Length::Fill)
            )
            .push(
                row()
                    .push(self.calc_btn_num("1", CalcAction::Digit('1')))
                    .push(self.calc_btn_num("2", CalcAction::Digit('2')))
                    .push(self.calc_btn_num("3", CalcAction::Digit('3')))
                    .push(self.calc_btn_op("+", CalcAction::Op('+')))
                    .spacing(4).width(Length::Fill)
            )
            .push(
                row()
                    .push(self.calc_btn_num("+/-", CalcAction::Negate))
                    .push(self.calc_btn_num("0", CalcAction::Digit('0')))
                    .push(self.calc_btn_num(".", CalcAction::Dot))
                    .push(self.calc_btn_equals("=", CalcAction::Equals))
                    .spacing(4).width(Length::Fill)
            )
            .spacing(4);

        let mut main_col = column()
            .push(display_section)
            .push(vertical_space().height(4))
            .push(mem_row)
            .push(vertical_space().height(4))
            .push(sci_row1)
            .push(sci_row2)
            .push(sci_row3)
            .push(sci_row4)
            .push(vertical_space().height(4))
            .push(numpad)
            .spacing(4)
            .padding(12)
            .width(Length::Fill);

        if self.show_history && !self.history.is_empty() {
            let history_panel = self.view_history_panel();
            row()
                .push(main_col.apply(container).width(Length::FillPortion(3)))
                .push(history_panel.apply(container).width(Length::FillPortion(2)))
                .spacing(8)
                .into()
        } else {
            main_col.into()
        }
    }

    fn view_history_panel(&self) -> Element<Message> {
        let header = row()
            .push(text("History").size(16.0))
            .push(horizontal_space())
            .push(
                button::text("Clear")
                    .on_press(Message::HistoryClear)
            )
            .align_items(Alignment::Center);

        let mut list = column().spacing(4);

        for entry in self.history.entries.iter().rev().take(50) {
            let item = column()
                .push(
                    text(&entry.expression)
                        .size(13.0)
                        .apply(container)
                        .width(Length::Fill)
                )
                .push(
                    text(&entry.result)
                        .size(18.0)
                        .apply(container)
                        .width(Length::Fill)
                        .align_x(cosmic::iced::alignment::Horizontal::Right)
                )
                .apply(button::custom)
                .on_press(Message::HistoryUse(entry.expression.clone()))
                .width(Length::Fill);

            list = list.push(item);
            list = list.push(horizontal_rule(1));
        }

        let scrollable_list = scrollable(list).height(Length::Fill);

        column()
            .push(header)
            .push(horizontal_rule(2))
            .push(scrollable_list)
            .spacing(8)
            .padding(12)
            .apply(container)
            .style(cosmic::theme::Container::Card)
            .height(Length::Fill)
            .into()
    }

    // ── Converter View ───────────────────────────────────────────────────────

    pub fn view_converter(&self) -> Element<Message> {
        let units = self.converter.category.units();
        let categories = ConverterCategory::all();

        let cat_options: Vec<_> = categories.iter().map(|c| c.label().to_string()).collect();
        let cur_cat_idx = categories.iter().position(|c| c == &self.converter.category).unwrap_or(0);

        let from_options: Vec<_> = units.iter().map(|u| u.to_string()).collect();
        let to_options: Vec<_> = units.iter().map(|u| u.to_string()).collect();

        let category_picker = column()
            .push(text("Category").size(14.0))
            .push(
                dropdown(&cat_options, Some(cur_cat_idx), Message::ConvCategoryChanged)
                    .width(Length::Fill)
            )
            .spacing(4);

        let input_field = column()
            .push(text("From").size(14.0))
            .push(
                dropdown(&from_options, Some(self.converter.from_unit_idx), Message::ConvFromUnitChanged)
                    .width(Length::Fill)
            )
            .push(
                text_input("Enter value...", &self.converter.input_value)
                    .on_input(Message::ConvInputChanged)
                    .size(24.0)
                    .width(Length::Fill)
            )
            .spacing(4);

        let swap_btn = button::icon(icon::from_name("object-flip-vertical-symbolic"))
            .on_press(Message::ConvSwap)
            .apply(container)
            .center_x();

        let output_field = column()
            .push(text("To").size(14.0))
            .push(
                dropdown(&to_options, Some(self.converter.to_unit_idx), Message::ConvToUnitChanged)
                    .width(Length::Fill)
            )
            .push(
                text(&self.converter.output_value)
                    .size(28.0)
                    .apply(container)
                    .width(Length::Fill)
                    .align_x(cosmic::iced::alignment::Horizontal::Right)
                    .padding(Padding::new(8.0))
                    .style(cosmic::theme::Container::Card)
            )
            .spacing(4);

        // Quick conversion table for common units
        let quick_table = self.view_conversion_table();

        column()
            .push(
                text("Unit Converter")
                    .size(24.0)
                    .apply(container)
                    .padding(Padding::from([0, 0, 8, 0]))
            )
            .push(category_picker)
            .push(vertical_space().height(8))
            .push(input_field)
            .push(swap_btn)
            .push(output_field)
            .push(vertical_space().height(16))
            .push(horizontal_rule(1))
            .push(vertical_space().height(8))
            .push(text("Quick Reference").size(16.0))
            .push(quick_table)
            .spacing(4)
            .padding(16)
            .apply(scrollable)
            .into()
    }

    fn view_conversion_table(&self) -> Element<Message> {
        let val: f64 = self.converter.input_value.parse().unwrap_or(1.0);
        let units = self.converter.category.units();

        let mut col = column().spacing(2);
        for (i, unit) in units.iter().enumerate().take(10) {
            let mut tmp = self.converter.clone();
            tmp.to_unit_idx = i;
            tmp.convert();

            let row_item = row()
                .push(text(*unit).size(14.0).width(Length::FillPortion(2)))
                .push(text(&tmp.output_value).size(14.0).width(Length::FillPortion(2)))
                .padding(Padding::from([2, 4]));

            col = col.push(row_item);
            if i < units.len() - 1 {
                col = col.push(horizontal_rule(1));
            }
        }

        col.apply(container)
            .style(cosmic::theme::Container::Card)
            .padding(8)
            .width(Length::Fill)
            .into()
    }

    // ── Programmer View ──────────────────────────────────────────────────────

    pub fn view_programmer(&self) -> Element<Message> {
        // Base display header
        let hex_val = self.prog.get_value_in_base(&Base::Hex);
        let dec_val = self.prog.get_value_in_base(&Base::Dec);
        let oct_val = self.prog.get_value_in_base(&Base::Oct);
        let bin_val = self.prog.get_value_in_base(&Base::Bin);

        let make_base_row = |label: &str, val: &str, is_active: bool, base: Base| {
            let indicator = if is_active {
                text("▶").size(12.0)
            } else {
                text(" ").size(12.0)
            };
            row()
                .push(text(label).size(14.0).width(Length::Fixed(40.0)))
                .push(indicator)
                .push(text(val).size(18.0).width(Length::Fill))
                .align_items(Alignment::Center)
                .padding(Padding::from([4, 8]))
        };

        let base_display = column()
            .push(make_base_row("HEX", &hex_val, self.prog.base == Base::Hex, Base::Hex))
            .push(horizontal_rule(1))
            .push(make_base_row("DEC", &dec_val, self.prog.base == Base::Dec, Base::Dec))
            .push(horizontal_rule(1))
            .push(make_base_row("OCT", &oct_val, self.prog.base == Base::Oct, Base::Oct))
            .push(horizontal_rule(1))
            .push(make_base_row("BIN", &bin_val, self.prog.base == Base::Bin, Base::Bin))
            .apply(container)
            .style(cosmic::theme::Container::Card)
            .width(Length::Fill);

        // Bit width selector
        let bit_widths = [BitWidth::Bit8, BitWidth::Bit16, BitWidth::Bit32, BitWidth::Bit64];
        let bw_row = row()
            .push(text("Bit Width:").size(14.0))
            .push(horizontal_space())
            .push({
                let mut r = row().spacing(4);
                for bw in &bit_widths {
                    let is_active = &self.prog.bit_width == bw;
                    let bw_clone = bw.clone();
                    let btn = if is_active {
                        button::suggested(bw.label())
                            .on_press(Message::ProgBitWidthChanged(bw_clone))
                    } else {
                        button::standard(bw.label())
                            .on_press(Message::ProgBitWidthChanged(bw_clone))
                    };
                    r = r.push(btn);
                }
                r
            })
            .align_items(Alignment::Center);

        // Expression input display
        let expr_display = text(if self.prog.expression.is_empty() { "0" } else { &self.prog.expression })
            .size(22.0)
            .apply(container)
            .width(Length::Fill)
            .align_x(cosmic::iced::alignment::Horizontal::Right)
            .padding(Padding::new(8.0));

        // Base selector row
        let bases = [Base::Hex, Base::Dec, Base::Oct, Base::Bin];
        let base_row = row()
            .push({
                let mut r = row().spacing(4);
                for base in &bases {
                    let is_active = &self.prog.base == base;
                    let base_clone = base.clone();
                    let btn = if is_active {
                        button::suggested(base.label())
                            .on_press(Message::ProgBaseChanged(base_clone))
                    } else {
                        button::standard(base.label())
                            .on_press(Message::ProgBaseChanged(base_clone))
                    };
                    r = r.push(btn);
                }
                r
            });

        // Bit panel
        let bit_panel = if self.settings.show_bit_panel {
            let bits = self.prog.get_bits();
            let num_bits = bits.len();
            let mut bit_col = column().spacing(2);

            // Show bits in groups of 8
            for chunk_start in (0..num_bits).step_by(8) {
                let chunk_end = (chunk_start + 8).min(num_bits);
                let mut bit_row = row().spacing(2);

                // Bit index label
                let idx_label = text(format!("{:2}", num_bits - chunk_start - 1)).size(10.0)
                    .width(Length::Fixed(20.0));
                bit_row = bit_row.push(idx_label);

                for i in chunk_start..chunk_end {
                    let is_set = bits[i];
                    let bit_idx = i;
                    let btn = button::text(if is_set { "1" } else { "0" })
                        .on_press(Message::ProgToggleBit(bit_idx))
                        .apply(container)
                        .width(Length::Fixed(30.0));
                    bit_row = bit_row.push(btn);
                }
                bit_col = bit_col.push(bit_row);
            }

            bit_col.apply(container)
                .style(cosmic::theme::Container::Card)
                .padding(8)
                .width(Length::Fill)
                .into()
        } else {
            vertical_space().height(0).into()
        };

        // Bitwise ops row
        let bitwise_row = row()
            .push(self.prog_btn_fn("AND", ProgAction::Bitwise(BitwiseOp::And)))
            .push(self.prog_btn_fn("OR", ProgAction::Bitwise(BitwiseOp::Or)))
            .push(self.prog_btn_fn("XOR", ProgAction::Bitwise(BitwiseOp::Xor)))
            .push(self.prog_btn_fn("NOT", ProgAction::Bitwise(BitwiseOp::Not)))
            .push(self.prog_btn_fn("<<", ProgAction::Bitwise(BitwiseOp::LShift)))
            .push(self.prog_btn_fn(">>", ProgAction::Bitwise(BitwiseOp::RShift)))
            .spacing(4).width(Length::Fill);

        // Hex numpad (A-F + standard)
        let hex_row = row()
            .push(self.prog_btn_hex('A'))
            .push(self.prog_btn_hex('B'))
            .push(self.prog_btn_hex('C'))
            .push(self.prog_btn_hex('D'))
            .push(self.prog_btn_hex('E'))
            .push(self.prog_btn_hex('F'))
            .spacing(4).width(Length::Fill);

        let numpad = column()
            .push(
                row()
                    .push(self.prog_btn("C", ProgAction::Clear))
                    .push(self.prog_btn("(", ProgAction::LeftParen))
                    .push(self.prog_btn(")", ProgAction::RightParen))
                    .push(self.prog_btn("⌫", ProgAction::Backspace))
                    .push(self.prog_btn("÷", ProgAction::Op('/')))
                    .spacing(4).width(Length::Fill)
            )
            .push(
                row()
                    .push(self.prog_btn_d('7'))
                    .push(self.prog_btn_d('8'))
                    .push(self.prog_btn_d('9'))
                    .push(self.prog_btn("×", ProgAction::Op('*')))
                    .spacing(4).width(Length::Fill)
            )
            .push(
                row()
                    .push(self.prog_btn_d('4'))
                    .push(self.prog_btn_d('5'))
                    .push(self.prog_btn_d('6'))
                    .push(self.prog_btn("-", ProgAction::Op('-')))
                    .spacing(4).width(Length::Fill)
            )
            .push(
                row()
                    .push(self.prog_btn_d('1'))
                    .push(self.prog_btn_d('2'))
                    .push(self.prog_btn_d('3'))
                    .push(self.prog_btn("+", ProgAction::Op('+')))
                    .spacing(4).width(Length::Fill)
            )
            .push(
                row()
                    .push(self.prog_btn("+/-", ProgAction::Negate))
                    .push(self.prog_btn_d('0'))
                    .push(self.prog_btn("%", ProgAction::Mod))
                    .push(self.calc_btn_equals("=", CalcAction::Equals).map(|m| {
                        if let Message::CalcButton(_) = &m {
                            Message::ProgButton(ProgAction::Equals)
                        } else { m }
                    }))
                    .spacing(4).width(Length::Fill)
            )
            .spacing(4);

        column()
            .push(bw_row)
            .push(base_display)
            .push(expr_display)
            .push(vertical_space().height(4))
            .push(base_row)
            .push(vertical_space().height(4))
            .push(bitwise_row)
            .push(hex_row)
            .push(vertical_space().height(4))
            .push(numpad)
            .push(vertical_space().height(8))
            .push(bit_panel)
            .spacing(4)
            .padding(12)
            .apply(scrollable)
            .into()
    }

    // ── Settings View ────────────────────────────────────────────────────────

    pub fn view_settings(&self) -> Element<Message> {
        let section = |title: &str| {
            text(title).size(18.0).apply(container).padding(Padding::from([8, 0, 4, 0]))
        };

        // Theme
        let themes: Vec<String> = [Theme::System, Theme::Light, Theme::Dark]
            .iter().map(|t| t.label().to_string()).collect();
        let theme_idx = match &self.settings.theme {
            Theme::System => 0, Theme::Light => 1, Theme::Dark => 2,
        };

        // Angle unit
        let angles: Vec<String> = AngleUnit::all().iter().map(|a| a.label().to_string()).collect();
        let angle_idx = AngleUnit::all().iter().position(|a| a == &self.settings.angle_mode).unwrap_or(0);

        // Font size
        let fonts: Vec<String> = FontSize::all().iter().map(|f| f.label().to_string()).collect();
        let font_idx = FontSize::all().iter().position(|f| f == &self.settings.font_size).unwrap_or(1);

        let content = column()
            .push(text("Settings").size(28.0))
            .push(vertical_space().height(16))

            // Appearance
            .push(section("Appearance"))
            .push(horizontal_rule(1))
            .push(
                settings::item(
                    "Color Theme",
                    dropdown(&themes, Some(theme_idx), Message::SettingTheme)
                        .width(Length::Fixed(200.0))
                )
            )
            .push(
                settings::item(
                    "Font Size",
                    dropdown(&fonts, Some(font_idx), Message::SettingFontSize)
                        .width(Length::Fixed(200.0))
                )
            )

            // Calculator
            .push(vertical_space().height(8))
            .push(section("Calculator"))
            .push(horizontal_rule(1))
            .push(
                settings::item(
                    "Angle Unit",
                    dropdown(&angles, Some(angle_idx), Message::SettingAngle)
                        .width(Length::Fixed(200.0))
                )
            )
            .push(
                settings::item_row(vec![
                    text("Thousands Separator").into(),
                    horizontal_space().into(),
                    toggler(None, self.settings.thousands_separator, Message::SettingThousandsSep).into(),
                ])
            )
            .push(
                settings::item_row(vec![
                    text("Round Results").into(),
                    horizontal_space().into(),
                    toggler(None, self.settings.round_results, Message::SettingRound).into(),
                ])
            )

            // History
            .push(vertical_space().height(8))
            .push(section("History"))
            .push(horizontal_rule(1))
            .push(
                settings::item_row(vec![
                    text("Show History Panel").into(),
                    horizontal_space().into(),
                    toggler(None, self.settings.show_history, Message::SettingShowHistory).into(),
                ])
            )

            // Programmer
            .push(vertical_space().height(8))
            .push(section("Programmer"))
            .push(horizontal_rule(1))
            .push(
                settings::item_row(vec![
                    text("Show Bit Panel").into(),
                    horizontal_space().into(),
                    toggler(None, self.settings.show_bit_panel, Message::SettingShowBitPanel).into(),
                ])
            )

            // About
            .push(vertical_space().height(16))
            .push(section("About"))
            .push(horizontal_rule(1))
            .push(
                column()
                    .push(text("COSMIC Calculator").size(16.0))
                    .push(text("Version 0.1.0").size(14.0))
                    .push(text("A full-featured calculator for the COSMIC desktop environment").size(13.0))
                    .push(text("Built with Rust + libcosmic").size(13.0))
                    .spacing(4)
                    .padding(8)
            )

            .spacing(4)
            .padding(16)
            .width(Length::Fill);

        scrollable(content).into()
    }

    // ── Button Helpers ───────────────────────────────────────────────────────

    fn calc_btn_num<'a>(&self, label: &'a str, action: CalcAction) -> Element<'a, Message> {
        button::standard(label)
            .on_press(Message::CalcButton(action))
            .width(Length::Fill)
            .into()
    }

    fn calc_btn_op<'a>(&self, label: &'a str, action: CalcAction) -> Element<'a, Message> {
        button::suggested(label)
            .on_press(Message::CalcButton(action))
            .width(Length::Fill)
            .into()
    }

    fn calc_btn_accent<'a>(&self, label: &'a str, action: CalcAction) -> Element<'a, Message> {
        button::destructive(label)
            .on_press(Message::CalcButton(action))
            .width(Length::Fill)
            .into()
    }

    fn calc_btn_equals<'a>(&self, label: &'a str, action: CalcAction) -> Element<'a, Message> {
        button::suggested(label)
            .on_press(Message::CalcButton(action))
            .width(Length::Fill)
            .into()
    }

    fn calc_btn_fn<'a>(&self, label: &'a str, action: CalcAction) -> Element<'a, Message> {
        button::standard(label)
            .on_press(Message::CalcButton(action))
            .width(Length::Fill)
            .into()
    }

    fn calc_btn_sm<'a>(&self, label: &'a str, action: CalcAction) -> Element<'a, Message> {
        button::standard(label)
            .on_press(Message::CalcButton(action))
            .width(Length::Fill)
            .into()
    }

    fn prog_btn<'a>(&self, label: &'a str, action: ProgAction) -> Element<'a, Message> {
        button::standard(label)
            .on_press(Message::ProgButton(action))
            .width(Length::Fill)
            .into()
    }

    fn prog_btn_fn<'a>(&self, label: &'a str, action: ProgAction) -> Element<'a, Message> {
        button::standard(label)
            .on_press(Message::ProgButton(action))
            .width(Length::Fill)
            .into()
    }

    fn prog_btn_d<'a>(&self, digit: char) -> Element<'a, Message> {
        let enabled = match self.prog.base {
            Base::Bin => digit <= '1',
            Base::Oct => digit <= '7',
            Base::Dec => digit <= '9',
            Base::Hex => true,
        };

        if enabled {
            button::standard(digit.to_string())
                .on_press(Message::ProgButton(ProgAction::Digit(digit)))
                .width(Length::Fill)
                .into()
        } else {
            button::standard(digit.to_string())
                .width(Length::Fill)
                .into()
        }
    }

    fn prog_btn_hex<'a>(&self, digit: char) -> Element<'a, Message> {
        let enabled = self.prog.base == Base::Hex;
        if enabled {
            button::suggested(digit.to_string())
                .on_press(Message::ProgButton(ProgAction::Digit(digit)))
                .width(Length::Fill)
                .into()
        } else {
            button::standard(digit.to_string())
                .width(Length::Fill)
                .into()
        }
    }
}
