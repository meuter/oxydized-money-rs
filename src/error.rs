#[cfg(feature = "with_serde")]
use serde::{Deserialize, Serialize};

use crate::Currency;
use std::{error::Error, fmt::Display};

/// `CurrencyError` represents all currency error that can occur during
/// arithmetic operations with [`Amount`](crate::Amount) or
/// [`AmounrResult`](crate::AmountResult).
///
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
pub enum CurrencyError {
    /// Error that occurs if one tries to perform arithmetic operations
    /// on amounts from differenc currencies.
    Mismatch(Currency, Currency),

    /// Error that occurs if one tries to divide an [`Amount`](crate::Amount) or
    /// [`AmounrResult`](crate::AmountResult)  by zero.
    DivideByZero,

    /// Error that occurs if one tries to perform a [`sum`](std::iter::Sum)
    /// on an empty collection of [`Amount`](crate::Amount)s.
    Unknown,
}

impl Error for CurrencyError {}

impl From<&CurrencyError> for CurrencyError {
    fn from(value: &CurrencyError) -> Self {
        *value
    }
}

impl From<&mut CurrencyError> for CurrencyError {
    fn from(value: &mut CurrencyError) -> Self {
        *value
    }
}

/// Type alias for a [`Result`] where the error is [`CurrencyError`]
pub type Result<T> = std::result::Result<T, CurrencyError>;

impl Display for CurrencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CurrencyError::*;
        match self {
            Mismatch(c1, c2) => write!(f, "mismatch currency '{}' and '{}'", c1.code(), c2.code()),
            DivideByZero => write!(f, "divide by zero"),
            Unknown => write!(f, "unknown currency"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        use Currency::{EUR, USD};
        use CurrencyError::*;

        assert_eq!(format!("{}", Unknown), "unknown currency");
        assert_eq!(format!("{}", DivideByZero), "divide by zero");
        assert_eq!(
            format!("{}", Mismatch(USD, EUR)),
            "mismatch currency 'USD' and 'EUR'"
        );
        assert_eq!(
            format!("{}", Mismatch(EUR, USD)),
            "mismatch currency 'EUR' and 'USD'"
        );
    }

    #[cfg(feature = "with_serde")]
    #[test]
    fn test_serde() {
        use serde_json::json;
        use Currency::{EUR, USD};
        use CurrencyError::*;

        assert_eq!(serde_json::to_value(Unknown).unwrap(), json!("Unknown"));
        assert_eq!(
            serde_json::to_value(DivideByZero).unwrap(),
            json!("DivideByZero")
        );
        assert_eq!(
            serde_json::to_value(Mismatch(EUR, USD)).unwrap(),
            json!({"Mismatch": ["EUR", "USD"]})
        );

        assert_eq!(
            serde_json::from_value::<CurrencyError>(json!("Unknown")).unwrap(),
            Unknown
        );
        assert_eq!(
            serde_json::from_value::<CurrencyError>(json!("DivideByZero")).unwrap(),
            DivideByZero
        );
        assert_eq!(
            serde_json::from_value::<CurrencyError>(json!({"Mismatch": ["USD", "EUR"]})).unwrap(),
            Mismatch(USD, EUR)
        );
    }
}
