use serde_json::Value;
pub fn parse_value_to_i64(string: &Value) -> i64 {
    let data = serde_json::to_string(string)
        .expect("msg")
        .replace("\"", "");
    let result = match data.parse::<i64>() {
        Ok(n) => n,
        Err(_) => {
            println!("Failed to parse integer {}", string);
            0
        }
    };
    return result;
}
#[allow(dead_code)]
pub fn parse_value_to_i32(string: &Value) -> i32 {
    let data = serde_json::to_string(string)
        .expect("msg")
        .replace("\"", "");
    println!("{}", &data);
    let result = match data.parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            println!("Failed to parse integer");
            0
        }
    };
    return result;
}

pub fn parse_value_to_f64(string: &Value) -> f64 {
    let data = serde_json::to_string(string)
        .expect("msg")
        .replace("\"", "");
    let result = match data.parse::<f64>() {
        Ok(n) => n,
        Err(_) => {
            println!("Failed to parse float");
            0.0
        }
    };
    return result;
}

pub fn parse_value_to_string(string: &Value) -> String {
    let result = match serde_json::to_string(string) {
        Ok(n) => n.replace("\"", ""),
        Err(_) => {
            println!("Failed to parse string");
            return "".to_string();
        }
    };
    return result;
}
