use std::{
    fmt::Display,
    iter::Sum,
    ops::{Deref, DerefMut},
};

use crate::{Amount, Currency, CurrencyError, Decimal, Result};

/// `AmountResult` represents the result of a computation involving
/// [amounts](Amount) of money. It can therefore either be an [`Amount`]
/// if the computation was successful, or a [`CurrencyError`] if the
/// computation was not successful.
///
/// Instances of [`AmountResult`] can themselves be used in arithmetic
/// operations, either with other instances of [`AmountResult`] or with
/// instances of [`Amount`]. All compuration errors are coalesced and
/// can/should be checked at the very end of the computation.
///
/// Note that [`AmountResult`] is nothing but a wrapper around a
/// [`std::result::Result<Amount, CurrencyError>`](Result).
///
#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub struct AmountResult(pub(crate) Result<Amount>);

impl AmountResult {
    /// Creates a [`AmountResult`] around a [`CurrencyError::Unknown`]
    ///
    /// # Examples
    ///
    /// ```
    /// use oxydized_money::{AmountResult, CurrencyError};
    ///
    /// assert!(AmountResult::unknown().is_err());
    /// assert_eq!(AmountResult::unknown().unwrap_err(), CurrencyError::Unknown);
    /// ```
    pub fn unknown() -> Self {
        Self::from(CurrencyError::Unknown)
    }

    /// Creates a [`AmountResult`] around a [`CurrencyError::Mismatch`]
    ///
    /// # Examples
    ///
    /// ```
    /// use oxydized_money::{AmountResult, CurrencyError, Currency::{USD,EUR}};
    ///
    /// assert!(AmountResult::mismatch(USD,EUR).is_err());
    /// assert_eq!(AmountResult::mismatch(EUR,USD).unwrap_err(), CurrencyError::Mismatch(EUR,USD));
    /// ```
    pub fn mismatch(c1: Currency, c2: Currency) -> Self {
        Self::from(CurrencyError::Mismatch(c1, c2))
    }

    /// Creates a [`AmountResult`] around a [`CurrencyError::DivideByZero`]
    ///
    /// # Examples
    ///
    /// ```
    /// use oxydized_money::{AmountResult, CurrencyError, Currency::{USD,EUR}};
    ///
    /// assert!(AmountResult::divide_by_zero().is_err());
    /// assert_eq!(AmountResult::divide_by_zero().unwrap_err(), CurrencyError::DivideByZero);
    /// ```
    pub fn divide_by_zero() -> Self {
        Self::from(CurrencyError::DivideByZero)
    }

    /// Returns the absolute value of `self` if it wraps an [`Amount`].
    /// Coalesces the error otherzise.
    ///
    /// # Examples
    ///
    /// ```
    /// use oxydized_money::{Currency::{USD, EUR}, AmountResult, Currency, Decimal};
    /// use oxydized_money_macros::eur;
    ///
    /// assert_eq!(AmountResult::from(eur!(-10)).abs(), AmountResult::from(eur!(10)));
    /// assert_eq!(AmountResult::unknown().abs(), AmountResult::unknown());
    /// assert_eq!(AmountResult::mismatch(USD,EUR).abs(), AmountResult::mismatch(USD,EUR));
    /// ```
    ///
    pub fn abs(&self) -> Self {
        AmountResult(self.map(|amount| amount.abs()))
    }

    /// Returns the value of `self` converted to the target currency if
    /// it wraps an [`Amount`]. Coalesces the error otherzise.
    ///
    /// # Examples
    ///
    /// ```
    /// use oxydized_money::{AmountResult, Decimal, Amount, Currency::{USD, EUR}};
    /// use oxydized_money_macros::{usd,eur,dec};
    ///
    /// let exchange_rate = dec!(0.9);
    /// assert_eq!(
    ///     AmountResult::from(eur!(10)).converted_to(USD, exchange_rate),
    ///     AmountResult::from(usd!(10) * exchange_rate)
    /// );
    /// assert_eq!(
    ///     AmountResult::unknown().converted_to(USD, exchange_rate),
    ///     AmountResult::unknown()
    /// );
    /// assert_eq!(
    ///     AmountResult::mismatch(EUR,USD).converted_to(USD, exchange_rate),
    ///     AmountResult::mismatch(EUR,USD)
    /// );
    /// ```
    ///
    pub fn converted_to(&self, target_currency: Currency, exchange_rate: Decimal) -> Self {
        AmountResult(self.map(|amount| amount.converted_to(target_currency, exchange_rate)))
    }

