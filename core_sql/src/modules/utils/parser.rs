// use std::str::FromStr;

use sqlx::types::BigDecimal;

#[allow(dead_code)]
pub fn parse_value_to_bigdecimal(string: &f64) -> BigDecimal {
    // let ipi_vbc_bigdecimal = BigDecimal::from_str().unwrap();
    let data = serde_json::to_string(string)
        .expect("Failed to convert json in string on parse to f64 function")
        .replace('\"', "");
    match data.parse::<BigDecimal>() {
        Ok(n) => n,
        Err(_) => {
            println!("Failed to parse BigDecimal");
            BigDecimal::from(0)
        }
    }
}
