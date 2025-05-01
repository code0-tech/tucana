# Tucana - Protobuf gRPC Interface for Rust

[![Crate](https://img.shields.io/crates/v/tucana.svg)](https://crates.io/crates/tucana)
[![Documentation](https://docs.rs/tucana/badge.svg)](https://docs.rs/tucana)

A Rust library providing Protobuf gRPC interfaces for internal service communication along with helper utilities for working with the generated types.

## Features

This crate provides modular functionality through feature flags:

- **shared** - Core data types and helper utilities for working with Protobuf messages (enabled by default)
- **aquila** - gRPC services and types for Aquila component communication
- **sagittarius** - gRPC services and types for Sagittarius component communication

## Overview

Tucana serves as the interface layer for internal service communication:

1. Generated Protobuf types for service messages (Flow, DataType, NodeFunction, etc.)
2. gRPC service definitions for inter-component communication
3. Helper utilities for manipulating Protobuf-generated types

## Core Functionality

### Protobuf Message Handling

The library provides utilities for working with the Protobuf-generated types, especially the Value and Struct types used throughout the services.

### Path-based Value Access

Access nested values in complex Protobuf message structures using dot notation:

```rust
use tucana::shared::helper::path;
use tucana::shared::Value;

// Get values of specific types from Protobuf messages
let flow_type = path::get_string("type", &flow_value);
let project_id = path::get_number("project_id", &flow_value);
let has_starting_node = path::exists_path("starting_node", &flow_value);

// Update values in a message
let updated_flow = path::set_value("input_type_identifier", &flow_value, new_type_value);
```

### JSON Conversion for Protobuf Messages

Convert between Protobuf messages and JSON for debugging or external interfaces:

```rust
use tucana::shared::helper::value;
use tucana::shared::Flow;
use serde_json;

// Create a Flow message
let flow = Flow {
    flow_id: 42,
    project_id: 100,
    r#type: "process".to_string(),
    data_types: vec![],
    input_type_identifier: Some("String".to_string()),
    return_type_identifier: Some("Number".to_string()),
    settings: vec![],
    starting_node: None,
};

// Serialize to JSON
let flow_json = serde_json::to_string(&flow).expect("Serialization failed");

// Parse back from JSON
let parsed_flow: Flow = serde_json::from_str(&flow_json).expect("Deserialization failed");
```

## Service Communication

The library contains the generated Protobuf interfaces for the internal microservices:

```rust
// Import the service needed for your component
#[cfg(feature = "aquila")]
use tucana::aquila::AquilaServiceClient;

#[cfg(feature = "sagittarius")]
use tucana::sagittarius::SagittariusServiceClient;
```

## Advanced Usage

### Working with Complex Protobuf Messages

```rust
use tucana::shared::helper::path;
use tucana::shared::DataType;
use tucana::shared::data_type::Variant;

// Create a data type
let data_type = DataType {
    variant: Variant::Primitive as i32,
    identifier: "string".to_string(),
    name: vec![],
    rules: vec![],
    input_types: vec![],
    return_type: None,
    parent_type_identifier: None,
};

// Use helper functions to work with nested messages
let struct_value = path::get_struct("settings.display", &flow_value);
let rules_list = path::get_list("data_types.0.rules", &flow_value);
```

## Examples

### Manipulating Flow Messages

```rust
use tucana::shared::{Flow, NodeFunction, Value};
use tucana::shared::value::Kind;
use tucana::shared::helper::{path, value};
use serde_json;

// Create a Flow message
let flow = Flow {
    flow_id: 1,
    project_id: 2,
    r#type: "standard".to_string(),
    data_types: vec![],
    input_type_identifier: Some("String".to_string()),
    return_type_identifier: None,
    settings: vec![],
    starting_node: None,
};

// Serialize to JSON (for external API or storage)
let json_str = serde_json::to_string(&flow).expect("Serialization failed");

// Parse back from JSON
let parsed_flow: Flow = serde_json::from_str(&json_str).expect("Deserialization failed");

// Convert complex nested values
let json_config = serde_json::json!({
    "flow": {
        "name": "My Flow",
        "version": 1.0,
        "active": true
    }
});
let proto_value = value::from_json_value(json_config);

// Access and modify nested properties
let flow_name = path::get_string("flow.name", &proto_value);
let is_active = path::get_bool("flow.active", &proto_value);
```
