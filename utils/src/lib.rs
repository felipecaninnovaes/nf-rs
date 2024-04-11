pub mod core;

#[cfg(test)]
mod fs {
    use crate::core::fs::Dir;

    #[test]
    fn read_dir() {
        let teste: Dir = Dir::read_dir("tests", ".txt");
        assert_eq!(
            teste.dir_files.unwrap(),
            vec!["tests/a.txt", "tests/b.txt", "tests/c.txt"]
        );
    }
}

#[cfg(test)]
mod parser {
    use crate::core::parser;

    #[test]
    fn value_to_i64() {
        let value = serde_json::Value::String("1".to_string());
        let result = parser::value_to_i64(&value);
        assert_eq!(result, 1);
    }

    #[test]
    fn value_to_i32() {
        let value = serde_json::Value::String("1".to_string());
        let result = parser::value_to_i32(&value);
        assert_eq!(result, 1);
    }

    #[test]
    fn value_to_f64() {
        let value = serde_json::Value::String("1.9".to_string());
        let result = parser::value_to_f64(&value);
        assert_eq!(result, 1.9);
    }

    #[test]
    fn value_to_string() {
        let value = serde_json::Value::String("1.9".to_string());
        let result = parser::value_to_string(&value);
        assert_eq!(result, "1.9");
    }

    #[test]
    fn string_to_i32() {
        let value = serde_json::Value::String("1".to_string());
        let result = parser::string_to_i32(&value);
        assert_eq!(result, 1);
    }

    #[test]
    fn string_to_uuid() {
        let value = "550e8400-e29b-41d4-a716-446655440000".to_string();
        let result = parser::string_to_uuid(value);
        assert_eq!(
            result.unwrap(),
            uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap()
        );
    }

    #[test]
    fn f64_to_bigdecimal() {
        let value = 1.9;
        let result = parser::f64_to_bigdecimal(&value);
        assert_eq!(parser::type_of(&result), "bigdecimal::BigDecimal");
    }

    #[test]
    fn type_of() {
        let type_string: String = "String".to_string();
        let type_i32: i32 = 1;
        let type_i64: i64 = 1;
        let type_f64: f64 = 1.9;
        let type_bool: bool = true;
        assert!(parser::type_of(&type_string) == "alloc::string::String");
        assert!(parser::type_of(&type_i32) == "i32");
        assert!(parser::type_of(&type_i64) == "i64");
        assert!(parser::type_of(&type_f64) == "f64");
        assert!(parser::type_of(&type_bool) == "bool");
    }
}
