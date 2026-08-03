use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn invoke(tool: &str, arguments: JsValue) -> Result<JsValue, JsValue> {
    let arguments: serde_json::Value = serde_wasm_bindgen::from_value(arguments)
        .map_err(|error| JsValue::from_str(&format!("invalid arguments: {error}")))?;
    let response = fiscal_core::invoke(tool, &arguments)
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let serializer = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
    response
        .serialize(&serializer)
        .map_err(|error| JsValue::from_str(&format!("serialization failed: {error}")))
}

#[wasm_bindgen]
pub fn contract_json() -> String {
    fiscal_core::CONTRACTS_JSON.to_owned()
}
