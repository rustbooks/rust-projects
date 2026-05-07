#[derive(Debug, Clone, PartialEq, Default)]
pub enum ConverterCategory {
    #[default]
    Length,
    Area,
    Volume,
    Mass,
    Temperature,
    Speed,
    Time,
    DataStorage,
    Energy,
    Pressure,
    Angle,
    Fuel,
}

impl ConverterCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Length => "Length",
            Self::Area => "Area",
            Self::Volume => "Volume",
            Self::Mass => "Mass/Weight",
            Self::Temperature => "Temperature",
            Self::Speed => "Speed",
            Self::Time => "Time",
            Self::DataStorage => "Data Storage",
            Self::Energy => "Energy",
            Self::Pressure => "Pressure",
            Self::Angle => "Angle",
            Self::Fuel => "Fuel Economy",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::Length, Self::Area, Self::Volume, Self::Mass,
            Self::Temperature, Self::Speed, Self::Time, Self::DataStorage,
            Self::Energy, Self::Pressure, Self::Angle, Self::Fuel,
        ]
    }

    pub fn units(&self) -> Vec<&'static str> {
        match self {
            Self::Length => vec!["Millimeter", "Centimeter", "Meter", "Kilometer", "Inch", "Foot", "Yard", "Mile", "Nautical Mile", "Light Year"],
            Self::Area => vec!["mm²", "cm²", "m²", "km²", "in²", "ft²", "yd²", "Acre", "Hectare", "mile²"],
            Self::Volume => vec!["Milliliter", "Centiliter", "Liter", "m³", "Teaspoon", "Tablespoon", "Cup", "Pint", "Quart", "Gallon (US)", "Gallon (UK)", "fl oz"],
            Self::Mass => vec!["Milligram", "Gram", "Kilogram", "Tonne", "Ounce", "Pound", "Stone", "US Ton", "UK Ton"],
            Self::Temperature => vec!["Celsius", "Fahrenheit", "Kelvin", "Rankine"],
            Self::Speed => vec!["m/s", "km/h", "mph", "knot", "ft/s", "Mach"],
            Self::Time => vec!["Nanosecond", "Microsecond", "Millisecond", "Second", "Minute", "Hour", "Day", "Week", "Month", "Year", "Decade", "Century"],
            Self::DataStorage => vec!["Bit", "Byte", "Kilobyte", "Megabyte", "Gigabyte", "Terabyte", "Petabyte", "Kibibyte", "Mebibyte", "Gibibyte"],
            Self::Energy => vec!["Joule", "Kilojoule", "Calorie", "Kilocalorie", "kWh", "BTU", "eV", "Foot-pound"],
            Self::Pressure => vec!["Pascal", "Kilopascal", "Bar", "Millibar", "PSI", "atm", "mmHg", "Torr"],
            Self::Angle => vec!["Degree", "Radian", "Gradian", "Arcminute", "Arcsecond"],
            Self::Fuel => vec!["L/100km", "km/L", "mpg (US)", "mpg (UK)"],
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Converter {
    pub category: ConverterCategory,
    pub from_unit_idx: usize,
    pub to_unit_idx: usize,
    pub input_value: String,
    pub output_value: String,
}

impl Converter {
    pub fn new() -> Self {
        Self {
            category: ConverterCategory::Length,
            from_unit_idx: 2, // Meter
            to_unit_idx: 5,   // Foot
            input_value: String::from("1"),
            output_value: String::new(),
        }
    }

    pub fn convert(&mut self) {
        let val: f64 = match self.input_value.parse() {
            Ok(v) => v,
            Err(_) => {
                self.output_value = "Invalid input".to_string();
                return;
            }
        };

        let units = self.category.units();
        if self.from_unit_idx >= units.len() || self.to_unit_idx >= units.len() {
            self.output_value = "Invalid unit".to_string();
            return;
        }

        let from = units[self.from_unit_idx];
        let to = units[self.to_unit_idx];

        let result = match &self.category {
            ConverterCategory::Temperature => convert_temperature(val, from, to),
            _ => {
                let to_base = to_base_unit(val, &self.category, from);
                from_base_unit(to_base, &self.category, to)
            }
        };

        self.output_value = format_result(result);
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.from_unit_idx, &mut self.to_unit_idx);
        self.input_value = self.output_value.clone();
        self.convert();
    }
}

fn convert_temperature(val: f64, from: &str, to: &str) -> f64 {
    // Convert to Celsius first
    let celsius = match from {
        "Celsius" => val,
        "Fahrenheit" => (val - 32.0) * 5.0 / 9.0,
        "Kelvin" => val - 273.15,
        "Rankine" => (val - 491.67) * 5.0 / 9.0,
        _ => val,
    };
    // Convert from Celsius to target
    match to {
        "Celsius" => celsius,
        "Fahrenheit" => celsius * 9.0 / 5.0 + 32.0,
        "Kelvin" => celsius + 273.15,
        "Rankine" => (celsius + 273.15) * 9.0 / 5.0,
        _ => celsius,
    }
}

fn to_base_unit(val: f64, cat: &ConverterCategory, unit: &str) -> f64 {
    val * get_factor(cat, unit)
}

