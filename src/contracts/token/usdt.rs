use std::fmt;

use alloy_primitives::U256;
use derive_more::{Add, AddAssign, Div, Mul, Sub, Sum};
use serde::{Deserialize, Serialize};

use super::{Token, TokenError};

#[derive(
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Hash,
    Add,
    AddAssign,
    Div,
    Mul,
    Sub,
    Sum,
)]
#[mul(forward)]
#[div(forward)]
pub struct Usdt(U256);

impl Usdt {
    pub const ZERO: Usdt = Usdt(U256::ZERO);

    /// Creates USDT from human-readable decimal amount.
    ///
    /// # Errors
    /// Returns [`TokenError::InvalidAmount`] if `value` is NaN, infinite,
    /// negative, or too large to represent.
    pub fn from_decimal(value: f64) -> Result<Self, TokenError> {
        let micro_usdt = (value * 1_000_000.0).round(); // 6 decimals
        if !micro_usdt.is_finite()
            || micro_usdt < 0.0
            || micro_usdt > u128::MAX as f64
        {
            return Err(TokenError::InvalidAmount);
        }

        Ok(Self(U256::from(micro_usdt as u128)))
    }

    pub fn is_whole(&self) -> bool {
        self.0 % U256::from(1_000_000) == U256::ZERO
    }

    /// Converts the raw amount (micro-USDT) into `T`.
    ///
    /// # Errors
    /// Returns an error if the value doesn't fit into `T`.
    pub fn try_to<T>(&self) -> Result<T, T::Error>
    where
        T: TryFrom<U256>,
    {
        T::try_from(self.0)
    }

    /// Converts the raw amount (micro-USDT) into `T`.
    ///
    /// # Panics
    /// Panics if the value doesn't fit into `T`. See [`Self::try_to`].
    #[expect(
        clippy::expect_used,
        reason = "documented panicking shorthand for try_to"
    )]
    pub fn to<T>(&self) -> T
    where
        T: TryFrom<U256>,
        T::Error: std::fmt::Debug,
    {
        self.try_to().expect("value doesn't fit into target type")
    }

    /// Creates USDT from a raw amount (micro-USDT).
    ///
    /// # Errors
    /// Returns an error if `val` can't be represented as `U256`
    /// (e.g. it is negative).
    pub fn try_from_value<T>(
        val: T,
    ) -> Result<Self, <U256 as TryFrom<T>>::Error>
    where
        U256: TryFrom<T>,
    {
        U256::try_from(val).map(Self)
    }

    /// Creates USDT from a raw amount (micro-USDT).
    ///
    /// # Panics
    /// Panics if `val` is negative or doesn't fit into `U256`.
    /// See [`Self::try_from_value`].
    #[expect(
        clippy::expect_used,
        reason = "documented panicking shorthand for try_from_value"
    )]
    pub fn from<T>(val: T) -> Self
    where
        U256: TryFrom<T>,
        <U256 as TryFrom<T>>::Error: std::fmt::Debug,
    {
        Self::try_from_value(val).expect("invalid raw USDT amount")
    }

    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }

    pub fn checked_sub(self, other: Self) -> Option<Self> {
        self.0.checked_sub(other.0).map(Self)
    }

    /// Returns the amount as `f64` (lossy for very large values).
    pub fn to_decimal(&self) -> f64 {
        self.to_decimal_string().parse().unwrap_or(f64::NAN)
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
        $crate::contracts::token::usdt::Usdt::from($val)
    };
}
