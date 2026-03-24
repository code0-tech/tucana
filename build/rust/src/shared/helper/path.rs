use crate::shared::{ListValue, Struct, Value, value::Kind, number_value};
use std::collections::HashMap;

/// Get the Kind at a given path from a Value
/// Returns None if:
/// - Path is invalid or doesn't exist in the Value
/// - Value at the path doesn't have a kind
/// - Path traversal encounters a non-struct value
pub fn expect_kind(path: &str, value: &Value) -> Option<Kind> {
    let kind = match &value.kind {
        Some(kind) => kind,
        None => return None,
    };

    let mut items: Vec<&str> = path.split(".").collect();

    if items.is_empty() {
        return None;
    }

    let first = match &items.first() {
        Some(key) => key.to_string().clone(),
        None => return None,
    };

    items.remove(0);

    match kind {
        Kind::StructValue(struct_value) => match struct_value.fields.get(&first) {
            Some(value) => {
                if items.is_empty() {
                    match &value.kind {
                        Some(kind) => return Some(kind.clone()),
                        None => return None,
                    }
                } else {
                    return expect_kind(items.join(".").as_str(), value);
                }
            }
            None => return None,
        },
        _ => return None,
    }
}

/// Get a reference to a Value at a given path
/// Returns None if the path doesn't exist
pub fn get_value(path: &str, value: Value) -> Option<Value> {
    let kind = match &value.kind {
        Some(kind) => kind,
        None => return None,
    };

    let mut items: Vec<&str> = path.split(".").collect();
    if items.is_empty() {
        return Some(value);
    }

    let first = items.remove(0);

    match kind {
        Kind::StructValue(struct_value) => {
            let field = struct_value.fields.get(first)?;
            if items.is_empty() {
                Some(field.clone())
            } else {
                get_value(&items.join("."), field.clone())
            }
        }
        _ => None,
    }
}

/// Check if a path exists in a Value
pub fn exists_path(path: &str, value: &Value) -> bool {
    get_value(path, value.clone()).is_some()
}

/// Extract a string value from a path
pub fn get_string(path: &str, value: &Value) -> Option<String> {
    match expect_kind(path, value)? {
        Kind::StringValue(s) => Some(s),
        _ => None,
    }
}

/// Extract an integer value from a path
pub fn get_integer(path: &str, value: &Value) -> Option<i64> {
    match expect_kind(path, value)? {
        Kind::NumberValue(n) => match n.number {
            Some(number_value::Number::Integer(i)) => Some(i),
            None => None,
            _ => None,
        },
        _ => None,
    }
}

/// Extract a float value from a path
pub fn get_float(path: &str, value: &Value) -> Option<f64> {
    match expect_kind(path, value)? {
        Kind::NumberValue(n) => match n.number {
            Some(number_value::Number::Float(f)) => Some(f),
            None => None,
            _ => None,
        },
        _ => None,
    }
}

/// Extract a boolean value from a path
pub fn get_bool(path: &str, value: &Value) -> Option<bool> {
    match expect_kind(path, value)? {
        Kind::BoolValue(b) => Some(b),
        _ => None,
    }
}

/// Extract a list value from a path
pub fn get_list(path: &str, value: &Value) -> Option<ListValue> {
    match expect_kind(path, value)? {
        Kind::ListValue(l) => Some(l),
        _ => None,
    }
}

/// Extract a struct value from a path
pub fn get_struct(path: &str, value: &Value) -> Option<Struct> {
    match expect_kind(path, value)? {
        Kind::StructValue(s) => Some(s),
        _ => None,
    }
}

/// Set a value at a specific path
/// Returns a new Value with the modified path
pub fn set_value(path: &str, current: &Value, new_value: Value) -> Value {
    if path.is_empty() {
        return new_value;
    }

    let mut result = current.clone();

    let mut items: Vec<&str> = path.split(".").collect();
    if items.is_empty() {
        return result;
    }

    let first = items.remove(0);

    if let Some(Kind::StructValue(struct_value)) = &mut result.kind {
        if items.is_empty() {
            // We're at the leaf node, set the value
            struct_value.fields.insert(first.to_string(), new_value);
        } else {
            // Navigate deeper
            let next_path = items.join(".");
            let default_value = Value {
                kind: Some(Kind::StructValue(Struct {
                    fields: HashMap::new(),
                })),
            };

            let existing_value = struct_value
                .fields
                .get(first)
                .cloned()
                .unwrap_or(default_value.clone());

            let updated_value = set_value(&next_path, &existing_value, new_value);
            struct_value.fields.insert(first.to_string(), updated_value);
        }
    } else if result.kind.is_none() {
        // If the current value has no kind, create a struct
        let mut fields = HashMap::new();

        if items.is_empty() {
            fields.insert(first.to_string(), new_value);
        } else {
            let next_path = items.join(".");
            let default_value = Value {
                kind: Some(Kind::StructValue(Struct {
                    fields: HashMap::new(),
                })),
            };
            let updated_value = set_value(&next_path, &default_value, new_value);
            fields.insert(first.to_string(), updated_value);
        }

        result.kind = Some(Kind::StructValue(Struct { fields }));
    }

    result
}

