#![allow(clippy::result_large_err)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
// #![feature(error_generic_member_access)]

#[allow(warnings)]
pub(crate) mod protocol;

pub mod client;
pub mod contracts;
pub mod domain;
pub mod error;
pub mod extractor;
pub mod listener;
pub mod provider;
pub mod signer;
pub(crate) mod utility;

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

#[macro_export]
macro_rules! impl_enum_conversions {
    ($from_enum:path => $to_enum:path {
        $($variant:ident),* $(,)?
    }) => {
        impl From<$from_enum> for $to_enum {
            fn from(value: $from_enum) -> Self {
                match value {
                    $( <$from_enum>::$variant => <$to_enum>::$variant, )*
                }
            }
        }

        impl From<$to_enum> for $from_enum {
            fn from(value: $to_enum) -> Self {
                match value {
                    $( <$to_enum>::$variant => <$from_enum>::$variant, )*
                }
            }
        }
    };
}
