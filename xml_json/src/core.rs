extern crate serde_json;

use log::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use serde_json::{Map, Value};

#[derive(Debug)]
pub struct Error {}

pub fn read(reader: &mut Reader<&[u8]>, depth: u64) -> Value {
    let mut buf = Vec::new();
    let mut values = Vec::new();
    let mut node = Map::new();
    debug!("Parsing at depth: {}", depth);

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if let Ok(name) = String::from_utf8(e.name().into_inner().to_vec()) {
                    let mut child = read(reader, depth + 1);
                    let mut attrs = Map::new();
                    debug!("{} children: {:?}", name, child);

                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            let key = String::from_utf8(attr.key.into_inner().to_vec());
                            let value = String::from_utf8(attr.value.to_vec());

                            // Only bother adding the attribute if both key and value are valid utf8
                            if let (Ok(key), Ok(value)) = (key, value) {
                                let key = format!("@{}", key);
                                let value = Value::String(value);

                                // If the child is already an object, that's where the insert
                                // should happen
                                if child.is_object() {
                                    child.as_object_mut().unwrap().insert(key, value);
                                } else {
                                    attrs.insert(key, value);
                                }
                            }
                        }
                    }

                    /*
                     * nodes with attributes need to be handled special
                     */
                    if !attrs.is_empty() {
                        if child.is_string() {
                            attrs.insert("#text".to_string(), child);
                        }

                        if let Ok(attrs) = serde_json::to_value(attrs) {
                            node.insert(name, attrs);
                        }
                    } else if let Some(existing) = node.remove(&name) {
                        debug!("Node contains `{}` already, need to convert to array", name);
                        let mut entries: Vec<Value> = vec![];

                        if existing.is_array() {
                            let existing = existing.as_array().unwrap();
                            entries.extend(existing.iter().cloned());
                        } else {
                            entries.push(existing);
                        }
                        entries.push(child);

                        node.insert(name, Value::Array(entries));
                    } else {
                        node.insert(name, child);
                    }
                }
            }
            Ok(Event::Text(ref e)) => {
                if let Ok(decoded) = e.unescape() {
                    values.push(Value::String(decoded.to_string()));
                }
            }
            Ok(Event::CData(ref e)) => {
                if let Ok(decoded) = e.clone().escape() {
                    if let Ok(decoded_bt) = decoded.unescape() {
                        node.insert("#cdata".to_string(), Value::String(decoded_bt.to_string()));
                    }
                }
            }
            Ok(Event::End(ref _e)) => break,
            Ok(Event::Eof) => break,
            _ => (),
        }
    }

    debug!("values to return: {:?}", values);
    if !node.is_empty() {
        // If we had collected some text along the way, that needs to be inserted
        // so we don't lose it
        let mut index = 0;
        let mut has_text = false;
        for value in values.iter() {
            if value.is_string() {
                has_text = true;
                break;
            }
            index += 1;
        }

        if has_text {
            node.insert("#text".to_string(), values.remove(index));
        }
        debug!("returning node instead: {:?}", node);
        return serde_json::to_value(&node).expect("Failed to #to_value() a node!");
    }

    match values.len() {
        0 => Value::Null,
        1 => values.pop().unwrap(),
        _ => Value::Array(values),
    }
}
