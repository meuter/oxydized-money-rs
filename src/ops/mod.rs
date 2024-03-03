#[cfg(test)]
macro_rules! W {
    ($expr:expr) => {
        $crate::AmountResult::from($expr)
    };
}

mod add;
mod div;
mod eq;
mod mul;
mod neg;
mod sub;
