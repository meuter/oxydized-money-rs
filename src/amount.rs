use crate::{AmountResult, Currency, CurrencyError, Decimal, Result};
use std::{cmp::Ordering, fmt::Display};

/// `Amount` represents an amount of money in a specific currency.
/// The quantity part is stored as a 128-bit fixed precision [`Decimal`].
/// The currency part is stored as a [`Currency`].
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct Amount(pub Decimal, pub Currency);

impl Amount {
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
        self.0
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
        self.1
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
        Amount(self.value().abs(), self.currency())
    }

    /// Returns `true` if and only if the quantity is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use oxydized_money_macros::{eur};
    /// use oxydized_money::Decimal;
    ///
    /// assert!( ! eur!(10).is_zero());
    /// assert!( ! eur!(-10).is_zero());
    /// assert!( eur!(0).is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        self.value().is_zero()
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
        Amount(self.value() * exchange_rate, target_currency)
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
}
