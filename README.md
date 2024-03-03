# Oxydized Money 💵 ↔ 💶

This crate aims at providing data types to manipulate amounts 
of money in specific currencies, convert amounts between currencies 
and make sure that any computation is performed on amounts of the same
currency.

# Motivations

This crate was introduced because all the existing known alternatives 
have some signigicant drawbacks:

- using native floating point type like `f32` or `f64` suffer from their lack 
  of precision.
- using [`rust_decimal::Decimal`](https://crates.io/crates/rust_decimal) solves
  this issue but does not prevent from adding amounts in different currency.
- using [`rusty_money::Money`](https://crates.io/crates/rusty-money), although slightly
  better, does not really solve the conversion issue because performing arithmetic
  operations on amounts of different currencies panics. 

Rust being dedicated to proper error handling, all these options feel like 
compromises. This crate aims to improve this by by providing three distinct 
data types:

- `Amount` for storing amounts in a given currency. 
- `CurrencyError` for representing any errors (currency mismatch, ...) during 
  arithmetic operations on `Amount`s.
- `AmountResult` for storing the result of arithmetic operations 
  (either an `Amount` or `CurrencyError`).

Arithmetic operations are defined in such a way that these three types
inte-roperate almost seemlessly. However, when performing an operation, 
the type of output always reflect whether an error could have occured.
Operation that cannot fail will output `Amount`s and operations that 
can fail will outout `AmountResult`s. Before getting at the underlying
`Amount`, `AmountResult`s need to be properly checked for errors.

No more 🦶-guns

# Examples

```rust
use oxydized_money_macros::{eur, usd, dec};
use oxydized_money::{
    Currency::{EUR,USD},
    CurrencyError,
    Decimal,
};

// Amount(USD)
let capital = usd!(10_000);

// Decinal
let exchange_rate = dec!(0.928);

// Amount(EUR)
let converted = capital.converted_to(EUR, exchange_rate);

// Amount(EUR)
let fees = eur!(15.2);

// Amount(EUR) + Amount(EUR) => AmountResult(EUR)
let subtotal = converted + fees;

// Amount(EUR) * Decimal => Amount(EUR)
let extras = eur!(50) * dec!(2);

// AmountResult(EUR) + Amount(EUR) => AmountResult(EUR)
let total = subtotal + extras;

// Comparing AmountResult with Amounts
assert_eq!(total, eur!(9_395.200));

// AmountResult(EUR) + Amount(USD) => AmountResult(Mismatch(EUR,USD))
let oops = total + usd!(20);

// Comparing AmountResult with CurrencyError
assert_eq!(oops, CurrencyError::Mismatch(EUR,USD));

// AmountResult(Mismatch(EUR,USD)) + Amount(USD) => AmountResult(Mismatch(EUR,USD))
let oh_my = oops + usd!(200);
assert_eq!(oh_my, CurrencyError::Mismatch(EUR,USD));

// "Everything, everywhere, all at once."
assert_eq!(
    usd!(10_000).converted_to(EUR, dec!(0.928)) + eur!(15.2) + eur!(50)*dec!(2),
    eur!(9_395.200)
);
```



# Supported Operations 

## Binary Operations

### `Amount`

| Left Operand    | Operator             | Right Operand   |     Output     |
|:----------------|:--------------------:|:----------------|:---------------|
| `Amount`        | `*`                  | `Decimal`       | `Amount`       |
| `Amount`        | `/`                  | `Decimal`       | `AmountResult` |
| `Amount`        | {`+`,`-`}            | `Amount`        | `AmountResult` |
| `Amount`        | {`+`,`-`}            | `AmountResult`  | `AmountResult` |
| `Amount`        | {`==`,`!=`}          | `Amount`        | `bool`         |
| `Amount`        | {`==`,`!=`}          | `AmountResult`  | `bool`         |
| `Amount`        | {`<`,`>`,`>=`,`<=` } | `Amount`        | `bool`         |

### `AmountResult`

| Left Operand    | Operator             | Right Operand   |     Output     |
|:----------------|:--------------------:|:----------------|:---------------|
| `AmountResult`  | `*`                  | `Decimal`       | `AmountResult` |
| `AmountResult`  | `/`                  | `Decimal`       | `AmountResult` |
| `AmountResult`  | {`+`,`-`}            | `Amount`        | `AmountResult` |
| `AmountResult`  | {`+`,`-`}            | `AmountResult`  | `AmountResult` |
| `AmountResult`  | {`==`,`!=`}          | `Amount`        | `bool`         |
| `AmountResult`  | {`==`,`!=`}          | `AmountResult`  | `bool`         |
| `AmountResult`  | {`==`,`!=`}          | `CurrencyError` | `bool`         |

### `CurrencyError` 

| Left Operand    | Operator             | Right Operand   |     Output     |
|:----------------|:--------------------:|:----------------|:---------------|
| `CurrencyError` | {`==`,`!=`}          | `AmountResult`  | `bool`         |
| `CurrencyError` | {`==`,`!=`}          | `CurrencyError` | `bool`         |


## Unary Operations

### `Amount`

| Operator  |     Operand     |     Output     |
|:---------:|:---------------:|:--------------:|
| `-`       | `Amount`        | `Amount`       |


### `AmountResult`

| Operator  |     Operand     |     Output     |
|:---------:|:---------------:|:--------------:|
| `-`       | `AmountResult`  | `AmountResult` |

