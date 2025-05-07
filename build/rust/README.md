# Tucana - Protobuf gRPC Interface for Rust

[![Crate](https://img.shields.io/crates/v/tucana.svg)](https://crates.io/crates/tucana)
[![Documentation](https://docs.rs/tucana/badge.svg)](https://docs.rs/tucana)

The Rust Code0 gRPC library (for internal service communication) providing interfaces for internal service communication along with helper utilities for working with the generated types.

## Features

- **aquila** - gRPC services and types for Aquila (as server) component communication
- **sagittarius** - gRPC services and types for Sagittarius (as server) component communication

## Overview

Tucana serves as the interface layer for internal service communication:

1. Generated Protobuf types for service messages (Flow, DataType, NodeFunction, etc.)
2. gRPC service definitions for inter-component communication
3. Helper utilities for manipulating Protobuf-generated types

## Core Functionality

### Protobuf Message Handling

The library provides utilities for working with the Protobuf-generated types, especially the Value and Struct (JSON representations) types used throughout the services.

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
