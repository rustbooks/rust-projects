// src/converter.rs
//
// Unit conversion engine.
// All conversions go through SI base units (double conversion).

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ConversionCategory {
    pub name: &'static str,
    pub units: Vec<UnitInfo>,
}

#[derive(Debug, Clone)]
pub struct UnitInfo {
    pub name: &'static str,
    /// Multiply by this to get SI base unit
    pub to_si: f64,
}

impl UnitInfo {
    const fn new(name: &'static str, to_si: f64) -> Self {
        Self { name, to_si }
    }
}

/// Convert `value` from `from_unit` to `to_unit` within a category.
/// Returns None if units not found.
pub fn convert(value: f64, from_unit: &str, to_unit: &str, category: &str) -> Option<f64> {
    let cats = all_categories();
    let cat = cats.iter().find(|c| c.name == category)?;
    let from = cat.units.iter().find(|u| u.name == from_unit)?;
    let to   = cat.units.iter().find(|u| u.name == to_unit)?;
    // value → SI → target
    let si = value * from.to_si;
    Some(si / to.to_si)
}

pub fn all_categories() -> Vec<ConversionCategory> {
    vec![
        ConversionCategory {
            name: "Length",
            units: vec![
                UnitInfo::new("Meters",      1.0),
                UnitInfo::new("Kilometers",  1_000.0),
                UnitInfo::new("Centimeters", 0.01),
                UnitInfo::new("Millimeters", 0.001),
                UnitInfo::new("Miles",       1_609.344),
                UnitInfo::new("Yards",       0.9144),
                UnitInfo::new("Feet",        0.3048),
                UnitInfo::new("Inches",      0.0254),
                UnitInfo::new("Nautical Mi", 1_852.0),
                UnitInfo::new("Light Years", 9.461e15),
            ],
        },
        ConversionCategory {
            name: "Weight",
            units: vec![
                UnitInfo::new("Kilograms",  1.0),
                UnitInfo::new("Grams",      0.001),
                UnitInfo::new("Milligrams", 1e-6),
                UnitInfo::new("Pounds",     0.453592),
                UnitInfo::new("Ounces",     0.0283495),
                UnitInfo::new("Tonnes",     1_000.0),
                UnitInfo::new("Stone",      6.35029),
                UnitInfo::new("Carats",     0.0002),
            ],
        },
        ConversionCategory {
            name: "Temperature",
            units: vec![
                // Special-cased below; to_si unused
                UnitInfo::new("Celsius",    1.0),
                UnitInfo::new("Fahrenheit", 1.0),
                UnitInfo::new("Kelvin",     1.0),
            ],
        },
        ConversionCategory {
            name: "Area",
            units: vec![
                UnitInfo::new("Sq Meters",     1.0),
                UnitInfo::new("Sq Kilometers", 1e6),
                UnitInfo::new("Sq Miles",      2.589988e6),
                UnitInfo::new("Sq Feet",       0.092903),
                UnitInfo::new("Sq Yards",      0.836127),
                UnitInfo::new("Acres",         4_046.86),
                UnitInfo::new("Hectares",      10_000.0),
            ],
        },
        ConversionCategory {
            name: "Volume",
            units: vec![
                UnitInfo::new("Liters",       1.0),
                UnitInfo::new("Milliliters",  0.001),
                UnitInfo::new("Cubic Meters", 1_000.0),
                UnitInfo::new("US Gallons",   3.78541),
                UnitInfo::new("UK Gallons",   4.54609),
                UnitInfo::new("US Cups",      0.236588),
                UnitInfo::new("Fl Oz (US)",   0.0295735),
                UnitInfo::new("Tablespoons",  0.0147868),
                UnitInfo::new("Teaspoons",    0.00492892),
            ],
        },
        ConversionCategory {
            name: "Speed",
            units: vec![
                UnitInfo::new("m/s",   1.0),
                UnitInfo::new("km/h",  1.0 / 3.6),
                UnitInfo::new("mph",   0.44704),
                UnitInfo::new("knots", 0.514444),
                UnitInfo::new("Mach",  340.29),
            ],
        },
        ConversionCategory {
            name: "Time",
            units: vec![
                UnitInfo::new("Seconds",      1.0),
                UnitInfo::new("Minutes",      60.0),
                UnitInfo::new("Hours",        3_600.0),
                UnitInfo::new("Days",         86_400.0),
                UnitInfo::new("Weeks",        604_800.0),
                UnitInfo::new("Months",       2_628_000.0),
                UnitInfo::new("Years",        31_536_000.0),
                UnitInfo::new("Milliseconds", 0.001),
                UnitInfo::new("Microseconds", 1e-6),
            ],
        },
        ConversionCategory {
            name: "Data",
            units: vec![
                UnitInfo::new("Bytes",     1.0),
                UnitInfo::new("Kilobytes", 1_024.0),
                UnitInfo::new("Megabytes", 1_048_576.0),
                UnitInfo::new("Gigabytes", 1_073_741_824.0),
                UnitInfo::new("Terabytes", 1_099_511_627_776.0),
                UnitInfo::new("Bits",      0.125),
                UnitInfo::new("Kilobits",  125.0),
                UnitInfo::new("Megabits",  125_000.0),
            ],
        },
        ConversionCategory {
            name: "Energy",
            units: vec![
                UnitInfo::new("Joules",      1.0),
                UnitInfo::new("Kilojoules",  1_000.0),
                UnitInfo::new("Calories",    4.184),
                UnitInfo::new("Kilocalories",4_184.0),
                UnitInfo::new("Watt-hours",  3_600.0),
                UnitInfo::new("kWh",         3_600_000.0),
                UnitInfo::new("BTU",         1_055.06),
                UnitInfo::new("eV",          1.602176634e-19),
            ],
        },
        ConversionCategory {
            name: "Pressure",
            units: vec![
                UnitInfo::new("Pascals",     1.0),
                UnitInfo::new("Kilopascals", 1_000.0),
                UnitInfo::new("Bar",         100_000.0),
                UnitInfo::new("PSI",         6_894.76),
                UnitInfo::new("Atm",         101_325.0),
                UnitInfo::new("mmHg",        133.322),
                UnitInfo::new("Torr",        133.322),
            ],
        },
    ]
}

