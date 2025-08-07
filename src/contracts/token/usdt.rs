use alloy_primitives::U256;
use serde::{Deserialize, Serialize};
use std::fmt;

use super::{Token, TokenError};

#[derive(
    Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct Usdt(U256);

impl Usdt {
    /// Creates USDT from human-readable decimal amount
    pub fn from_decimal(value: f64) -> Result<Self, TokenError> {
        let micro_usdt = (value * 1_000_000.0).round(); // 6 decimals
        if micro_usdt.is_nan() || micro_usdt.is_infinite() {
            return Err(TokenError::InvalidAmount);
        }

        // Convert to U256 safely
        let amount = U256::from(micro_usdt.abs() as u128);
        Ok(Self(amount))
    }

    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }

    pub fn checked_sub(self, other: Self) -> Option<Self> {
        self.0.checked_sub(other.0).map(Self)
    }

    /// Returns the amount as a decimal string without trailing zeros
    pub fn to_decimal_string(&self) -> String {
        let divisor = U256::from(10).pow(U256::from(Self::decimals()));
        let integer = self.0 / divisor;
        let fraction = self.0 % divisor;

        let mut s = format!("{integer}.{fraction:06}");
        while s.ends_with('0') {
            s.pop();
        }
        if s.ends_with('.') {
            s.pop();
        }
        s
    }
}

impl From<U256> for Usdt {
    fn from(value: U256) -> Self {
        Usdt(value)
    }
}

impl From<Usdt> for U256 {
    fn from(value: Usdt) -> Self {
        value.0
    }
}

impl Token for Usdt {
    fn symbol() -> &'static str {
        "USDT"
    }

    fn decimals() -> u32 {
        6
    }
}

impl fmt::Display for Usdt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.to_decimal_string(), Self::symbol())
    }
}

impl fmt::Debug for Usdt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Usdt")
            .field(&self.to_decimal_string())
            .finish()
    }
}

#[macro_export]
macro_rules! usdt {
    ($val:literal USDT) => {{
        $crate::contracts::token::usdt::Usdt::from_decimal($val)
            .expect("Invalid USDT amount")
    }};
    ($val:literal MICRO_USDT) => {
        $crate::contracts::token::usdt::Usdt::from_base_units(U256::from($val))
    };
}
