#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
#![warn(
    missing_docs,
    rustdoc::missing_crate_level_docs,
    rustdoc::broken_intra_doc_links
)]

mod amount;
mod error;
mod ops;
mod result;

pub use amount::Amount;
pub use error::{CurrencyError, Result};
pub use iso_currency::Currency;
pub use result::AmountResult;
pub use rust_decimal::Decimal;
