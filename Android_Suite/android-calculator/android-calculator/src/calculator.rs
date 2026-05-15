// src/calculator.rs
//
// Recursive-descent expression parser + evaluator.
// Supports: +  -  *  /  %  ^  unary-minus  parentheses
//           sin cos tan asin acos atan log ln sqrt cbrt x² x^y 1/x ! π e

use crate::error::{CalcError, CalcResult};

// ── Tokeniser ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    LParen,
    RParen,
    Ident(String),
    End,
}

struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        // Normalise display symbols to ASCII operators
        let normalised = input
            .replace('×', "*")
            .replace('÷', "/")
            .replace('−', "-")
            .replace("x²", "sq(")   // handled as function
            .replace("√x", "sqrt(") // ditto
            .replace("∛x", "cbrt(")
            .replace("1/x", "inv(")
            .replace("xʸ", "^")
            .replace('π', "pi")
            .replace("²", "^2")
            .replace("³", "^3");
        Self {
            chars: normalised.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).copied();
        self.pos += 1;
        c
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(' ' | '\t' | '\n')) {
            self.advance();
        }
    }

    fn next_token(&mut self) -> CalcResult<Token> {
        self.skip_ws();
        match self.peek() {
            None => Ok(Token::End),
            Some(c) => match c {
                '+' => { self.advance(); Ok(Token::Plus) }
                '-' => { self.advance(); Ok(Token::Minus) }
                '*' => { self.advance(); Ok(Token::Star) }
                '/' => { self.advance(); Ok(Token::Slash) }
                '%' => { self.advance(); Ok(Token::Percent) }
                '^' => { self.advance(); Ok(Token::Caret) }
                '(' => { self.advance(); Ok(Token::LParen) }
                ')' => { self.advance(); Ok(Token::RParen) }
                '0'..='9' | '.' => {
                    let mut s = String::new();
                    while matches!(self.peek(), Some('0'..='9' | '.')) {
                        s.push(self.advance().unwrap());
                    }
                    s.parse::<f64>()
                        .map(Token::Number)
                        .map_err(|_| CalcError::InvalidExpression(s))
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut s = String::new();
                    while matches!(self.peek(), Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_')) {
                        s.push(self.advance().unwrap());
                    }
                    Ok(Token::Ident(s))
                }
                other => Err(CalcError::InvalidExpression(format!("Unknown char: {other}"))),
            }
        }
    }
}

// ── Parser / Evaluator ───────────────────────────────────────────────────────

pub struct Calculator {
    tokens: Vec<Token>,
    pos: usize,
    pub use_degrees: bool,
}

impl Calculator {
    pub fn new() -> Self {
        Self { tokens: vec![], pos: 0, use_degrees: true }
    }

    fn tokenise(input: &str) -> CalcResult<Vec<Token>> {
        let mut lexer = Lexer::new(input);
        let mut out = Vec::new();
        loop {
            let tok = lexer.next_token()?;
            let done = tok == Token::End;
            out.push(tok);
            if done { break; }
        }
        Ok(out)
    }

