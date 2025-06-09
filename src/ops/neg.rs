use crate::{Amount, AmountResult};
use std::ops::Neg;

impl Neg for Amount {
    type Output = Amount;

    fn neg(self) -> Self::Output {
        Amount::new(-self.value(), self.currency())
    }
}

impl Neg for AmountResult {
    type Output = AmountResult;
    fn neg(self) -> Self::Output {
        match self.0 {
            Ok(amount) => (-amount).into(),
            Err(error) => error.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate as oxydized_money;
    use oxydized_money::Decimal;
    use oxydized_money::{Currency::*, CurrencyError::*};
    use oxydized_money_macros::{eur, usd};

    #[test]
    fn neg_amount() {
        assert_eq!(-eur!(2), eur!(-2));
        assert_eq!(-(-eur!(2)), eur!(2));
        assert_eq!(-usd!(-2), usd!(2));
    }

    #[test]
    fn neg_amount_result() {
        assert_eq!(-W!(eur!(2)), -eur!(2));
        assert_eq!(-W!(Mismatch(EUR, USD)), W!(Mismatch(EUR, USD)));
        assert_eq!(-W!(Unknown), W!(Unknown));
        assert_eq!(-W!(DivideByZero), W!(DivideByZero));
    }
}
