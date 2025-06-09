use std::ops::{Add, AddAssign};

use crate::{Amount, AmountResult, CurrencyError::*};

impl Add<Amount> for Amount {
    type Output = AmountResult;

    fn add(self, rhs: Amount) -> Self::Output {
        if self.currency() == rhs.currency() {
            Amount::new(self.value() + rhs.value(), self.currency()).into()
        } else {
            Mismatch(self.currency(), rhs.currency()).into()
        }
    }
}

impl Add<AmountResult> for Amount {
    type Output = AmountResult;

    fn add(self, rhs: AmountResult) -> Self::Output {
        match rhs.0 {
            Ok(amount) => self + amount,
            Err(Unknown) => self.into(),
            Err(error) => error.into(),
        }
    }
}

impl Add<AmountResult> for AmountResult {
    type Output = AmountResult;

    fn add(self, rhs: AmountResult) -> Self::Output {
        match self.0 {
            Ok(amount) => amount + rhs,
            Err(Unknown) => rhs,
            Err(error) => error.into(),
        }
    }
}

impl Add<Amount> for AmountResult {
    type Output = AmountResult;

    fn add(self, rhs: Amount) -> Self::Output {
        match self.0 {
            Ok(amount) => amount + rhs,
            Err(Unknown) => rhs.into(),
            Err(error) => error.into(),
        }
    }
}

impl AddAssign<Amount> for AmountResult {
    fn add_assign(&mut self, rhs: Amount) {
        *self = *self + rhs
    }
}

impl AddAssign<AmountResult> for AmountResult {
    fn add_assign(&mut self, rhs: AmountResult) {
        *self = *self + rhs
    }
}

#[cfg(test)]
mod test {
    use crate as oxydized_money;
    use oxydized_money::Decimal;
    use oxydized_money::{Currency::*, CurrencyError::*};
    use oxydized_money_macros::{eur, usd};

    #[test]
    #[allow(clippy::op_ref)]
    fn amount_add_amount() {
        assert_eq!(eur!(3) + eur!(5), eur!(8));
        assert_eq!(eur!(3) + usd!(5), Mismatch(EUR, USD));
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn amount_add_amount_result() {
        assert_eq!(eur!(3) + W!(eur!(1)), eur!(4));
        assert_eq!(eur!(3) + W!(Unknown), eur!(3));
        assert_eq!(eur!(3) + W!(DivideByZero), W!(DivideByZero));
        assert_eq!(eur!(3) + W!(Mismatch(EUR, USD)), W!(Mismatch(EUR, USD)));
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn amount_result_add_amount() {
        assert_eq!(W!(eur!(1)) + eur!(3), eur!(4));
        assert_eq!(W!(Unknown) + eur!(1), eur!(1));
        assert_eq!(W!(DivideByZero) + eur!(1), W!(DivideByZero));
        assert_eq!(W!(Mismatch(EUR, USD)) + eur!(1), W!(Mismatch(EUR, USD)));
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn amount_result_add_amount_result() {
        assert_eq!(W!(eur!(3)) + W!(eur!(1)), eur!(4));
        assert_eq!(W!(eur!(3)) + W!(Unknown), eur!(3));
        assert_eq!(W!(eur!(3)) + W!(DivideByZero), W!(DivideByZero));
        assert_eq!(W!(eur!(3)) + W!(Mismatch(EUR, USD)), W!(Mismatch(EUR, USD)));
    }

    #[test]
    fn amount_result_add_assign_amount() {
        let mut accum = W!(eur!(2));
        accum += eur!(12);
        assert_eq!(accum, eur!(14));
        accum += usd!(1);
        assert_eq!(accum, W!(Mismatch(EUR, USD)));
    }

    #[test]
    fn amount_result_add_assign_amount_result() {
        let mut accum = W!(eur!(2));
        accum += W!(eur!(12));
        assert_eq!(accum, eur!(14));
        accum += W!(usd!(1));
        assert_eq!(accum, W!(Mismatch(EUR, USD)));
    }
}
