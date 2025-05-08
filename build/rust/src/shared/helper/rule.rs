use crate::shared::data_type_input_types_rule_config::DataTypeInputType;
use crate::shared::{
    DataTypeContainsKeyRuleConfig, DataTypeContainsTypeRuleConfig, DataTypeInputTypesRuleConfig,
    DataTypeItemOfCollectionRuleConfig, DataTypeNumberRangeRuleConfig, DataTypeRegexRuleConfig,
    DataTypeReturnTypeRuleConfig, DataTypeRule, Value, data_type_rule::Config,
};

pub struct RuleBuilder {
    rules: Vec<DataTypeRule>,
}

impl RuleBuilder {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_contains_key(mut self, key: String, data_type_identifier: String) -> Self {
        self.rules.push(DataTypeRule {
            config: Some(Config::ContainsKey(DataTypeContainsKeyRuleConfig {
                key,
                data_type_identifier,
            })),
        });
        self
    }

    pub fn add_contains_type(mut self, data_type_identifier: String) -> Self {
        self.rules.push(DataTypeRule {
            config: Some(Config::ContainsType(DataTypeContainsTypeRuleConfig {
                data_type_identifier,
            })),
        });
        self
    }

    pub fn add_item_of_collection(mut self, items: Vec<Value>) -> Self {
        self.rules.push(DataTypeRule {
            config: Some(Config::ItemOfCollection(
                DataTypeItemOfCollectionRuleConfig { items },
            )),
        });
        self
    }

    pub fn add_number_range(mut self, from: i64, to: i64, steps: Option<i64>) -> Self {
        self.rules.push(DataTypeRule {
            config: Some(Config::NumberRange(DataTypeNumberRangeRuleConfig {
                from,
                to,
                steps,
            })),
        });
        self
    }

    pub fn add_regex(mut self, pattern: String) -> Self {
        self.rules.push(DataTypeRule {
            config: Some(Config::Regex(DataTypeRegexRuleConfig { pattern })),
        });
        self
    }

    pub fn add_input_types(mut self, input_types: Vec<DataTypeInputType>) -> Self {
        self.rules.push(DataTypeRule {
            config: Some(Config::InputTypes(DataTypeInputTypesRuleConfig {
                input_types,
            })),
        });
        self
    }

    pub fn add_return_type(mut self, data_type_identifier: String) -> Self {
        self.rules.push(DataTypeRule {
            config: Some(Config::ReturnType(DataTypeReturnTypeRuleConfig {
                data_type_identifier,
            })),
        });
        self
    }

    pub fn build(self) -> Vec<DataTypeRule> {
        self.rules
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::{
        data_type_input_types_rule_config::DataTypeInputType, data_type_rule::Config,
        helper::value::ToValue,
    };

    #[test]
    fn test_add_contains_key() {
        let rules = RuleBuilder::new()
            .add_contains_key("id".into(), "User".into())
            .build();

        match &rules[0].config {
            Some(Config::ContainsKey(cfg)) => {
                assert_eq!(cfg.key, "id");
                assert_eq!(cfg.data_type_identifier, "User");
            }
            _ => panic!("Expected ContainsKey config"),
        }
    }

    #[test]
    fn test_add_contains_type() {
        let rules = RuleBuilder::new().add_contains_type("User".into()).build();

        match &rules[0].config {
            Some(Config::ContainsType(cfg)) => {
                assert_eq!(cfg.data_type_identifier, "User");
            }
            _ => panic!("Expected ContainsType config"),
        }
    }

    #[test]
    fn test_add_item_of_collection() {
        let items = vec!["a".to_value(), 42.to_value()];
        let rules = RuleBuilder::new()
            .add_item_of_collection(items.clone())
            .build();

        match &rules[0].config {
            Some(Config::ItemOfCollection(cfg)) => {
                assert_eq!(cfg.items, items);
            }
            _ => panic!("Expected ItemOfCollection config"),
        }
    }

    #[test]
    fn test_add_number_range() {
        let rules = RuleBuilder::new().add_number_range(1, 10, Some(2)).build();

        match &rules[0].config {
            Some(Config::NumberRange(cfg)) => {
                assert_eq!(cfg.from, 1);
                assert_eq!(cfg.to, 10);
                assert_eq!(cfg.steps, Some(2));
            }
            _ => panic!("Expected NumberRange config"),
        }
    }

    #[test]
    fn test_add_regex() {
        let rules = RuleBuilder::new().add_regex(r"^\d+$".into()).build();

        match &rules[0].config {
            Some(Config::Regex(cfg)) => {
                assert_eq!(cfg.pattern, r"^\d+$");
            }
            _ => panic!("Expected Regex config"),
        }
    }

    #[test]
    fn test_add_input_types() {
        let input_types = vec![
            DataTypeInputType {
                data_type_identifier: "Type1".into(),
                input_identifier: "input1".into(),
            },
            DataTypeInputType {
                data_type_identifier: "Type2".into(),
                input_identifier: "input2".into(),
            },
        ];

        let rules = RuleBuilder::new()
            .add_input_types(input_types.clone())
            .build();

        match &rules[0].config {
            Some(Config::InputTypes(cfg)) => {
                assert_eq!(cfg.input_types, input_types);
            }
            _ => panic!("Expected InputTypes config"),
        }
    }

    #[test]
    fn test_add_return_type() {
        let rules = RuleBuilder::new().add_return_type("Result".into()).build();

        match &rules[0].config {
            Some(Config::ReturnType(cfg)) => {
                assert_eq!(cfg.data_type_identifier, "Result");
            }
            _ => panic!("Expected ReturnType config"),
        }
    }

    #[test]
    fn test_add_many_rules() {
        let rules = RuleBuilder::new()
            .add_contains_key("id".into(), "User".into())
            .add_return_type("Result".into())
            .add_regex(r"^\d+$".into())
            .add_contains_key("id".into(), "User".into())
            .build();

        match &rules[0].config {
            Some(Config::ContainsKey(cfg)) => {
                assert_eq!(cfg.key, "id");
                assert_eq!(cfg.data_type_identifier, "User");
            }
            _ => panic!("Expected ContainsKey config"),
        }

        match &rules[1].config {
            Some(Config::ReturnType(cfg)) => {
                assert_eq!(cfg.data_type_identifier, "Result");
            }
            _ => panic!("Expected ReturnType config"),
        }

        match &rules[2].config {
            Some(Config::Regex(cfg)) => {
                assert_eq!(cfg.pattern, r"^\d+$");
            }
            _ => panic!("Expected Regex config"),
        }

        match &rules[3].config {
            Some(Config::ContainsKey(cfg)) => {
                assert_eq!(cfg.key, "id");
                assert_eq!(cfg.data_type_identifier, "User");
            }
            _ => panic!("Expected ContainsKey config"),
        }
    }
}
