use iso_currency::{Currency, IntoEnumIterator};
use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

fn generate_currency_macro(file: &mut impl Write, currency: Currency) {
    let mut code_lower = currency.code().to_lowercase();
    let code_upper = code_lower.to_uppercase();
    let name = currency.name();

    if code_lower == "try" {
        code_lower = "r#try".into()
    }

    writeln!(
        file,
        r#"
        /// Convenience macro to construct amounts of money in "{name}".
        #[macro_export]
        macro_rules! {code_lower} {{
            ($amount:expr) => {{
                oxydized_money::Amount::new(rust_decimal_macros::dec!($amount), oxydized_money::Currency::{code_upper})
            }};
        }}
    "#
    )
    .unwrap();
}

fn main() {
    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("currency_macros.rs");
    let mut file = BufWriter::new(File::create(out_path).unwrap());

    for currency in Currency::iter() {
        generate_currency_macro(&mut file, currency);
    }

    println!("cargo:rerun-if-changed=build.rs");
}
