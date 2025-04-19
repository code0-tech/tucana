use crate::shared::value::Kind;
use crate::shared::{ListValue, Struct, Value};
use serde_json::{Number, Value as JsonValue};

/// Converts a serde_json::Value into our internal Value type
pub fn from_json_value(value: JsonValue) -> Value {
    match value {
        JsonValue::Null => Value {
            kind: Some(Kind::NullValue(0)),
        },
        JsonValue::Bool(b) => Value {
            kind: Some(Kind::BoolValue(b)),
        },
        JsonValue::Number(n) => Value {
            kind: Some(Kind::NumberValue(n.as_f64().unwrap_or_default())),
        },
        JsonValue::String(s) => Value {
            kind: Some(Kind::StringValue(s)),
        },
        JsonValue::Array(arr) => Value {
            kind: Some(Kind::ListValue(ListValue {
                values: arr.into_iter().map(|v| from_json_value(v)).collect(),
            })),
        },
        JsonValue::Object(obj) => Value {
            kind: Some(Kind::StructValue(Struct {
                fields: obj
                    .into_iter()
                    .map(|(k, v)| (k, from_json_value(v)))
                    .collect(),
            })),
        },
    }
}

/// Converts our internal Value type back to a serde_json::Value
pub fn to_json_value(value: Value) -> JsonValue {
    match value.kind {
        Some(Kind::NullValue(_)) => JsonValue::Null,
        Some(Kind::BoolValue(b)) => JsonValue::Bool(b),
        Some(Kind::NumberValue(n)) => {
            // Try to preserve the original number format when possible
            if n.fract() == 0.0 && n >= 0.0 && n <= i64::MAX as f64 {
                // It's an integer within the range of i64
                match Number::from(n as i64) {
                    num => JsonValue::Number(num),
                }
            } else {
                // It's a floating point or out of i64 range
                JsonValue::Number(Number::from_f64(n).expect("Invalid number value"))
            }
        }
        Some(Kind::StringValue(s)) => JsonValue::String(s),
        Some(Kind::ListValue(ListValue { values })) => {
            JsonValue::Array(values.into_iter().map(to_json_value).collect())
        }
        Some(Kind::StructValue(Struct { fields })) => JsonValue::Object(
            fields
                .into_iter()
                .map(|(k, v)| (k, to_json_value(v)))
                .collect(),
        ),
        None => JsonValue::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_from_json_null() {
        let json_val = serde_json::Value::Null;
        let val = from_json_value(json_val);
        assert!(matches!(val.kind, Some(Kind::NullValue(_))));
    }

    #[test]
    fn test_from_json_bool() {
        let json_val = serde_json::Value::Bool(true);
        let val = from_json_value(json_val);
        assert!(matches!(val.kind, Some(Kind::BoolValue(true))));

        let json_val = serde_json::Value::Bool(false);
        let val = from_json_value(json_val);
        assert!(matches!(val.kind, Some(Kind::BoolValue(false))));
    }

    #[test]
    fn test_from_json_number() {
        let json_val = json!(42);
        let val = from_json_value(json_val);
        if let Some(Kind::NumberValue(n)) = val.kind {
            assert_eq!(n, 42.0);
        } else {
            panic!("Expected NumberValue kind");
        }
    }

    #[test]
    fn test_from_json_string() {
        let json_val = json!("hello");
        let val = from_json_value(json_val);
        if let Some(Kind::StringValue(s)) = val.kind {
            assert_eq!(s, "hello");
        } else {
            panic!("Expected StringValue kind");
        }
    }

    #[test]
    fn test_from_json_array() {
        let json_val = json!([1, "test", true, null]);
        let val = from_json_value(json_val);
        if let Some(Kind::ListValue(list)) = val.kind {
            assert_eq!(list.values.len(), 4);
            assert!(matches!(list.values[0].kind, Some(Kind::NumberValue(_))));
            assert!(matches!(list.values[1].kind, Some(Kind::StringValue(_))));
            assert!(matches!(list.values[2].kind, Some(Kind::BoolValue(true))));
            assert!(matches!(list.values[3].kind, Some(Kind::NullValue(_))));
        } else {
            panic!("Expected ListValue kind");
        }
    }

    #[test]
    fn test_from_json_object() {
        let json_val = json!({
            "number": 42,
            "string": "value",
            "bool": true,
            "null": null
        });
        let val = from_json_value(json_val);
        if let Some(Kind::StructValue(s)) = val.kind {
            assert_eq!(s.fields.len(), 4);
            assert!(matches!(
                s.fields.get("number").unwrap().kind,
                Some(Kind::NumberValue(_))
            ));
            assert!(matches!(
                s.fields.get("string").unwrap().kind,
                Some(Kind::StringValue(_))
            ));
            assert!(matches!(
                s.fields.get("bool").unwrap().kind,
                Some(Kind::BoolValue(true))
            ));
            assert!(matches!(
                s.fields.get("null").unwrap().kind,
                Some(Kind::NullValue(_))
            ));
        } else {
            panic!("Expected StructValue kind");
        }
    }

    #[test]
    fn test_to_json_null() {
        let val = Value {
            kind: Some(Kind::NullValue(0)),
        };
        let json_val = to_json_value(val);
        assert!(json_val.is_null());
    }

    #[test]
    fn test_to_json_bool() {
        let val = Value {
            kind: Some(Kind::BoolValue(true)),
        };
        let json_val = to_json_value(val);
        assert!(json_val.is_boolean());
        assert_eq!(json_val.as_bool().unwrap(), true);
    }

    #[test]
    fn test_to_json_number() {
        let val = Value {
            kind: Some(Kind::NumberValue(42.0)),
        };
        let json_val = to_json_value(val);
        assert!(json_val.is_number());
        assert_eq!(json_val.as_f64().unwrap(), 42.0);
    }

    #[test]
    fn test_to_json_string() {
        let val = Value {
            kind: Some(Kind::StringValue("hello".to_string())),
        };
        let json_val = to_json_value(val);
        assert!(json_val.is_string());
        assert_eq!(json_val.as_str().unwrap(), "hello");
    }

    #[test]
    fn test_to_json_array() {
        let val = Value {
            kind: Some(Kind::ListValue(ListValue {
                values: vec![
                    Value {
                        kind: Some(Kind::NumberValue(1.0)),
                    },
                    Value {
                        kind: Some(Kind::StringValue("test".to_string())),
                    },
                ],
            })),
        };
        let json_val = to_json_value(val);
        assert!(json_val.is_array());
        let arr = json_val.as_array().unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0].as_f64().unwrap(), 1.0);
        assert_eq!(arr[1].as_str().unwrap(), "test");
    }

    #[test]
    fn test_to_json_object() {
        let mut fields = std::collections::HashMap::new();
        fields.insert(
            "key1".to_string(),
            Value {
                kind: Some(Kind::StringValue("value1".to_string())),
            },
        );
        fields.insert(
            "key2".to_string(),
            Value {
                kind: Some(Kind::NumberValue(42.0)),
            },
        );

        let val = Value {
            kind: Some(Kind::StructValue(Struct { fields })),
        };
        let json_val = to_json_value(val);
        assert!(json_val.is_object());
        let obj = json_val.as_object().unwrap();
        assert_eq!(obj.len(), 2);
        assert_eq!(obj.get("key1").unwrap().as_str().unwrap(), "value1");
        assert_eq!(obj.get("key2").unwrap().as_f64().unwrap(), 42.0);
    }

    #[test]
    fn test_roundtrip_conversion() {
        let original = json!({
            "string": "hello",
            "number": 42.5,
            "bool": true,
            "null": null,
            "array": [1, 2, 3],
            "object": {
                "nested": "value"
            }
        });

        let val = from_json_value(original.clone());
        let roundtrip = to_json_value(val);

        assert_eq!(original, roundtrip);
    }

    #[test]
    fn test_none_kind() {
        let val = Value { kind: None };
        let json_val = to_json_value(val);
        assert!(json_val.is_null());
    }
}
