/*=============================================================
 * THIS LIB IS A CLONE https://github.com/rtyler/xmltojson    =
 *=============================================================
*/

/*=============================================================
 *                    EDIT FOR FELIPE CN                      =
 *=============================================================
*/

mod core;
mod json;
mod utils;
pub use json::{to_json_from_file, to_json_from_str};

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn test_read_file() {
        let result = utils::read_file("tests/data/1.xml").unwrap();
        assert_eq!(result, "<a>1</a>");
    }

    #[test]
    fn test_to_json_from_file() {
        let result = to_json_from_file("tests/data/2.xml").unwrap();
        assert_eq!(
            result,
            json!({
                "testes": {
                    "teste": [
                        {
                            "@id": "1",
                            "nome": "Teste 1",
                            "resultado": "Passou"
                        },
                        {
                            "@id": "2",
                            "nome": "Teste 2",
                            "resultado": "Falhou"
                        }
                    ]
                }
            })
        );
    }
}
