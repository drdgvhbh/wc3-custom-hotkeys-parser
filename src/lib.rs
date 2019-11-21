extern crate wasm_bindgen;

use std::error::Error;
use wasm_bindgen::prelude::*;

mod ast;

#[allow(dead_code)]
mod parser;

#[wasm_bindgen]
pub fn parse_custom_hotkeys(custom_hotkeys: &str) -> Result<Vec<JsValue>, JsValue> {
    let p = parser::HotkeyOverridesParser::new();
    let result = p.parse(custom_hotkeys);
    if result.is_err() {
        return Err(JsValue::from_str(result.unwrap_err().description()));
    }

    Ok(result
        .unwrap()
        .into_iter()
        .map(|ho| JsValue::from_serde(&ho).unwrap())
        .map(JsValue::from)
        .collect::<Vec<_>>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let contents =
            fs::read_to_string("custom-hotkeys.txt").expect("should be able to read test file");
        let p = parser::HotkeyOverridesParser::new();
        let result = p.parse(&contents);
        result.expect("it should parse");
    }
    #[test]

    fn it_parses_ability_id() {
        let contents = "[abcd]";
        let p = parser::AbilityIdParser::new();
        let result = p.parse(&contents);
        result.expect("it should parse");
    }
}
