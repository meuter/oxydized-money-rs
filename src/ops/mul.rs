use crate::{Amount, AmountResult, Decimal};
use std::ops::Mul;

impl Mul<Decimal> for Amount {
    type Output = Amount;

    fn mul(self, rhs: Decimal) -> Self::Output {
        Amount::new(self.value() * rhs, self.currency())
    }
}

impl Mul<Decimal> for AmountResult {
    type Output = AmountResult;

    fn mul(self, rhs: Decimal) -> Self::Output {
        match self.0 {
            Ok(amount) => (amount * rhs).into(),
            Err(error) => error.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate as oxydized_money;
    use oxydized_money::Decimal;
    use oxydized_money::{Currency::*, CurrencyError::*};
    use oxydized_money_macros::{dec, eur};

    #[test]
    #[allow(clippy::op_ref)]
    fn amount_mul_decimal() {
        assert_eq!(eur!(2) * dec!(3), eur!(6));
        assert_eq!(eur!(-2) * dec!(3), eur!(-6));
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn amount_result_mul_decimal() {
        assert_eq!(W!(eur!(2)) * dec!(3), eur!(6));
        assert_eq!(W!(Mismatch(USD, EUR)) * dec!(3), W!(Mismatch(USD, EUR)));
        assert_eq!(W!(Unknown) * dec!(3), W!(Unknown));
        assert_eq!(W!(DivideByZero) * dec!(3), W!(DivideByZero));
    }
}
