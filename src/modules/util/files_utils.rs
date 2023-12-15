use serde_json::Value;
pub use std::fs::File;
pub use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_file(file: String) -> String {
    let mut result = String::new();

    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(line_value) = line {
                result.push_str(&line_value);
            }
        }
    }
    return result;
}

pub fn xml_to_json(file: String) -> String {
    use xml2json_rs::JsonBuilder;
    let xml_content = read_file(file);
    let json_builder = JsonBuilder::default();
    let json = json_builder.build_string_from_xml(&xml_content);
    return json.expect("s");
}

pub fn json_to_object(file_path: String) -> Value {
    let result = xml_to_json(file_path.to_string());
    let v: Value = serde_json::from_str(&result).unwrap();
    return v;
}
