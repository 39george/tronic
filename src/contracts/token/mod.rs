use std::{collections::HashMap, fmt, sync::Arc};

use alloy_primitives::U256;

use crate::domain::address::TronAddress;

pub mod usdt;

#[derive(Debug, thiserror::Error)]
pub enum TokenError {
    #[error("Invalid amount")]
    InvalidAmount,
    #[error("Precision overflow")]
    PrecisionOverflow,
}

pub trait Token:
    Sized
    + fmt::Display
    + fmt::Debug
    + Into<U256>
    + From<U256>
    + PartialEq
    + PartialOrd
    + Clone
    + Copy
{
    fn symbol() -> &'static str;
    fn decimals() -> u32;
}

pub trait TokenRegistry {
    fn resolve_token(&self, address: &TronAddress) -> Option<TokenKind>;
}

#[derive(Clone, Debug)]
pub enum TokenKind {
    Usdt,
    Usdc,
    Other(String),
}

#[derive(Default, Clone)]
pub struct InMemoryTokenRegistry {
    pub map: Arc<HashMap<TronAddress, TokenKind>>,
}

impl From<HashMap<TronAddress, TokenKind>> for InMemoryTokenRegistry {
    fn from(value: HashMap<TronAddress, TokenKind>) -> Self {
        InMemoryTokenRegistry {
            map: Arc::new(value),
        }
    }
}

impl TokenRegistry for InMemoryTokenRegistry {
    fn resolve_token(&self, address: &TronAddress) -> Option<TokenKind> {
        self.map.get(address).cloned()
    }
}