    /// Extracts the inner part of type [`std::result::Result<Amount, CurrencyError>`].
    ///
    /// This can be useful to use the question mark operator `?` on
    /// the underlying `Result`.
    ///
    /// # Examples
    ///
    /// ```
    /// use oxydized_money::{Amount, AmountResult, Decimal};
    /// use oxydized_money_macros::dec;
    /// use std::result::Result;
    /// use std::error::Error;
    ///
    /// fn add_amounts_and_double(a: Amount, b: Amount) -> Result<Amount, Box<dyn Error>> {
    ///     let intermediate = (a+b).into_inner()?;
    ///     Ok(intermediate * dec!(2))
    /// }
    /// ```
    pub fn into_inner(self) -> Result<Amount> {
        self.0
    }

    /// Returns `true` if and only if self is an error of type
    /// [`CurrencyError::Unknown`].
    ///
    /// # Example
    ///
    /// ```
    /// use oxydized_money::{Amount, AmountResult};
    ///
    /// let amounts: Vec<Amount> = Vec::default();
    /// assert!(amounts.into_iter().sum::<AmountResult>().is_unknown());
    /// ```
    pub fn is_unknown(&self) -> bool {
        matches!(self.0, Err(CurrencyError::Unknown))
    }

    /// Returns `true` if and only if self is an error of type
    /// [`CurrencyError::Mismatch`].
    ///
    /// # Example
    ///
    /// ```
    /// use oxydized_money_macros::{eur, usd};
    /// use oxydized_money::Decimal;
    ///
    /// assert!((eur!(420) + usd!(69)).is_mismatch());
    /// ```
    pub fn is_mismatch(&self) -> bool {
        matches!(self.0, Err(CurrencyError::Mismatch(_, _)))
    }

    /// Returns `true` if and only if self is an error of type
    /// [`CurrencyError::DivideByZero`].
    ///
    /// # Example
    ///
    /// ```
    /// use oxydized_money_macros::{eur, usd, dec};
    /// use oxydized_money::Decimal;
    ///
    /// assert!((eur!(420) / dec!(0)).is_divide_by_zero());
    /// ```
    pub fn is_divide_by_zero(&self) -> bool {
        matches!(self.0, Err(CurrencyError::DivideByZero))
    }
}

impl Display for AmountResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Ok(amount) => amount.fmt(f),
            Err(error) => error.fmt(f),
        }
    }
}

impl Deref for AmountResult {
    type Target = Result<Amount>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AmountResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Amount> for AmountResult {
    fn from(amount: Amount) -> Self {
        AmountResult(Ok(amount))
    }
}

impl From<&Amount> for AmountResult {
    fn from(amount: &Amount) -> Self {
        AmountResult(Ok(*amount))
    }
}

impl From<&mut Amount> for AmountResult {
    fn from(amount: &mut Amount) -> Self {
        AmountResult(Ok(*amount))
    }
}

impl From<&AmountResult> for AmountResult {
    fn from(amount: &AmountResult) -> Self {
        *amount
    }
}

impl From<&mut AmountResult> for AmountResult {
    fn from(amount: &mut AmountResult) -> Self {
        *amount
    }
}

impl From<CurrencyError> for AmountResult {
    fn from(amount: CurrencyError) -> Self {
        AmountResult(Err(amount))
    }
}

impl From<&CurrencyError> for AmountResult {
    fn from(amount: &CurrencyError) -> Self {
        AmountResult(Err(*amount))
    }
}

impl From<&mut CurrencyError> for AmountResult {
    fn from(amount: &mut CurrencyError) -> Self {
        AmountResult(Err(*amount))
    }
}

impl Sum<Amount> for AmountResult {
    fn sum<I: Iterator<Item = Amount>>(mut iter: I) -> Self {
        if let Some(amount) = iter.next() {
            iter.fold(AmountResult::from(amount), |a, b| a + b)
        } else {
            CurrencyError::Unknown.into()
        }
    }
}

impl<'a> Sum<&'a Amount> for AmountResult {
    fn sum<I: Iterator<Item = &'a Amount>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl Sum<AmountResult> for AmountResult {
    fn sum<I: Iterator<Item = AmountResult>>(mut iter: I) -> Self {
        if let Some(amount) = iter.next() {
            iter.fold(amount, |a, b| a + b)
        } else {
            CurrencyError::Unknown.into()
        }
    }
}

impl<'a> Sum<&'a AmountResult> for AmountResult {
    fn sum<I: Iterator<Item = &'a AmountResult>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

#[cfg(test)]
mod test {
    use crate as oxydized_money;
    use oxydized_money::Decimal;
    use oxydized_money::{Amount, AmountResult, Currency::*, CurrencyError::*};
    use oxydized_money_macros::{dec, eur, usd};