#[cfg(test)]
pub mod tests {

    use crate::shared::{
        NumberValue, Struct, Value, number_value,
        helper::path::{
            exists_path, expect_kind, get_bool, get_float, get_integer, get_list, get_string,
            get_struct,
            set_value,
        },
    };
    use std::collections::HashMap;

    #[test]
    fn test_expect_none() {
        let value = Value {
            kind: Some(crate::shared::value::Kind::StructValue(Struct {
                fields: HashMap::from([
                    (
                        "name".to_string(),
                        Value {
                            kind: Some(crate::shared::value::Kind::StringValue("John".to_string())),
                        },
                    ),
                    (
                        "age".to_string(),
                        Value {
                            kind: Some(crate::shared::value::Kind::NumberValue(NumberValue {
                                number: Some(number_value::Number::Integer(30)),
                            })),
                        },
                    ),
                ]),
            })),
        };

        assert_eq!(expect_kind(".", &value), None);
        assert_eq!(expect_kind("", &value), None);
    }

    #[test]
    fn test_expect_kind() {
        let value = Value {
            kind: Some(crate::shared::value::Kind::StructValue(
                crate::shared::Struct {
                    fields: HashMap::from([
                        (
                            "name".to_string(),
                            Value {
                                kind: Some(crate::shared::value::Kind::StringValue(
                                    "John".to_string(),
                                )),
                            },
                        ),
                        (
                            "age".to_string(),
                            Value {
                                kind: Some(crate::shared::value::Kind::NumberValue(NumberValue {
                                    number: Some(number_value::Number::Integer(30)),
                                })),
                            },
                        ),
                    ]),
                },
            )),
        };
        assert_eq!(
            expect_kind("name", &value),
            Some(crate::shared::value::Kind::StringValue("John".to_string()))
        );
        assert_eq!(
            expect_kind("age", &value),
            Some(crate::shared::value::Kind::NumberValue(NumberValue {
                number: Some(number_value::Number::Integer(30)),
            }))
        );
        assert_eq!(expect_kind("address", &value), None);
    }

