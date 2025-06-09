use crate::{Amount, AmountResult, CurrencyError::DivideByZero, Decimal};
use std::ops::Div;

impl Div<Decimal> for Amount {
    type Output = AmountResult;

    fn div(self, rhs: Decimal) -> Self::Output {
        if rhs.is_zero() {
            DivideByZero.into()
        } else {
            Amount::new(self.value() / rhs, self.currency()).into()
        }
    }
}

impl Div<Decimal> for AmountResult {
    type Output = AmountResult;

    fn div(self, rhs: Decimal) -> Self::Output {
        match self.0 {
            Ok(amount) => amount / rhs,
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
    fn amount_div_decimal() {
        assert_eq!(eur!(6.3) / dec!(3), eur!(2.1));
        assert_eq!(eur!(-6.3) / dec!(3), eur!(-2.1));
        assert_eq!(eur!(6.3) / dec!(-3), eur!(-2.1));
        assert_eq!(eur!(6.3) / dec!(0), W!(DivideByZero));
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn amount_result_div_decimal() {
        assert_eq!(W!(eur!(10)) / dec!(5), eur!(2));
        assert_eq!(W!(eur!(10)) / dec!(0), W!(DivideByZero));
        assert_eq!(W!(Mismatch(USD, EUR)) / dec!(3), W!(Mismatch(USD, EUR)));
        assert_eq!(W!(Unknown) / dec!(3), W!(Unknown));
        assert_eq!(W!(DivideByZero) / dec!(3), W!(DivideByZero));
    }
}