fn from_base_unit(val: f64, cat: &ConverterCategory, unit: &str) -> f64 {
    val / get_factor(cat, unit)
}

fn get_factor(cat: &ConverterCategory, unit: &str) -> f64 {
    match cat {
        ConverterCategory::Length => match unit {
            "Millimeter" => 0.001,
            "Centimeter" => 0.01,
            "Meter" => 1.0,
            "Kilometer" => 1000.0,
            "Inch" => 0.0254,
            "Foot" => 0.3048,
            "Yard" => 0.9144,
            "Mile" => 1609.344,
            "Nautical Mile" => 1852.0,
            "Light Year" => 9.461e15,
            _ => 1.0,
        },
        ConverterCategory::Area => match unit {
            "mm²" => 1e-6,
            "cm²" => 1e-4,
            "m²" => 1.0,
            "km²" => 1e6,
            "in²" => 6.4516e-4,
            "ft²" => 0.092903,
            "yd²" => 0.836127,
            "Acre" => 4046.86,
            "Hectare" => 10000.0,
            "mile²" => 2.59e6,
            _ => 1.0,
        },
        ConverterCategory::Volume => match unit {
            "Milliliter" => 0.001,
            "Centiliter" => 0.01,
            "Liter" => 1.0,
            "m³" => 1000.0,
            "Teaspoon" => 0.00492892,
            "Tablespoon" => 0.0147868,
            "Cup" => 0.236588,
            "Pint" => 0.473176,
            "Quart" => 0.946353,
            "Gallon (US)" => 3.78541,
            "Gallon (UK)" => 4.54609,
            "fl oz" => 0.0295735,
            _ => 1.0,
        },
        ConverterCategory::Mass => match unit {
            "Milligram" => 1e-6,
            "Gram" => 0.001,
            "Kilogram" => 1.0,
            "Tonne" => 1000.0,
            "Ounce" => 0.0283495,
            "Pound" => 0.453592,
            "Stone" => 6.35029,
            "US Ton" => 907.185,
            "UK Ton" => 1016.05,
            _ => 1.0,
        },
        ConverterCategory::Speed => match unit {
            "m/s" => 1.0,
            "km/h" => 1.0 / 3.6,
            "mph" => 0.44704,
            "knot" => 0.514444,
            "ft/s" => 0.3048,
            "Mach" => 343.0,
            _ => 1.0,
        },
        ConverterCategory::Time => match unit {
            "Nanosecond" => 1e-9,
            "Microsecond" => 1e-6,
            "Millisecond" => 0.001,
            "Second" => 1.0,
            "Minute" => 60.0,
            "Hour" => 3600.0,
            "Day" => 86400.0,
            "Week" => 604800.0,
            "Month" => 2629800.0,
            "Year" => 31557600.0,
            "Decade" => 315576000.0,
            "Century" => 3155760000.0,
            _ => 1.0,
        },
        ConverterCategory::DataStorage => match unit {
            "Bit" => 1.0 / 8.0,
            "Byte" => 1.0,
            "Kilobyte" => 1e3,
            "Megabyte" => 1e6,
            "Gigabyte" => 1e9,
            "Terabyte" => 1e12,
            "Petabyte" => 1e15,
            "Kibibyte" => 1024.0,
            "Mebibyte" => 1048576.0,
            "Gibibyte" => 1073741824.0,
            _ => 1.0,
        },
        ConverterCategory::Energy => match unit {
            "Joule" => 1.0,
            "Kilojoule" => 1000.0,
            "Calorie" => 4.184,
            "Kilocalorie" => 4184.0,
            "kWh" => 3.6e6,
            "BTU" => 1055.06,
            "eV" => 1.602e-19,
            "Foot-pound" => 1.35582,
            _ => 1.0,
        },
        ConverterCategory::Pressure => match unit {
            "Pascal" => 1.0,
            "Kilopascal" => 1000.0,
            "Bar" => 100000.0,
            "Millibar" => 100.0,
            "PSI" => 6894.76,
            "atm" => 101325.0,
            "mmHg" => 133.322,
            "Torr" => 133.322,
            _ => 1.0,
        },
        ConverterCategory::Angle => match unit {
            "Degree" => 1.0,
            "Radian" => 180.0 / std::f64::consts::PI,
            "Gradian" => 0.9,
            "Arcminute" => 1.0 / 60.0,
            "Arcsecond" => 1.0 / 3600.0,
            _ => 1.0,
        },
        ConverterCategory::Fuel => match unit {
            "L/100km" => 1.0,
            "km/L" => 100.0,       // inverse
            "mpg (US)" => 235.214,
            "mpg (UK)" => 282.481,
            _ => 1.0,
        },
        _ => 1.0,
    }
}

fn format_result(val: f64) -> String {
    if val.is_nan() { return "NaN".to_string(); }
    if val.is_infinite() { return "∞".to_string(); }
    let abs = val.abs();
    if abs == 0.0 { return "0".to_string(); }
    if abs >= 1e15 || (abs < 1e-10 && abs != 0.0) {
        return format!("{:.6e}", val);
    }
    let s = format!("{:.10}", val);
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}