    #[test]
    fn test_expect_kind_nested() {
        let value = Value {
            kind: Some(crate::shared::value::Kind::StructValue(
                crate::shared::Struct {
                    fields: HashMap::from([
                        (
                            "name".to_string(),
                            Value {
                                kind: Some(crate::shared::value::Kind::StringValue(
                                    "John".to_string(),
                                )),
                            },
                        ),
                        (
                            "address".to_string(),
                            Value {
                                kind: Some(crate::shared::value::Kind::StructValue(
                                    crate::shared::Struct {
                                        fields: HashMap::from([
                                            (
                                                "street".to_string(),
                                                Value {
                                                    kind: Some(
                                                        crate::shared::value::Kind::StringValue(
                                                            "123 Main St".to_string(),
                                                        ),
                                                    ),
                                                },
                                            ),
                                            (
                                                "city".to_string(),
                                                Value {
                                                    kind: Some(
                                                        crate::shared::value::Kind::StringValue(
                                                            "Anytown".to_string(),
                                                        ),
                                                    ),
                                                },
                                            ),
                                            (
                                                "zipcode".to_string(),
                                                Value {
                                                    kind: Some(
                                                        crate::shared::value::Kind::NumberValue(
                                                            NumberValue {
                                                                number: Some(
                                                                    number_value::Number::Integer(
                                                                        12345,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ]),
                                    },
                                )),
                            },
                        ),
                    ]),
                },
            )),
        };

        // Test basic top-level fields
        assert_eq!(
            expect_kind("name", &value),
            Some(crate::shared::value::Kind::StringValue("John".to_string()))
        );

        // Test nested fields
        assert_eq!(
            expect_kind("address.street", &value),
            Some(crate::shared::value::Kind::StringValue(
                "123 Main St".to_string()
            ))
        );
        assert_eq!(
            expect_kind("address.city", &value),
            Some(crate::shared::value::Kind::StringValue(
                "Anytown".to_string()
            ))
        );
        assert_eq!(
            expect_kind("address.zipcode", &value),
            Some(crate::shared::value::Kind::NumberValue(NumberValue {
                number: Some(number_value::Number::Integer(12345)),
            }))
        );

        // Test nonexistent fields
        assert_eq!(expect_kind("address.country", &value), None);
        assert_eq!(expect_kind("phone", &value), None);
        assert_eq!(expect_kind("address.street.number", &value), None);

        assert!(exists_path("address.city", &value));
        assert!(!exists_path("address.street.number", &value));
    }

    #[test]
    fn test_get_type_functions() {
        let value = Value {
            kind: Some(crate::shared::value::Kind::StructValue(
                crate::shared::Struct {
                    fields: HashMap::from([
                        (
                            "name".to_string(),
                            Value {
                                kind: Some(crate::shared::value::Kind::StringValue(
                                    "John".to_string(),
                                )),
                            },
                        ),
                        (
                            "age".to_string(),
                            Value {
                                kind: Some(crate::shared::value::Kind::NumberValue(NumberValue {
                                    number: Some(number_value::Number::Integer(30)),
                                })),
                            },
                        ),
                        (
                            "is_active".to_string(),
                            Value {
                                kind: Some(crate::shared::value::Kind::BoolValue(true)),
                            },
                        ),
                        (
                            "skills".to_string(),
                            Value {
                                kind: Some(crate::shared::value::Kind::ListValue(
                                    crate::shared::ListValue {
                                        values: vec![
                                            Value {
                                                kind: Some(
                                                    crate::shared::value::Kind::StringValue(
                                                        "Rust".to_string(),
                                                    ),
                                                ),
                                            },
                                            Value {
                                                kind: Some(
                                                    crate::shared::value::Kind::StringValue(
                                                        "TypeScript".to_string(),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                )),
                            },
                        ),
                    ]),
                },
            )),
        };

        // Test get_string
        assert_eq!(get_string("name", &value), Some("John".to_string()));
        assert_eq!(get_string("age", &value), None);

        // Test get_integer / get_float
        assert_eq!(get_integer("age", &value), Some(30));
        assert_eq!(get_integer("name", &value), None);
        assert_eq!(get_float("age", &value), None);
        assert_eq!(get_float("name", &value), None);

        // Test get_bool
        assert_eq!(get_bool("is_active", &value), Some(true));
        assert_eq!(get_bool("name", &value), None);

        // Test get_list
        let list = get_list("skills", &value);
        assert!(list.is_some());
        assert_eq!(list.unwrap().values.len(), 2);

        // Test get_struct
        let address_struct = get_struct("address", &value);
        assert!(address_struct.is_none());
    }

    #[test]
    fn test_set_value() {
        let value = Value {
            kind: Some(crate::shared::value::Kind::StructValue(
                crate::shared::Struct {
                    fields: HashMap::from([(
                        "name".to_string(),
                        Value {
                            kind: Some(crate::shared::value::Kind::StringValue("John".to_string())),
                        },
                    )]),
                },
            )),
        };

        // Test setting a top-level field
        let new_value = Value {
            kind: Some(crate::shared::value::Kind::NumberValue(NumberValue {
                number: Some(number_value::Number::Integer(40)),
            })),
        };
        let result = set_value("age", &value, new_value);

        assert_eq!(get_integer("age", &result), Some(40));
        assert_eq!(get_string("name", &result), Some("John".to_string()));

        // Test setting a nested field
        let address_value = Value {
            kind: Some(crate::shared::value::Kind::StringValue(
                "New York".to_string(),
            )),
        };
        let result = set_value("address.city", &value, address_value);

        assert_eq!(
            get_string("address.city", &result),
            Some("New York".to_string())
        );
        assert_eq!(get_string("name", &result), Some("John".to_string()));

        // Test overwriting an existing field
        let new_name = Value {
            kind: Some(crate::shared::value::Kind::StringValue("Jane".to_string())),
        };
        let result = set_value("name", &value, new_name);

        assert_eq!(get_string("name", &result), Some("Jane".to_string()));
    }
}
