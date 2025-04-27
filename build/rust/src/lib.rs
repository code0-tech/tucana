#[cfg(feature = "shared")]
pub mod shared {
    pub mod helper;

    include!("generated/shared.rs");

    #[cfg(test)]
    pub mod tests {
        use crate::shared::Flow;
        use serde_json;

        #[test]
        fn test_serialize() {
            let flow = Flow {
                flow_id: 0,
                project_id: 0,
                data_types: vec![],
                input_type_identifier: None,
                return_type_identifier: None,
                r#type: "no".to_string(),
                settings: vec![],
                starting_node: None,
            };

            let str_flow = serde_json::to_string(&flow).expect("Serialization failed");
            let json_flow: serde_json::Value =
                serde_json::from_str(&str_flow).expect("Failed to parse JSON");

            let expected_json: serde_json::Value = serde_json::json!({
                "flow_id": 0,
                "project_id": 0,
                "type": "no",
                "input_type_identifier": null,
                "return_type_identifier": null,
                "data_types": [],
                "settings": [],
                "starting_node": null
            });

            assert_eq!(json_flow, expected_json);
        }

        #[test]
        fn test_deserialize() {
            let json_data = r#"{
            "flow_id": 0,
            "project_id": 0,
            "type": "no",
            "input_type_identifier": null,
            "return_type_identifier": null,
            "data_types": [],
            "settings": [],
            "starting_node": null
        }"#;

            let deserialized: Result<Flow, _> = serde_json::from_str(json_data);
            assert!(deserialized.is_ok());

            let expected_flow = Flow {
                flow_id: 0,
                project_id: 0,
                r#type: "no".to_string(),
                settings: vec![],
                data_types: vec![],
                input_type_identifier: None,
                return_type_identifier: None,
                starting_node: None,
            };

            assert_eq!(deserialized.unwrap(), expected_flow);
        }
    }
}

#[cfg(feature = "aquila")]
pub mod aquila {
    include!("generated/aquila.rs");
}

#[cfg(feature = "sagittarius")]
pub mod sagittarius {
    include!("generated/sagittarius.rs");
}
