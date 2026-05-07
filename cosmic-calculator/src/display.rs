/// Formats display value with thousands separators
pub fn format_display(value: &str) -> String {
    if value.contains('e') || value.contains('E') || value.contains('∞') || value.contains("NaN") {
        return value.to_string();
    }

    let (sign, rest) = if value.starts_with('-') {
        ("-", &value[1..])
    } else {
        ("", value)
    };

    let (integer_part, decimal_part) = if let Some(dot_pos) = rest.find('.') {
        (&rest[..dot_pos], Some(&rest[dot_pos+1..]))
    } else {
        (rest, None)
    };

    // Add thousands separators to integer part
    let with_sep = add_thousands_sep(integer_part);

    match decimal_part {
        Some(dec) => format!("{}{}.{}", sign, with_sep, dec),
        None => format!("{}{}", sign, with_sep),
    }
}

fn add_thousands_sep(s: &str) -> String {
    let digits: Vec<char> = s.chars().collect();
    let len = digits.len();
    let mut result = String::new();

    for (i, ch) in digits.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*ch);
    }
    result
}

pub struct Display;
