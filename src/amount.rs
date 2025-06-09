#[cfg(feature = "with_serde")]
use serde::{Deserialize, Serialize};

use crate::{AmountResult, Currency, CurrencyError, Decimal, Result};
use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Deref, DerefMut},
};

/// `Amount` represents an amount of money in a specific currency.
/// The quantity part is stored as a 128-bit fixed precision [`Decimal`].
/// The currency part is stored as a [`Currency`].
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
pub struct Amount {
    value: Decimal,
    currency: Currency,
}

impl Amount {
    /// Creates a new amount
    ///
    /// # Arguments
    ///
    /// * `value` - the quantity of money
    /// * `currency` - currency in which [`value`](Amount::value) is measured.
    pub fn new(value: Decimal, currency: Currency) -> Self {
        Self { value, currency }
    }

    /// Returns the quantity of money.
    ///
    /// # Examples
    ///
    /// ```
    /// use oxydized_money_macros::{dec, eur};
    /// use oxydized_money::{Decimal,Amount};
    ///
    /// assert_eq!(eur!(10.5).value(), dec!(10.5))
    /// ```
    pub fn value(&self) -> Decimal {
        self.value
    }

    /// Returns the currency in which [`value`](Amount::value) is measured.
    ///
    /// # Examples
    ///
    /// ```
    /// use oxydized_money::{Currency::USD, Decimal};
    /// use oxydized_money_macros::usd;
    ///
    /// assert_eq!(usd!(10.5).currency(), USD)
    /// ```
    pub fn currency(&self) -> Currency {
        self.currency
    }

    /// Returns the absolute value of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use oxydized_money_macros::{usd, eur};
    /// use oxydized_money::{Decimal,Amount};
    ///
    /// assert_eq!(usd!(-10.5).abs(), usd!(10.5));
    /// assert_eq!(eur!(10.6).abs(), eur!(10.6));
    /// ```
    ///
    pub fn abs(&self) -> Self {
        Amount::new(self.value().abs(), self.currency())
    }

    /// Returns `self` converted in another currency using the provided
    /// exchange rate.
    ///
    /// # Arguments
    ///
    /// * `exchange_rate` - the exchange rate to be used during the conversion
    /// * `target_currency` - the the resulting currency.
    ///
    /// # Examples
    ///
    /// ```
    /// use oxydized_money::{Currency::USD, Decimal};
    /// use oxydized_money_macros::{usd, eur, dec};
    ///
    /// let exchange_rate = dec!(0.9);
    /// assert_eq!(eur!(10.5).converted_to(USD, exchange_rate), usd!(10.5) * exchange_rate);
    ///
    pub fn converted_to(&self, target_currency: Currency, exchange_rate: Decimal) -> Self {
        Amount::new(self.value() * exchange_rate, target_currency)
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(2);
        write!(
            f,
            "{} {:.*}",
            self.currency().symbol(),
            precision,
            self.value()
        )
    }
}

impl TryFrom<AmountResult> for Amount {
    type Error = CurrencyError;

    fn try_from(res: AmountResult) -> Result<Self> {
        if res.is_ok() {
            Ok(res.unwrap())
        } else {
            Err(res.unwrap_err())
        }
    }
}

impl PartialOrd<Amount> for Amount {
    fn partial_cmp(&self, other: &Amount) -> Option<Ordering> {
        if self.currency() == other.currency() {
            self.value().partial_cmp(&other.value())
        } else {
            None
        }
    }
}

impl Deref for Amount {
    type Target = Decimal;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for Amount {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[cfg(test)]
mod test {
    use crate as oxydized_money;
    use assert_matches::assert_matches;
    use oxydized_money::Decimal;
    use oxydized_money::{Currency::*, CurrencyError::*};
    use oxydized_money_macros::{dec, eur, usd};
    use std::cmp::Ordering::*;

    #[test]
    fn test_precision() {
        assert_eq!(eur!(1.2) * dec!(1), eur!(1.2));
        assert_eq!(eur!(1.2) * -dec!(1), eur!(-1.2));
        assert_eq!(eur!(1.2) * -dec!(1), -eur!(1.2));
        assert_eq!(eur!(1.2) * -dec!(1) * dec!(10), -eur!(12));
    }

    #[test]
    fn test_display() {
        assert_eq!("€ 2.00", format!("{}", eur!(2)));
        assert_eq!("$ 5.40", format!("{}", usd!(5.4)));
        let amount = ((usd!(2) / dec!(3)) + usd!(1)).unwrap();
        assert_eq!("$ 1.66", format!("{}", amount));
        assert_eq!("$ 1.666", format!("{:.3}", amount));
    }

    #[test]
    fn test_sub() {
        assert_eq!(eur!(3) - eur!(5), (-eur!(2)));
        assert_eq!(eur!(3) - usd!(5), Mismatch(EUR, USD));
        assert_eq!(eur!(10) - eur!(5), eur!(5));
    }

    #[test]
    fn amount_ord_amount() {
        assert_matches!(eur!(1).partial_cmp(&eur!(1)), Some(Equal));
        assert_matches!(eur!(1).partial_cmp(&eur!(2)), Some(Less));
        assert_matches!(eur!(3).partial_cmp(&eur!(2)), Some(Greater));
        assert_matches!(eur!(1).partial_cmp(&usd!(1)), None);
        assert_matches!(eur!(1).partial_cmp(&usd!(2)), None);
        assert_matches!(eur!(3).partial_cmp(&usd!(2)), None);
    }

    #[test]
    fn test_as_ref_decimal() {
        assert!(eur!(-1).is_sign_negative());
        assert!(eur!(0).is_zero());
    }

    #[cfg(feature = "with_serde")]
    #[test]
    fn test_serde() {
        use serde_json::json;
        assert_eq!(
            serde_json::to_value(eur!(1)).unwrap(),
            json!({ "value": "1", "currency" :"EUR"})
        );

        assert_eq!(
            serde_json::from_value::<crate::Amount>(json!({ "value": "1", "currency" :"EUR"}))
                .unwrap(),
            eur!(1)
        )
    }
}
