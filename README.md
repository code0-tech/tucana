# Tucana

This repository is responsible for all gRPC definitions that we use.

## Setup

See the setup guide for the following languages. Support for other languages is not planed!

### Rust

See: [Crate](https://crates.io/crates/tucana)

```toml
[dependencies]
tucana = { version = "<version>" }
```

To enable additional features::

```toml
[dependencies]
tucana = { version = "<version>", features = ["sagittarius", "aquila"] }
```

### Ruby

See: [Gem](https://rubygems.org/gems/tucana)

```ruby
gem 'tucana', '<version>'
```

Don't forget to initialize the required feature:
```ruby
# For Sagittarius
Tucana.load_protocol(:sagittarius)

# For Aquila
Tucana.load_protocol(:aquila)
```

### TypeScript/JavaScript

See: [NPM](https://www.npmjs.com/package/@code0-tech/tucana)

```console
npm i @code0-tech/tucana
```

## Project Structure

The project is organized with services functioning as servers. Each protocol in the Sagittarius folder corresponds to
services that Sagittarius must implement as a server.

```ascii-tree
.
├── aquila
│   ├── action - Action service (emits events, and handles executions)
│   ├── module - Module service for Taurus to send over datatypes, functions and flow types
│   ├── runtime_status - Service for runtime status (handles information about Draco and Taurus)
│   └── runtime_usage - Service for runtime usage (handles execution time of a flow)
├── sagittarius
│   ├── flow - Flow service (handles flow updates)
│   ├── module - Module service to receive datatypes, functions and flow types from aquila
│   ├── ping - Ping service (performs life checks)
│   ├── runtime_status - Service for runtime status (handles information about Draco and Taurus)
│   ├── runtime_usage - Service for runtime usage (handles execution time of a flow)
│   └── test_execution - Service and Types for the test execution
└── shared
    ├── data_type - Defines types for data types
    ├── errors - Defines error object
    ├── execution_result - Defines execution result of a flow
    ├── flow - Defines the flow to execute
    ├── flow_type - Defines types for flows based upon runtime flow types
    ├── runtime_flow_type - Defines runtime / primitive types for flows
    ├── function - Defines a function
    ├── module - Defines group of functions, runtime functions, flow types and data types
    ├── runtime_function - Defines a runtime function
    ├── runtime_status - Defines a runtime status (handles information about Draco and Taurus)
    ├── runtime_usage - Defines a runtime usage (handles execution time of a flow)
    ├── struct - Defines json representations
    └── translation - Contains translations with country codes and translated messages
```