/// Special temperature conversion (non-linear)
pub fn convert_temperature(value: f64, from: &str, to: &str) -> Option<f64> {
    // All → Celsius first
    let celsius = match from {
        "Celsius"    => value,
        "Fahrenheit" => (value - 32.0) * 5.0 / 9.0,
        "Kelvin"     => value - 273.15,
        _            => return None,
    };
    // Celsius → target
    let result = match to {
        "Celsius"    => celsius,
        "Fahrenheit" => celsius * 9.0 / 5.0 + 32.0,
        "Kelvin"     => celsius + 273.15,
        _            => return None,
    };
    Some(result)
}

/// Smart convert: handles temperature separately
pub fn smart_convert(value: f64, from: &str, to: &str, category: &str) -> Option<f64> {
    if category == "Temperature" {
        convert_temperature(value, from, to)
    } else {
        convert(value, from, to, category)
    }
}

/// Default unit pairs per category (from, to)
pub fn default_units(category: &str) -> (&'static str, &'static str) {
    match category {
        "Length"      => ("Meters",    "Feet"),
        "Weight"      => ("Kilograms", "Pounds"),
        "Temperature" => ("Celsius",   "Fahrenheit"),
        "Area"        => ("Sq Meters", "Sq Feet"),
        "Volume"      => ("Liters",    "US Gallons"),
        "Speed"       => ("km/h",      "mph"),
        "Time"        => ("Hours",     "Minutes"),
        "Data"        => ("Megabytes", "Gigabytes"),
        "Energy"      => ("Joules",    "Calories"),
        "Pressure"    => ("Bar",       "PSI"),
        _             => ("",          ""),
    }
}

/// Get ordered unit names for a category
pub fn unit_names(category: &str) -> Vec<&'static str> {
    all_categories()
        .into_iter()
        .find(|c| c.name == category)
        .map(|c| c.units.into_iter().map(|u| u.name).collect())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meters_to_feet() {
        let result = smart_convert(1.0, "Meters", "Feet", "Length").unwrap();
        assert!((result - 3.28084).abs() < 0.001);
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        let result = smart_convert(100.0, "Celsius", "Fahrenheit", "Temperature").unwrap();
        assert!((result - 212.0).abs() < 0.001);
    }

    #[test]
    fn test_celsius_to_kelvin() {
        let result = smart_convert(0.0, "Celsius", "Kelvin", "Temperature").unwrap();
        assert!((result - 273.15).abs() < 0.001);
    }

    #[test]
    fn test_kg_to_pounds() {
        let result = smart_convert(1.0, "Kilograms", "Pounds", "Weight").unwrap();
        assert!((result - 2.20462).abs() < 0.001);
    }
}
