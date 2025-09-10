use slint::Color;

pub struct AppTheme {
    pub primary: Color,
    pub accent: Color,
    pub background: Color,
}

impl AppTheme {
    pub fn new() -> Self {
        Self {
            primary: Color::from_rgb_u8(98, 0, 238), // #6200EE
            accent: Color::from_rgb_u8(3, 218, 198), // #03DAC6
            background: Color::from_rgb_u8(18, 18, 18), // #121212
        }
    }
}