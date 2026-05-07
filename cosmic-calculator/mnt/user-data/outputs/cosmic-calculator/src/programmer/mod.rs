#[derive(Debug, Clone, PartialEq, Default)]
pub enum Base {
    Hex,
    #[default]
    Dec,
    Oct,
    Bin,
}

impl Base {
    pub fn label(&self) -> &'static str {
        match self {
            Base::Hex => "HEX",
            Base::Dec => "DEC",
            Base::Oct => "OCT",
            Base::Bin => "BIN",
        }
    }

    pub fn radix(&self) -> u32 {
        match self {
            Base::Hex => 16,
            Base::Dec => 10,
            Base::Oct => 8,
            Base::Bin => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum BitWidth {
    Bit8,
    Bit16,
    #[default]
    Bit32,
    Bit64,
}

impl BitWidth {
    pub fn label(&self) -> &'static str {
        match self {
            BitWidth::Bit8 => "8-bit",
            BitWidth::Bit16 => "16-bit",
            BitWidth::Bit32 => "32-bit",
            BitWidth::Bit64 => "64-bit",
        }
    }

    pub fn mask(&self) -> u64 {
        match self {
            BitWidth::Bit8 => 0xFF,
            BitWidth::Bit16 => 0xFFFF,
            BitWidth::Bit32 => 0xFFFFFFFF,
            BitWidth::Bit64 => 0xFFFFFFFFFFFFFFFF,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProgrammerCalc {
    pub base: Base,
    pub bit_width: BitWidth,
    pub expression: String,
    pub result: i64,
    pub display_value: String,
    pub error: Option<String>,
}

impl ProgrammerCalc {
    pub fn new() -> Self {
        Self {
            base: Base::Dec,
            bit_width: BitWidth::Bit32,
            expression: String::new(),
            result: 0,
            display_value: String::from("0"),
            error: None,
        }
    }

    pub fn append(&mut self, ch: &str) {
        self.error = None;
        self.expression.push_str(ch);
        self.update_display();
    }

    pub fn backspace(&mut self) {
        let mut chars = self.expression.chars();
        chars.next_back();
        self.expression = chars.as_str().to_string();
        self.update_display();
    }

    pub fn clear(&mut self) {
        self.expression.clear();
        self.result = 0;
        self.display_value = "0".to_string();
        self.error = None;
    }

    pub fn evaluate(&mut self) {
        match self.parse_expr() {
            Ok(val) => {
                let masked = (val as u64) & self.bit_width.mask();
                self.result = masked as i64;
                self.update_display();
                self.error = None;
            }
            Err(e) => {
                self.error = Some(e);
            }
        }
    }

    fn update_display(&self) -> () {}

    pub fn get_display(&self) -> String {
        match self.base {
            Base::Dec => format!("{}", self.result),
            Base::Hex => format!("{:X}", (self.result as u64) & self.bit_width.mask()),
            Base::Oct => format!("{:o}", (self.result as u64) & self.bit_width.mask()),
            Base::Bin => format!("{:b}", (self.result as u64) & self.bit_width.mask()),
        }
    }

    pub fn get_value_in_base(&self, base: &Base) -> String {
        let val = (self.result as u64) & self.bit_width.mask();
        match base {
            Base::Dec => format!("{}", val as i64),
            Base::Hex => {
                let hex = format!("{:X}", val);
                // Group in 4s
                let pad_len = ((hex.len() + 3) / 4) * 4;
                let padded = format!("{:0>width$}", hex, width = pad_len);
                padded.chars().collect::<Vec<_>>().chunks(4)
                    .map(|c| c.iter().collect::<String>())
                    .collect::<Vec<_>>().join(" ")
            }
            Base::Oct => format!("{:o}", val),
            Base::Bin => {
                let bin = format!("{:b}", val);
                let pad_len = ((bin.len() + 7) / 8) * 8;
                let padded = format!("{:0>width$}", bin, width = pad_len);
                padded.chars().collect::<Vec<_>>().chunks(8)
                    .map(|c| c.iter().collect::<String>())
                    .collect::<Vec<_>>().join(" ")
            }
        }
    }

    pub fn get_bits(&self) -> Vec<bool> {
        let val = (self.result as u64) & self.bit_width.mask();
        let num_bits = match self.bit_width {
            BitWidth::Bit8 => 8,
            BitWidth::Bit16 => 16,
            BitWidth::Bit32 => 32,
            BitWidth::Bit64 => 64,
        };
        (0..num_bits).rev().map(|i| (val >> i) & 1 == 1).collect()
    }

    pub fn toggle_bit(&mut self, bit: usize) {
        let num_bits = match self.bit_width {
            BitWidth::Bit8 => 8usize,
            BitWidth::Bit16 => 16,
            BitWidth::Bit32 => 32,
            BitWidth::Bit64 => 64,
        };
        if bit < num_bits {
            let actual_bit = num_bits - 1 - bit;
            self.result ^= 1i64 << actual_bit;
        }
    }

    pub fn apply_bitwise(&mut self, op: BitwiseOp) {
        self.expression.push_str(match op {
            BitwiseOp::And => " AND ",
            BitwiseOp::Or => " OR ",
            BitwiseOp::Xor => " XOR ",
            BitwiseOp::Not => "NOT ",
            BitwiseOp::LShift => " << ",
            BitwiseOp::RShift => " >> ",
        });
    }

    fn parse_expr(&self) -> Result<i64, String> {
        let expr = self.expression.trim().to_uppercase();
        if expr.is_empty() {
            return Ok(0);
        }

        // Tokenize with AND/OR/XOR/NOT support
        let tokens = self.tokenize_prog(&expr)?;
        self.eval_prog(&tokens, &mut 0)
    }

    fn tokenize_prog(&self, expr: &str) -> Result<Vec<ProgToken>, String> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = expr.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            match chars[i] {
                ' ' | '\t' => { i += 1; }
                '+' => { tokens.push(ProgToken::Plus); i += 1; }
                '-' => { tokens.push(ProgToken::Minus); i += 1; }
                '*' => { tokens.push(ProgToken::Mul); i += 1; }
                '/' => { tokens.push(ProgToken::Div); i += 1; }
                '%' => { tokens.push(ProgToken::Mod); i += 1; }
                '(' => { tokens.push(ProgToken::LParen); i += 1; }
                ')' => { tokens.push(ProgToken::RParen); i += 1; }
                '~' => { tokens.push(ProgToken::BitNot); i += 1; }
                '&' => { tokens.push(ProgToken::And); i += 1; }
                '|' => { tokens.push(ProgToken::Or); i += 1; }
                '^' => { tokens.push(ProgToken::Xor); i += 1; }
                '<' if i + 1 < chars.len() && chars[i + 1] == '<' => {
                    tokens.push(ProgToken::Lsh); i += 2;
                }
                '>' if i + 1 < chars.len() && chars[i + 1] == '>' => {
                    tokens.push(ProgToken::Rsh); i += 2;
                }
                _ => {
                    let rest: String = chars[i..].iter().collect();
                    if rest.starts_with("AND") {
                        tokens.push(ProgToken::And); i += 3;
                    } else if rest.starts_with("OR") {
                        tokens.push(ProgToken::Or); i += 2;
                    } else if rest.starts_with("XOR") {
                        tokens.push(ProgToken::Xor); i += 3;
                    } else if rest.starts_with("NOT") {
                        tokens.push(ProgToken::BitNot); i += 3;
                    } else if chars[i] == '0' && i + 1 < chars.len() && chars[i+1] == 'X' {
                        // Hex literal
                        i += 2;
                        let mut hex = String::new();
                        while i < chars.len() && chars[i].is_ascii_hexdigit() {
                            hex.push(chars[i]); i += 1;
                        }
                        let val = i64::from_str_radix(&hex, 16)
                            .map_err(|_| format!("Invalid hex: {}", hex))?;
                        tokens.push(ProgToken::Num(val));
                    } else if chars[i].is_ascii_hexdigit() {
                        let mut num = String::new();
                        while i < chars.len() && chars[i].is_ascii_hexdigit() {
                            num.push(chars[i]); i += 1;
                        }
                        let val = i64::from_str_radix(&num, self.base.radix())
                            .map_err(|_| format!("Invalid number: {}", num))?;
                        tokens.push(ProgToken::Num(val));
                    } else {
                        return Err(format!("Unknown char: '{}'", chars[i]));
                    }
                }
            }
        }
        Ok(tokens)
    }

    fn eval_prog(&self, tokens: &[ProgToken], pos: &mut usize) -> Result<i64, String> {
        self.eval_or(tokens, pos)
    }

    fn eval_or(&self, tokens: &[ProgToken], pos: &mut usize) -> Result<i64, String> {
        let mut left = self.eval_xor(tokens, pos)?;
        while *pos < tokens.len() && tokens[*pos] == ProgToken::Or {
            *pos += 1;
            left |= self.eval_xor(tokens, pos)?;
        }
        Ok(left)
    }

    fn eval_xor(&self, tokens: &[ProgToken], pos: &mut usize) -> Result<i64, String> {
        let mut left = self.eval_and(tokens, pos)?;
        while *pos < tokens.len() && tokens[*pos] == ProgToken::Xor {
            *pos += 1;
            left ^= self.eval_and(tokens, pos)?;
        }
        Ok(left)
    }

    fn eval_and(&self, tokens: &[ProgToken], pos: &mut usize) -> Result<i64, String> {
        let mut left = self.eval_shift(tokens, pos)?;
        while *pos < tokens.len() && tokens[*pos] == ProgToken::And {
            *pos += 1;
            left &= self.eval_shift(tokens, pos)?;
        }
        Ok(left)
    }

    fn eval_shift(&self, tokens: &[ProgToken], pos: &mut usize) -> Result<i64, String> {
        let mut left = self.eval_add(tokens, pos)?;
        while *pos < tokens.len() {
            match &tokens[*pos] {
                ProgToken::Lsh => { *pos += 1; let r = self.eval_add(tokens, pos)?; left <<= r; }
                ProgToken::Rsh => { *pos += 1; let r = self.eval_add(tokens, pos)?; left >>= r; }
                _ => break,
            }
        }
        Ok(left)
    }

    fn eval_add(&self, tokens: &[ProgToken], pos: &mut usize) -> Result<i64, String> {
        let mut left = self.eval_mul(tokens, pos)?;
        while *pos < tokens.len() {
            match &tokens[*pos] {
                ProgToken::Plus => { *pos += 1; left = left.wrapping_add(self.eval_mul(tokens, pos)?); }
                ProgToken::Minus => { *pos += 1; left = left.wrapping_sub(self.eval_mul(tokens, pos)?); }
                _ => break,
            }
        }
        Ok(left)
    }

    fn eval_mul(&self, tokens: &[ProgToken], pos: &mut usize) -> Result<i64, String> {
        let mut left = self.eval_unary(tokens, pos)?;
        while *pos < tokens.len() {
            match &tokens[*pos] {
                ProgToken::Mul => { *pos += 1; left = left.wrapping_mul(self.eval_unary(tokens, pos)?); }
                ProgToken::Div => {
                    *pos += 1;
                    let r = self.eval_unary(tokens, pos)?;
                    if r == 0 { return Err("Division by zero".to_string()); }
                    left /= r;
                }
                ProgToken::Mod => {
                    *pos += 1;
                    let r = self.eval_unary(tokens, pos)?;
                    if r == 0 { return Err("Division by zero".to_string()); }
                    left %= r;
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn eval_unary(&self, tokens: &[ProgToken], pos: &mut usize) -> Result<i64, String> {
        if *pos < tokens.len() && tokens[*pos] == ProgToken::BitNot {
            *pos += 1;
            return Ok(!self.eval_unary(tokens, pos)?);
        }
        self.eval_primary(tokens, pos)
    }

    fn eval_primary(&self, tokens: &[ProgToken], pos: &mut usize) -> Result<i64, String> {
        if *pos >= tokens.len() {
            return Err("Unexpected end".to_string());
        }
        match &tokens[*pos] {
            ProgToken::Num(n) => { let v = *n; *pos += 1; Ok(v) }
            ProgToken::LParen => {
                *pos += 1;
                let val = self.eval_prog(tokens, pos)?;
                if *pos >= tokens.len() || tokens[*pos] != ProgToken::RParen {
                    return Err("Missing )".to_string());
                }
                *pos += 1;
                Ok(val)
            }
            _ => Err(format!("Unexpected token: {:?}", tokens[*pos])),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BitwiseOp {
    And, Or, Xor, Not, LShift, RShift,
}

#[derive(Debug, Clone, PartialEq)]
enum ProgToken {
    Num(i64), Plus, Minus, Mul, Div, Mod,
    And, Or, Xor, BitNot, Lsh, Rsh,
    LParen, RParen,
}
