mod engine;
pub mod registry;

pub use engine::{FiscalError, ToolResponse, invoke};

pub const CONTRACTS_JSON: &str = include_str!("../../../contracts/tools.json");

pub fn tool_count() -> usize {
    serde_json::from_str::<serde_json::Value>(CONTRACTS_JSON)
        .ok()
        .and_then(|v| v.get("tools")?.as_array().map(Vec::len))
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Map, Value, json};
    use std::collections::BTreeSet;

    #[test]
    fn contract_has_exactly_62_unique_tools() {
        let contract: serde_json::Value = serde_json::from_str(CONTRACTS_JSON).unwrap();
        let tools = contract["tools"].as_array().unwrap();
        let names: BTreeSet<_> = tools
            .iter()
            .map(|tool| tool["name"].as_str().unwrap())
            .collect();
        assert_eq!(tools.len(), 62);
        assert_eq!(names.len(), 62);
    }

    #[test]
    fn all_contract_annotations_are_safe() {
        let contract: serde_json::Value = serde_json::from_str(CONTRACTS_JSON).unwrap();
        for tool in contract["tools"].as_array().unwrap() {
            let a = &tool["annotations"];
            assert_eq!(a["readOnlyHint"], true);
            assert_eq!(a["destructiveHint"], false);
            assert_eq!(a["idempotentHint"], true);
            assert_eq!(a["openWorldHint"], false);
        }
    }

    fn sample_value(schema: &Value) -> Value {
        if let Some(first) = schema
            .get("enum")
            .and_then(Value::as_array)
            .and_then(|values| values.first())
        {
            return first.clone();
        }
        match schema
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or("string")
        {
            "number" => json!(1.0),
            "integer" => json!(1),
            "boolean" => json!(false),
            "array" => json!([]),
            "object" => json!({}),
            _ => json!("test"),
        }
    }

    #[test]
    fn every_declared_tool_is_dispatchable() {
        let contract: Value = serde_json::from_str(CONTRACTS_JSON).unwrap();
        for tool in contract["tools"].as_array().unwrap() {
            let schema = &tool["inputSchema"];
            let mut args = Map::new();
            if let Some(required) = schema.get("required").and_then(Value::as_array) {
                for name in required.iter().filter_map(Value::as_str) {
                    args.insert(name.to_owned(), sample_value(&schema["properties"][name]));
                }
            }
            let name = tool["name"].as_str().unwrap();
            invoke(name, &Value::Object(args))
                .unwrap_or_else(|error| panic!("tool {name} is not dispatchable: {error}"));
        }
    }
}
