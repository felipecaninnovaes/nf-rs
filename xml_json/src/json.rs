use quick_xml::Reader;
use serde::de::value::Error;
use serde_json::Value;

use super::core::read;
use super::utils::read_file;

/**
 * to_json() will take an input string and attempt to convert it into a form
 * of JSON
 */
pub fn to_json_from_str(xml: &str) -> Result<Value, Error> {
    let mut reader = Reader::from_str(xml);
    reader.expand_empty_elements(true).trim_text(true);
    Ok(read(&mut reader, 0))
}

pub fn to_json_from_file(xml_file: &str) -> Result<Value, Error> {
    let reader = to_json_from_str(&read_file(xml_file).unwrap()).expect("Error converting xml to json");
    Ok(reader)
}
