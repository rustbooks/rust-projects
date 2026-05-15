// src/scientific.rs
//
// Additional scientific helpers used by the calculator.
// Core trig/log functions are handled directly in calculator.rs
// via the apply_function() method. This module provides
// supplementary utilities.

use crate::error::{CalcError, CalcResult};
use std::f64::consts::{E, PI};

/// Gamma function approximation (Stirling / Lanczos)
/// Used for non-integer factorials.
pub fn gamma(x: f64) -> CalcResult<f64> {
    if x <= 0.0 && x.fract() == 0.0 {
        return Err(CalcError::MathDomain("Gamma: poles at non-positive integers".into()));
    }
    // Lanczos approximation (g=7, n=9)
    const G: f64 = 7.0;
    const C: [f64; 9] = [
        0.99999999999980993,
        676.5203681218851,
        -1259.1392167224028,
        771.32342877765313,
        -176.61502916214059,
        12.507343278686905,
        -0.13857109526572012,
        9.9843695780195716e-6,
        1.5056327351493116e-7,
    ];
    if x < 0.5 {
        Ok(PI / ((PI * x).sin() * gamma(1.0 - x)?))
    } else {
        let x = x - 1.0;
        let mut a = C[0];
        let t = x + G + 0.5;
        for (i, &ci) in C[1..].iter().enumerate() {
            a += ci / (x + i as f64 + 1.0);
        }
        Ok((2.0 * PI).sqrt() * t.powf(x + 0.5) * (-t).exp() * a)
    }
}

/// log base n: log_n(x) = ln(x) / ln(n)
pub fn log_base(x: f64, base: f64) -> CalcResult<f64> {
    if x <= 0.0 { return Err(CalcError::MathDomain("log domain: x > 0".into())); }
    if base <= 0.0 || (base - 1.0).abs() < f64::EPSILON {
        return Err(CalcError::MathDomain("log base must be > 0 and ≠ 1".into()));
    }
    Ok(x.ln() / base.ln())
}

/// Hyperbolic functions
pub fn sinh(x: f64) -> f64 { x.sinh() }
pub fn cosh(x: f64) -> f64 { x.cosh() }
pub fn tanh(x: f64) -> f64 { x.tanh() }

/// Degrees ↔ Radians helpers
pub fn deg_to_rad(deg: f64) -> f64 { deg * PI / 180.0 }
pub fn rad_to_deg(rad: f64) -> f64 { rad * 180.0 / PI }

/// Permutations: P(n, r) = n! / (n-r)!
pub fn permutations(n: u64, r: u64) -> CalcResult<f64> {
    if r > n { return Err(CalcError::MathDomain("r must be <= n".into())); }
    let mut result = 1u64;
    for i in (n - r + 1)..=n {
        result = result.checked_mul(i).ok_or(CalcError::Overflow)?;
    }
    Ok(result as f64)
}

/// Combinations: C(n, r) = n! / (r! * (n-r)!)
pub fn combinations(n: u64, r: u64) -> CalcResult<f64> {
    if r > n { return Err(CalcError::MathDomain("r must be <= n".into())); }
    let r = r.min(n - r); // use smaller r
    let mut result = 1.0f64;
    for i in 0..r {
        result *= (n - i) as f64;
        result /= (i + 1) as f64;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_base() {
        assert!((log_base(1000.0, 10.0).unwrap() - 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_combinations() {
        assert_eq!(combinations(5, 2).unwrap(), 10.0);
    }

    #[test]
    fn test_permutations() {
        assert_eq!(permutations(5, 2).unwrap(), 20.0);
    }

    #[test]
    fn test_deg_rad() {
        assert!((deg_to_rad(180.0) - PI).abs() < 1e-9);
        assert!((rad_to_deg(PI) - 180.0).abs() < 1e-9);
    }
}