    pub fn evaluate(&mut self, input: &str) -> CalcResult<f64> {
        self.tokens = Self::tokenise(input)?;
        self.pos = 0;
        let val = self.parse_expr()?;
        if self.current() != &Token::End {
            return Err(CalcError::InvalidExpression("Unexpected token".into()));
        }
        if val.is_nan() {
            return Err(CalcError::MathDomain("Result is NaN".into()));
        }
        if val.is_infinite() {
            return Err(CalcError::Overflow);
        }
        Ok(val)
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::End)
    }

    fn consume(&mut self) -> Token {
        let t = self.tokens.get(self.pos).cloned().unwrap_or(Token::End);
        self.pos += 1;
        t
    }

    // Expr = Term (('+' | '-') Term)*
    fn parse_expr(&mut self) -> CalcResult<f64> {
        let mut lhs = self.parse_term()?;
        loop {
            match self.current() {
                Token::Plus  => { self.consume(); lhs += self.parse_term()?; }
                Token::Minus => { self.consume(); lhs -= self.parse_term()?; }
                _ => break,
            }
        }
        Ok(lhs)
    }

    // Term = Unary (('*' | '/' | '%') Unary)*
    fn parse_term(&mut self) -> CalcResult<f64> {
        let mut lhs = self.parse_power()?;
        loop {
            match self.current() {
                Token::Star => {
                    self.consume();
                    lhs *= self.parse_power()?;
                }
                Token::Slash => {
                    self.consume();
                    let rhs = self.parse_power()?;
                    if rhs == 0.0 { return Err(CalcError::DivisionByZero); }
                    lhs /= rhs;
                }
                Token::Percent => {
                    self.consume();
                    let rhs = self.parse_power()?;
                    if rhs == 0.0 { return Err(CalcError::DivisionByZero); }
                    lhs %= rhs;
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    // Power = Unary ('^' Unary)*   (right-associative)
    fn parse_power(&mut self) -> CalcResult<f64> {
        let base = self.parse_unary()?;
        if self.current() == &Token::Caret {
            self.consume();
            let exp = self.parse_power()?;
            Ok(base.powf(exp))
        } else {
            Ok(base)
        }
    }

    // Unary = '-' Unary | Factorial
    fn parse_unary(&mut self) -> CalcResult<f64> {
        if self.current() == &Token::Minus {
            self.consume();
            Ok(-self.parse_unary()?)
        } else {
            self.parse_factorial()
        }
    }

    // Factorial = Primary ('!')*
    fn parse_factorial(&mut self) -> CalcResult<f64> {
        let mut val = self.parse_primary()?;
        while self.current() == &Token::Ident("!".to_string()) {
            self.consume();
            val = factorial(val)?;
        }
        Ok(val)
    }

    // Primary = Number | Ident '(' Expr ')' | '(' Expr ')' | Const
    fn parse_primary(&mut self) -> CalcResult<f64> {
        match self.current().clone() {
            Token::Number(n) => { self.consume(); Ok(n) }

            Token::LParen => {
                self.consume();
                let val = self.parse_expr()?;
                if self.current() != &Token::RParen {
                    return Err(CalcError::InvalidExpression("Expected ')'".into()));
                }
                self.consume();
                Ok(val)
            }

            Token::Ident(name) => {
                self.consume();
                match name.as_str() {
                    // Constants
                    "pi" | "PI" | "π"   => Ok(std::f64::consts::PI),
                    "e"  | "E"           => Ok(std::f64::consts::E),

                    // Functions with argument in parens
                    func => {
                        // Consume opening paren if present
                        let has_paren = self.current() == &Token::LParen;
                        if has_paren { self.consume(); }
                        let arg = self.parse_expr()?;
                        if has_paren {
                            if self.current() != &Token::RParen {
                                return Err(CalcError::InvalidExpression("Expected ')'".into()));
                            }
                            self.consume();
                        }
                        self.apply_function(func, arg)
                    }
                }
            }

            other => Err(CalcError::InvalidExpression(format!("Unexpected token: {other:?}"))),
        }
    }

    fn to_rad(&self, x: f64) -> f64 {
        if self.use_degrees { x.to_radians() } else { x }
    }

    fn from_rad(&self, x: f64) -> f64 {
        if self.use_degrees { x.to_degrees() } else { x }
    }

    fn apply_function(&self, name: &str, arg: f64) -> CalcResult<f64> {
        match name {
            "sin"  => Ok(self.to_rad(arg).sin()),
            "cos"  => Ok(self.to_rad(arg).cos()),
            "tan"  => Ok(self.to_rad(arg).tan()),
            "asin" | "arcsin" => {
                if !(-1.0..=1.0).contains(&arg) {
                    return Err(CalcError::MathDomain("asin domain: [-1, 1]".into()));
                }
                Ok(self.from_rad(arg.asin()))
            }
            "acos" | "arccos" => {
                if !(-1.0..=1.0).contains(&arg) {
                    return Err(CalcError::MathDomain("acos domain: [-1, 1]".into()));
                }
                Ok(self.from_rad(arg.acos()))
            }
            "atan" | "arctan" => Ok(self.from_rad(arg.atan())),
            "log"  | "log10"  => {
                if arg <= 0.0 { return Err(CalcError::MathDomain("log domain: x > 0".into())); }
                Ok(arg.log10())
            }
            "ln"   | "log_e"  => {
                if arg <= 0.0 { return Err(CalcError::MathDomain("ln domain: x > 0".into())); }
                Ok(arg.ln())
            }
            "sqrt" => {
                if arg < 0.0 { return Err(CalcError::MathDomain("sqrt domain: x >= 0".into())); }
                Ok(arg.sqrt())
            }
            "cbrt" => Ok(arg.cbrt()),
            "sq"   => Ok(arg * arg),
            "inv"  => {
                if arg == 0.0 { return Err(CalcError::DivisionByZero); }
                Ok(1.0 / arg)
            }
            "abs"  => Ok(arg.abs()),
            "ceil" => Ok(arg.ceil()),
            "floor"=> Ok(arg.floor()),
            "round"=> Ok(arg.round()),
            "exp"  => Ok(arg.exp()),
            "!"    => factorial(arg),
            other  => Err(CalcError::InvalidExpression(format!("Unknown function: {other}"))),
        }
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

fn factorial(n: f64) -> CalcResult<f64> {
    if n < 0.0 || n.fract() != 0.0 {
        return Err(CalcError::MathDomain("Factorial: non-negative integers only".into()));
    }
    if n > 170.0 {
        return Err(CalcError::Overflow);
    }
    let mut result = 1.0_f64;
    let mut i = 2u64;
    while i <= n as u64 {
        result *= i as f64;
        i += 1;
    }
    Ok(result)
}

/// Format a float for display: trim trailing zeros, limit decimals.
pub fn format_result(v: f64) -> String {
    if v.abs() >= 1e15 || (v != 0.0 && v.abs() < 1e-10) {
        // Scientific notation
        format!("{:.6e}", v)
    } else {
        let s = format!("{:.10}", v);
        let s = s.trim_end_matches('0');
        let s = s.trim_end_matches('.');
        s.to_string()
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn calc(expr: &str) -> f64 {
        Calculator::new().evaluate(expr).unwrap()
    }

    #[test]
    fn test_basic_arithmetic() {
        assert_eq!(calc("2+3"), 5.0);
        assert_eq!(calc("10-4"), 6.0);
        assert_eq!(calc("3*4"), 12.0);
        assert_eq!(calc("10/4"), 2.5);
    }

    #[test]
    fn test_precedence() {
        assert_eq!(calc("2+3*4"), 14.0);
        assert_eq!(calc("(2+3)*4"), 20.0);
    }

    #[test]
    fn test_power() {
        assert_eq!(calc("2^10"), 1024.0);
    }

    #[test]
    fn test_division_by_zero() {
        assert!(Calculator::new().evaluate("1/0").is_err());
    }

    #[test]
    fn test_sqrt() {
        assert!((calc("sqrt(16)") - 4.0).abs() < 1e-9);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(calc("5!"), 120.0);
    }

    #[test]
    fn test_pi_e() {
        assert!((calc("pi") - std::f64::consts::PI).abs() < 1e-9);
    }
}
