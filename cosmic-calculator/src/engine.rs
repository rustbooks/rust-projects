use std::f64::consts::{PI, E};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    LeftParen,
    RightParen,
    Percent,
    Factorial,
    Sqrt,
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
    Abs,
    Mod,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AngleMode {
    Degrees,
    Radians,
    Gradians,
}

#[derive(Debug, Clone)]
pub struct CalcEngine {
    pub expression: String,
    pub result: String,
    pub angle_mode: AngleMode,
    pub memory: f64,
    pub last_result: Option<f64>,
    pub error: Option<String>,
}

impl Default for CalcEngine {
    fn default() -> Self {
        Self {
            expression: String::new(),
            result: String::from("0"),
            angle_mode: AngleMode::Degrees,
            memory: 0.0,
            last_result: None,
            error: None,
        }
    }
}

impl CalcEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, ch: &str) {
        self.error = None;
        self.expression.push_str(ch);
    }

    pub fn backspace(&mut self) {
        if !self.expression.is_empty() {
            // Remove last UTF-8 character
            let mut chars = self.expression.chars();
            chars.next_back();
            self.expression = chars.as_str().to_string();
        }
    }

    pub fn clear(&mut self) {
        self.expression.clear();
        self.result = String::from("0");
        self.error = None;
    }

    pub fn clear_entry(&mut self) {
        // Remove last number entry
        let trimmed = self.expression.trim_end_matches(|c: char| c.is_ascii_digit() || c == '.');
        self.expression = trimmed.to_string();
    }

    pub fn evaluate(&mut self) -> Option<f64> {
        let expr = self.expression.trim().to_string();
        if expr.is_empty() {
            return None;
        }

        match self.parse_and_eval(&expr) {
            Ok(val) => {
                self.last_result = Some(val);
                self.result = format_number(val);
                self.error = None;
                Some(val)
            }
            Err(e) => {
                self.error = Some(e.clone());
                self.result = format!("Error: {}", e);
                None
            }
        }
    }

    pub fn negate(&mut self) {
        if let Ok(val) = self.expression.parse::<f64>() {
            self.expression = format_number(-val);
        } else {
            self.expression = format!("-({})", self.expression);
        }
    }

    pub fn percentage(&mut self) {
        if let Ok(val) = self.expression.parse::<f64>() {
            self.expression = format_number(val / 100.0);
        } else {
            self.append("%");
        }
    }

    pub fn memory_store(&mut self) {
        if let Ok(val) = self.result.parse::<f64>() {
            self.memory = val;
        }
    }

    pub fn memory_recall(&mut self) {
        self.expression.push_str(&format_number(self.memory));
    }

    pub fn memory_add(&mut self) {
        if let Ok(val) = self.result.parse::<f64>() {
            self.memory += val;
        }
    }

    pub fn memory_subtract(&mut self) {
        if let Ok(val) = self.result.parse::<f64>() {
            self.memory -= val;
        }
    }

    pub fn memory_clear(&mut self) {
        self.memory = 0.0;
    }

    fn to_radians(&self, val: f64) -> f64 {
        match self.angle_mode {
            AngleMode::Degrees => val.to_radians(),
            AngleMode::Radians => val,
            AngleMode::Gradians => val * PI / 200.0,
        }
    }

    fn from_radians(&self, val: f64) -> f64 {
        match self.angle_mode {
            AngleMode::Degrees => val.to_degrees(),
            AngleMode::Radians => val,
            AngleMode::Gradians => val * 200.0 / PI,
        }
    }

    fn parse_and_eval(&self, expr: &str) -> Result<f64, String> {
        let tokens = self.tokenize(expr)?;
        let result = self.eval_tokens(&tokens, &mut 0)?;
        Ok(result)
    }

    fn tokenize(&self, expr: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = expr.chars().peekable();
        let mut i = 0;
        let chars_vec: Vec<char> = expr.chars().collect();

        while i < chars_vec.len() {
            let ch = chars_vec[i];
            match ch {
                ' ' | '\t' => { i += 1; }
                '0'..='9' | '.' => {
                    let mut num = String::new();
                    while i < chars_vec.len() && (chars_vec[i].is_ascii_digit() || chars_vec[i] == '.') {
                        num.push(chars_vec[i]);
                        i += 1;
                    }
                    let val: f64 = num.parse().map_err(|_| format!("Invalid number: {}", num))?;
                    tokens.push(Token::Number(val));
                }
                '+' => { tokens.push(Token::Plus); i += 1; }
                '-' => {
                    // Check for unary minus
                    if tokens.is_empty() || matches!(tokens.last(), Some(Token::LeftParen) | Some(Token::Plus) | Some(Token::Minus) | Some(Token::Multiply) | Some(Token::Divide) | Some(Token::Power)) {
                        tokens.push(Token::Number(-1.0));
                        tokens.push(Token::Multiply);
                    } else {
                        tokens.push(Token::Minus);
                    }
                    i += 1;
                }
                '×' | '*' => { tokens.push(Token::Multiply); i += 1; }
                '÷' | '/' => { tokens.push(Token::Divide); i += 1; }
                '^' => { tokens.push(Token::Power); i += 1; }
                '(' => { tokens.push(Token::LeftParen); i += 1; }
                ')' => { tokens.push(Token::RightParen); i += 1; }
                '%' => { tokens.push(Token::Percent); i += 1; }
                '!' => { tokens.push(Token::Factorial); i += 1; }
                'π' => { tokens.push(Token::Pi); i += 1; }
                'e' if i + 1 < chars_vec.len() && chars_vec[i+1] != 'x' => {
                    // Euler's number
                    tokens.push(Token::E); i += 1;
                }
                _ => {
                    // Try to match function names
                    let rest: String = chars_vec[i..].iter().collect();
                    if rest.starts_with("sqrt") || rest.starts_with("√") {
                        tokens.push(Token::Sqrt);
                        i += if rest.starts_with("sqrt") { 4 } else { 1 };
                    } else if rest.starts_with("asin") {
                        tokens.push(Token::Asin); i += 4;
                    } else if rest.starts_with("acos") {
                        tokens.push(Token::Acos); i += 4;
                    } else if rest.starts_with("atan") {
                        tokens.push(Token::Atan); i += 4;
                    } else if rest.starts_with("sin") {
                        tokens.push(Token::Sin); i += 3;
                    } else if rest.starts_with("cos") {
                        tokens.push(Token::Cos); i += 3;
                    } else if rest.starts_with("tan") {
                        tokens.push(Token::Tan); i += 3;
                    } else if rest.starts_with("log") {
                        tokens.push(Token::Log); i += 3;
                    } else if rest.starts_with("ln") {
                        tokens.push(Token::Ln); i += 2;
                    } else if rest.starts_with("abs") {
                        tokens.push(Token::Abs); i += 3;
                    } else if rest.starts_with("mod") {
                        tokens.push(Token::Mod); i += 3;
                    } else if rest.starts_with("pi") {
                        tokens.push(Token::Pi); i += 2;
                    } else {
                        return Err(format!("Unknown character: '{}'", ch));
                    }
                }
            }
        }
        Ok(tokens)
    }

    fn eval_tokens(&self, tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
        self.parse_expr(tokens, pos)
    }

    fn parse_expr(&self, tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
        let mut left = self.parse_term(tokens, pos)?;

        while *pos < tokens.len() {
            match &tokens[*pos] {
                Token::Plus => {
                    *pos += 1;
                    left += self.parse_term(tokens, pos)?;
                }
                Token::Minus => {
                    *pos += 1;
                    left -= self.parse_term(tokens, pos)?;
                }
                Token::Mod => {
                    *pos += 1;
                    let right = self.parse_term(tokens, pos)?;
                    if right == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    left %= right;
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_term(&self, tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
        let mut left = self.parse_power(tokens, pos)?;

        while *pos < tokens.len() {
            match &tokens[*pos] {
                Token::Multiply => {
                    *pos += 1;
                    left *= self.parse_power(tokens, pos)?;
                }
                Token::Divide => {
                    *pos += 1;
                    let right = self.parse_power(tokens, pos)?;
                    if right == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    left /= right;
                }
                Token::Percent => {
                    *pos += 1;
                    left /= 100.0;
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_power(&self, tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
        let mut base = self.parse_unary(tokens, pos)?;

        if *pos < tokens.len() && tokens[*pos] == Token::Power {
            *pos += 1;
            let exp = self.parse_power(tokens, pos)?;
            base = base.powf(exp);
        }

        // Factorial
        while *pos < tokens.len() && tokens[*pos] == Token::Factorial {
            *pos += 1;
            base = factorial(base)?;
        }

        Ok(base)
    }

    fn parse_unary(&self, tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
        if *pos >= tokens.len() {
            return Err("Unexpected end of expression".to_string());
        }

        match &tokens[*pos].clone() {
            Token::Sqrt => {
                *pos += 1;
                let val = self.parse_unary(tokens, pos)?;
                if val < 0.0 {
                    return Err("Square root of negative number".to_string());
                }
                Ok(val.sqrt())
            }
            Token::Sin => {
                *pos += 1;
                let val = self.parse_unary(tokens, pos)?;
                Ok(self.to_radians(val).sin())
            }
            Token::Cos => {
                *pos += 1;
                let val = self.parse_unary(tokens, pos)?;
                Ok(self.to_radians(val).cos())
            }
            Token::Tan => {
                *pos += 1;
                let val = self.parse_unary(tokens, pos)?;
                let rad = self.to_radians(val);
                if (rad - PI / 2.0).abs() < 1e-10 || (rad + PI / 2.0).abs() < 1e-10 {
                    return Err("tan undefined at 90°".to_string());
                }
                Ok(rad.tan())
            }
            Token::Asin => {
                *pos += 1;
                let val = self.parse_unary(tokens, pos)?;
                if val < -1.0 || val > 1.0 {
                    return Err("asin domain error".to_string());
                }
                Ok(self.from_radians(val.asin()))
            }
            Token::Acos => {
                *pos += 1;
                let val = self.parse_unary(tokens, pos)?;
                if val < -1.0 || val > 1.0 {
                    return Err("acos domain error".to_string());
                }
                Ok(self.from_radians(val.acos()))
            }
            Token::Atan => {
                *pos += 1;
                let val = self.parse_unary(tokens, pos)?;
                Ok(self.from_radians(val.atan()))
            }
            Token::Log => {
                *pos += 1;
                let val = self.parse_unary(tokens, pos)?;
                if val <= 0.0 {
                    return Err("log of non-positive number".to_string());
                }
                Ok(val.log10())
            }
            Token::Ln => {
                *pos += 1;
                let val = self.parse_unary(tokens, pos)?;
                if val <= 0.0 {
                    return Err("ln of non-positive number".to_string());
                }
                Ok(val.ln())
            }
            Token::Abs => {
                *pos += 1;
                let val = self.parse_unary(tokens, pos)?;
                Ok(val.abs())
            }
            Token::Pi => {
                *pos += 1;
                Ok(PI)
            }
            Token::E => {
                *pos += 1;
                Ok(E)
            }
            Token::LeftParen => {
                *pos += 1;
                let val = self.parse_expr(tokens, pos)?;
                if *pos >= tokens.len() || tokens[*pos] != Token::RightParen {
                    return Err("Missing closing parenthesis".to_string());
                }
                *pos += 1;
                Ok(val)
            }
            Token::Number(n) => {
                let val = *n;
                *pos += 1;
                Ok(val)
            }
            _ => Err(format!("Unexpected token: {:?}", tokens[*pos])),
        }
    }
}

fn factorial(n: f64) -> Result<f64, String> {
    if n < 0.0 {
        return Err("Factorial of negative number".to_string());
    }
    if n != n.floor() {
        return Err("Factorial requires integer".to_string());
    }
    if n > 170.0 {
        return Err("Factorial too large".to_string());
    }
    let mut result = 1.0f64;
    let mut i = 2u64;
    while i <= n as u64 {
        result *= i as f64;
        i += 1;
    }
    Ok(result)
}

pub fn format_number(val: f64) -> String {
    if val.is_nan() {
        return "NaN".to_string();
    }
    if val.is_infinite() {
        return if val > 0.0 { "∞".to_string() } else { "-∞".to_string() };
    }
    if val == 0.0 {
        return "0".to_string();
    }

    let abs = val.abs();
    if abs >= 1e15 || (abs < 1e-10 && abs != 0.0) {
        return format!("{:e}", val);
    }

    // Remove trailing zeros
    let formatted = format!("{:.10}", val);
    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
    trimmed.to_string()
}