    macro_rules! W {
        ($expr:expr) => {
            AmountResult::from($expr)
        };
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", W!(eur!(2))), "€ 2.00");
        assert_eq!(format!("{}", W!(usd!(5.4))), "$ 5.40");
        let amount = (usd!(2) / dec!(3)) + usd!(1);
        assert_eq!("$ 1.66", format!("{}", amount));
        assert_eq!("$ 1.666", format!("{:.3}", amount));

        assert_eq!(format!("{}", W!(Unknown)), "unknown currency");
        assert_eq!(
            format!("{}", W!(Mismatch(USD, EUR))),
            "mismatch currency 'USD' and 'EUR'"
        );
        assert_eq!(
            format!("{}", W!(Mismatch(EUR, USD))),
            "mismatch currency 'EUR' and 'USD'"
        );
    }

    #[test]
    fn test_deref() {
        assert!(W!(Mismatch(EUR, USD)).is_err());
        assert!(W!(Mismatch(USD, EUR)).is_err());
        assert!(W!(Unknown).is_err());
        assert!(W!(eur!(19)).is_ok());
    }

    #[test]
    fn test_deref_mut() {
        #[allow(unused_mut)]
        let mut x = W!(Unknown);
        assert!(x.is_err());
        *x = Ok(eur!(10));
    }

    #[test]
    fn test_from() {
        let res = W!(Amount(dec!(10), EUR));
        let amount = Amount::try_from(res).unwrap();
        assert_eq!(amount.value(), dec!(10));

        let res = W!(Unknown);
        assert_eq!(res.unwrap_err(), Unknown)
    }

    #[test]
    fn test_sum() {
        let sum = [eur!(1), eur!(2)].iter().sum::<AmountResult>();
        assert_eq!(sum, eur!(3));

        let sum = [eur!(1), eur!(2)].into_iter().sum::<AmountResult>();
        assert_eq!(sum, eur!(3));

        let sum = Vec::<Amount>::from([]).iter().sum::<AmountResult>();
        assert_eq!(sum, W!(Unknown));

        let sum = [eur!(2), usd!(3)].iter().sum::<AmountResult>();
        assert_eq!(sum, W!(Mismatch(EUR, USD)));

        let sum = [eur!(1), eur!(2)].into_iter().sum::<AmountResult>();
        assert_eq!(sum, eur!(3));

        let sum = Vec::<AmountResult>::from([]).iter().sum::<AmountResult>();
        assert_eq!(sum, W!(Unknown));

        let sum = [W!(Mismatch(EUR, USD))].iter().sum::<AmountResult>();
        assert_eq!(sum, W!(Mismatch(EUR, USD)));

        let sum = [W!(Unknown)].iter().sum::<AmountResult>();
        assert_eq!(sum, W!(Unknown));

        let sum = [W!(Unknown)].into_iter().sum::<AmountResult>();
        assert_eq!(sum, W!(Unknown));

        let sum = [eur!(1).into(), W!(Unknown), eur!(2).into()]
            .iter()
            .sum::<AmountResult>();
        assert_eq!(sum, eur!(3));

        let sum = [eur!(1).into(), W!(Mismatch(USD, EUR)), eur!(2).into()]
            .iter()
            .sum::<AmountResult>();
        assert_eq!(sum, W!(Mismatch(USD, EUR)));

        let sum = [eur!(2), usd!(3), usd!(4)].iter().sum::<AmountResult>();
        assert_eq!(sum, W!(Mismatch(EUR, USD)));
    }

    #[test]
    fn test_sub() {
        assert_eq!(eur!(2) - eur!(3), eur!(-1));
        assert_eq!(eur!(1) - eur!(2) - eur!(3), eur!(-4));
        assert_eq!((eur!(1) - eur!(2)) - eur!(3), eur!(-4));
        assert_eq!(eur!(1) - (eur!(2) - eur!(3)), eur!(2));
        //
        assert_eq!(eur!(1) - W!(Unknown), eur!(1));
        assert_eq!(W!(Unknown) - eur!(1), eur!(-1));
        assert_eq!(W!(Unknown) - W!(Unknown), W!(Unknown));
        assert_eq!(W!(Unknown) - W!(Mismatch(USD, EUR)), W!(Mismatch(USD, EUR)));
        assert_eq!(W!(Mismatch(USD, EUR)) - W!(Unknown), W!(Mismatch(USD, EUR)));
        assert_eq!(
            W!(Mismatch(USD, EUR)) - W!(Mismatch(EUR, USD)),
            W!(Mismatch(USD, EUR))
        );
        assert_eq!(
            W!(Mismatch(EUR, USD)) - W!(Mismatch(USD, EUR)),
            W!(Mismatch(EUR, USD))
        );
    }

    #[test]
    fn test_sub_assign() {
        let mut accum = W!(eur!(2));
        accum -= eur!(8);
        assert_eq!(accum, eur!(-6));
        accum -= usd!(1);
        assert_eq!(accum, W!(Mismatch(EUR, USD)));
    }
}
