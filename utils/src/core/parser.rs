use serde_json::Value;
use sqlx::types::BigDecimal;
use uuid::Uuid;

#[allow(dead_code)]
pub fn value_to_i64(value: &Value) -> i64 {
    let data = serde_json::to_string(value)
        .expect("Failed to convert json in string on parse to i64 function")
        .replace('\"', "");
    match data.is_empty() {
        true => 0,
        false => data.parse::<i64>().unwrap_or(0),
    }
}

#[allow(dead_code)]
pub fn value_to_i32(value: &Value) -> i32 {
    let data = serde_json::to_string(value)
        .expect("Failed to convert json in string on parse to i64 function")
        .replace('\"', "");
    match data.is_empty() {
        true => 0,
        false => data.parse::<i32>().unwrap_or(0),
    }
}

#[allow(dead_code)]
pub fn value_to_f64(value: &Value) -> f64 {
    let data = serde_json::to_string(value)
        .expect("Failed to convert json in string on parse to i64 function")
        .replace('\"', "");
    match data.is_empty() {
        true => 0.0,
        false => data.parse::<f64>().unwrap_or(0.0),
    }
}

#[allow(dead_code)]
pub fn value_to_string(value: &Value) -> String {
    let data = serde_json::to_string(value)
        .expect("Failed to convert json in string on parse to i64 function")
        .replace('\"', "");
    match data.is_empty() {
        true => String::new(),
        false => data.parse::<String>().unwrap_or(String::new()),
    }
}

#[allow(dead_code)]
pub fn string_to_i32(value: &Value) -> i32 {
    let data = serde_json::to_string(value)
        .expect("Failed to convert string in i32")
        .replace('\"', "");
    match data.is_empty() {
        true => 0,
        false => data.parse::<i32>().unwrap_or(0),
    }
}

#[allow(dead_code)]
pub fn string_to_uuid(uuid: String) -> Result<Uuid, String> {
    match uuid.len() {
        36 => match Uuid::parse_str(uuid.as_str()) {
            Ok(uuid) => Ok(uuid),
            Err(_) => Err("Invalid uuid".to_owned()),
        },
        32 => match Uuid::parse_str(uuid.as_str()) {
            Ok(uuid) => Ok(uuid),
            Err(_) => Err("Invalid uuid".to_owned()),
        },
        _ => Err("Invalid uuid".to_owned()),
    }
}

#[allow(dead_code)]
pub fn f64_to_bigdecimal(value: &f64) -> BigDecimal {
    let data = serde_json::to_string(value)
        .expect("Failed to convert json in string on parse to f64 function")
        .replace('\"', "");
    match data.is_empty() {
        true => BigDecimal::from(0),
        false => data.parse::<BigDecimal>().unwrap_or(BigDecimal::from(0)),
    }
}

#[allow(dead_code)]
pub fn type_of<T>(_: &T) -> String {
    let result = std::any::type_name::<T>();
    result.to_string()
}
