#![allow(clippy::result_large_err)]
#![feature(error_generic_member_access)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(impl_trait_in_assoc_type)]

use crate::domain::address::TronAddress;

#[allow(warnings)]
pub mod protocol;

pub mod client;
pub mod contracts;
pub mod domain;
pub mod error;
pub mod extractor;
pub mod listener;
pub mod providers;
pub mod signer;

type Result<T> = std::result::Result<T, error::Error>;

/// Trait to filter by some criteria
#[async_trait::async_trait]
pub trait Filter<T> {
    async fn filter(&self, by: T) -> bool;
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{e}\n")?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{cause}")?;
        current = cause.source();
    }
    Ok(())
}

#[macro_export]
macro_rules! impl_debug {
    ($type:ident) => {
        use $crate::error_chain_fmt;
        impl std::fmt::Debug for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                error_chain_fmt(self, f)
            }
        }
    };
}
