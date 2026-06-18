//! Trigonometry in degrees plus angle/hour normalization. The classical prayer
//! time formulas are all stated in degrees, so working in radians everywhere
//! just adds conversion noise and rounding error. Kept internal to the crate.

use std::f64::consts::PI;

#[inline(always)]
pub(crate) fn radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[inline(always)]
pub(crate) fn degrees(radians: f64) -> f64 {
    radians * 180.0 / PI
}

#[inline(always)]
pub(crate) fn sin(d: f64) -> f64 {
    radians(d).sin()
}

#[inline(always)]
pub(crate) fn cos(d: f64) -> f64 {
    radians(d).cos()
}

#[inline(always)]
pub(crate) fn tan(d: f64) -> f64 {
    radians(d).tan()
}

#[inline(always)]
pub(crate) fn asin(x: f64) -> f64 {
    degrees(x.asin())
}

#[inline(always)]
pub(crate) fn acos(x: f64) -> f64 {
    degrees(x.acos())
}

#[inline(always)]
pub(crate) fn atan2(y: f64, x: f64) -> f64 {
    degrees(y.atan2(x))
}

/// arccot in degrees.
#[inline(always)]
pub(crate) fn acot(x: f64) -> f64 {
    degrees(1.0_f64.atan2(x))
}

/// Wrap an angle into [0, 360).
#[inline(always)]
pub(crate) fn fix_angle(a: f64) -> f64 {
    fix(a, 360.0)
}

/// Wrap an hour value into [0, 24).
#[inline(always)]
pub(crate) fn fix_hour(h: f64) -> f64 {
    fix(h, 24.0)
}

#[inline(always)]
fn fix(value: f64, modulo: f64) -> f64 {
    let r = value - modulo * (value / modulo).floor();
    if r < 0.0 {
        r + modulo
    } else {
        r
    }
}
