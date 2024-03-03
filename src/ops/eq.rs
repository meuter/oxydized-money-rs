use crate::{Amount, AmountResult, CurrencyError};

impl PartialEq<AmountResult> for Amount {
    fn eq(&self, other: &AmountResult) -> bool {
        match other.0 {
            Ok(amount) => *self == amount,
            Err(_) => false,
        }
    }
}

impl PartialEq<Amount> for AmountResult {
    fn eq(&self, other: &Amount) -> bool {
        match self.0 {
            Ok(amount) => amount == *other,
            Err(_) => false,
        }
    }
}

impl PartialEq<AmountResult> for CurrencyError {
    fn eq(&self, other: &AmountResult) -> bool {
        match other.0 {
            Ok(_) => false,
            Err(error) => *self == error,
        }
    }
}

impl PartialEq<CurrencyError> for AmountResult {
    fn eq(&self, other: &CurrencyError) -> bool {
        match self.0 {
            Ok(_) => false,
            Err(error) => error == *other,
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
    #[allow(clippy::op_ref)]
    fn amount_eq_amount_resut() {
        assert!(eur!(10) == W!(eur!(10)));
        assert!(eur!(10) != W!(eur!(12)));
        assert!(eur!(10) != W!(usd!(10)));
        assert!(eur!(10) != W!(usd!(12)));
        assert!(eur!(10) != W!(Unknown));
        assert!(eur!(10) != W!(DivideByZero));
        assert!(eur!(10) != W!(Mismatch(EUR, USD)));
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn amount_result_eq_amount() {
        assert!(W!(eur!(10)) == eur!(10));
        assert!(W!(eur!(10)) != eur!(12));
        assert!(W!(eur!(10)) != usd!(10));
        assert!(W!(eur!(10)) != usd!(12));
        assert!(W!(Unknown) != eur!(10));
        assert!(W!(DivideByZero) != eur!(10));
        assert!(W!(Mismatch(EUR, USD)) != eur!(10));
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn amount_result_eq_error() {
        assert!(W!(eur!(10)) != Unknown);
        assert!(W!(eur!(10)) != DivideByZero);
        assert!(W!(eur!(10)) != Mismatch(EUR, USD));
        assert!(W!(Unknown) == Unknown);
        assert!(W!(Unknown) != DivideByZero);
        assert!(W!(Unknown) != Mismatch(EUR, USD));
        assert!(W!(DivideByZero) != Unknown);
        assert!(W!(DivideByZero) == DivideByZero);
        assert!(W!(DivideByZero) != Mismatch(EUR, USD));
        assert!(W!(Mismatch(EUR, USD)) != Unknown);
        assert!(W!(Mismatch(EUR, USD)) != DivideByZero);
        assert!(W!(Mismatch(EUR, USD)) == Mismatch(EUR, USD));
        assert!(W!(Mismatch(EUR, USD)) != Mismatch(USD, EUR));
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn error_eq_amount_result() {
        assert!(Unknown != W!(eur!(10)));
        assert!(DivideByZero != W!(eur!(10)));
        assert!(Mismatch(EUR, USD) != W!(eur!(10)));
        assert!(Unknown == W!(Unknown));
        assert!(DivideByZero != W!(Unknown));
        assert!(Mismatch(EUR, USD) != W!(Unknown));
        assert!(Unknown != W!(DivideByZero));
        assert!(DivideByZero == W!(DivideByZero));
        assert!(Mismatch(EUR, USD) != W!(DivideByZero));
        assert!(Unknown != W!(Mismatch(EUR, USD)));
        assert!(DivideByZero != W!(Mismatch(EUR, USD)));
        assert!(Mismatch(EUR, USD) == W!(Mismatch(EUR, USD)));
        assert!(Mismatch(USD, EUR) != W!(Mismatch(EUR, USD)));
    }
}
