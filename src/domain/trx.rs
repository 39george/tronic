use std::fmt;

use derive_more::{Add, AddAssign, Div, Mul, Sub, Sum};

#[derive(
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Add,
    Sub,
    AddAssign,
    Mul,
    Sum,
    Div,
)]
pub struct Trx(i64);

impl Trx {
    pub const ZERO: Trx = Trx(0);
    pub fn from_sun(sat: i64) -> Self {
        Self(sat)
    }
    pub fn to_sun(&self) -> i64 {
        self.0
    }
}

impl From<f64> for Trx {
    fn from(trx: f64) -> Self {
        const SUN_PER_TRX: f64 = 1_000_000.0;
        let sun = (trx * SUN_PER_TRX).round() as i64;
        Trx(sun)
    }
}

impl From<i64> for Trx {
    fn from(value: i64) -> Self {
        Trx(value)
    }
}

impl From<Trx> for i64 {
    fn from(value: Trx) -> Self {
        value.0
    }
}

impl fmt::Display for Trx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let trx = self.0 as f64 * 0.000_001; // 1 sun = 0.000001 TRX
        let formatted = if trx.fract() == 0.0 {
            // Integer value (e.g., 1.0 → "1 TRX")
            format!("{} TRX", trx as i64)
        } else {
            // Trim trailing zeros (e.g., 0.000010 → "0.00001 TRX")
            let mut s = format!("{trx:.6}");
            while s.ends_with('0') {
                s.pop(); // Remove trailing zeros
            }
            if s.ends_with('.') {
                s.pop(); // Remove trailing decimal point (e.g., "1." → "1")
            }
            format!("{s} TRX")
        };
        f.write_str(&formatted)
    }
}

impl fmt::Debug for Trx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

// TODO: add possibility to use integer literal with TRX label
#[macro_export]
macro_rules! trx {
    // Handle decimal TRX (converts to SUN)
    ($val:literal TRX) => {{
        const SUN_PER_TRX: i64 = 1_000_000;
        // Compile-time decimal to integer conversion
        const fn to_sun(trx: f64) -> i64 {
            (trx * SUN_PER_TRX as f64) as i64
        }
        $crate::domain::trx::Trx::from(to_sun($val))
    }};

    // Handle integer SUN
    ($val:literal SUN) => {
        $crate::domain::trx::Trx::from($val)
    };

    // Handle zero case explicitly
    (0) => {
        $crate::domain::trx::Trx::ZERO
    };
}
