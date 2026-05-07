use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: Theme,
    pub angle_mode: AngleUnit,
    pub decimal_separator: DecimalSeparator,
    pub thousands_separator: bool,
    pub precision: usize,
    pub show_history: bool,
    pub history_max: usize,
    pub font_size: FontSize,
    pub show_bit_panel: bool,
    pub round_results: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    System,
    Light,
    Dark,
}

impl Theme {
    pub fn label(&self) -> &'static str {
        match self {
            Theme::System => "Follow System",
            Theme::Light => "Light",
            Theme::Dark => "Dark",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![Self::System, Self::Light, Self::Dark]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AngleUnit {
    Degrees,
    Radians,
    Gradians,
}

impl AngleUnit {
    pub fn label(&self) -> &'static str {
        match self {
            AngleUnit::Degrees => "Degrees",
            AngleUnit::Radians => "Radians",
            AngleUnit::Gradians => "Gradians",
        }
    }
    pub fn all() -> Vec<Self> {
        vec![Self::Degrees, Self::Radians, Self::Gradians]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DecimalSeparator {
    Dot,
    Comma,
}

impl DecimalSeparator {
    pub fn label(&self) -> &'static str {
        match self {
            DecimalSeparator::Dot => "Period (.)",
            DecimalSeparator::Comma => "Comma (,)",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FontSize {
    Small,
    Medium,
    Large,
}

impl FontSize {
    pub fn label(&self) -> &'static str {
        match self {
            FontSize::Small => "Small",
            FontSize::Medium => "Medium",
            FontSize::Large => "Large",
        }
    }
    pub fn size(&self) -> f32 {
        match self {
            FontSize::Small => 14.0,
            FontSize::Medium => 18.0,
            FontSize::Large => 22.0,
        }
    }
    pub fn all() -> Vec<Self> {
        vec![Self::Small, Self::Medium, Self::Large]
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: Theme::System,
            angle_mode: AngleUnit::Degrees,
            decimal_separator: DecimalSeparator::Dot,
            thousands_separator: true,
            precision: 10,
            show_history: true,
            history_max: 100,
            font_size: FontSize::Medium,
            show_bit_panel: true,
            round_results: false,
        }
    }
}

impl AppSettings {
    pub fn load() -> Self {
        // Would load from config file in production
        Self::default()
    }

    pub fn save(&self) {
        // Would save to config file in production
    }
}
