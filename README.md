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
tucana = { version = "<version>", features = ["sagittarius_rails", "aquila"] }
```

### Ruby

See: [Gem](https://rubygems.org/gems/tucana)

```ruby
gem 'tucana', '<version>'
```

Don't forget to initialize the required feature:
```ruby
# For Sagittarius
Tucana.load_protocol(:sagittarius_rails)

# For Aquila
Tucana.load_protocol(:aquila)
```

### TypeScript/JavaScript

See: [NPM](https://www.npmjs.com/package/@code0-tech/tucana)

```console
npm i @code0-tech/tucana
```

### Python

See: [PyPI](https://pypi.org/project/tucana/)

```console
pip install tucana
```

## Project Structure

The project is organized with services functioning as servers. Each protocol in the Aquila folder corresponds to
services that Aquila must implement as a server.

> Notice: Sagittarius has been splitted into a service without and with streams regarding issues due to the thread blocking implementation of Ruby gRPC

```ascii-tree
.
├── aquila
│   ├── action - Action service (emits events, and handles executions)
│   ├── module - Module service for Taurus to send over datatypes, functions and flow types
│   ├── runtime_status - Service for runtime status (handles information about Draco and Taurus)
│   └── runtime_usage - Service for runtime usage (handles execution time of a flow)
├── sagittarius_gateway
│   ├── flow - Flow service (handles flow updates)
│   ├── module - Module service to receive datatypes, functions and flow types from aquila
│   ├── runtime_status - Service for runtime status (handles information about Draco and Taurus)
│   ├── runtime_usage - Service for runtime usage (handles execution time of a flow)
│   └── test_execution - Service and Types for the test execution├── sagittarius
├── sagittarius_rails
│   ├── flow - Flow service (handles flow updates)
│   ├── module - Module service to receive datatypes, functions and flow types from aquila
│   ├── runtime_status - Service for runtime status (handles information about Draco and Taurus)
│   ├── runtime_usage - Service for runtime usage (handles execution time of a flow)
│   ├── test_execution - Service and Types for the test execution
│   └── token - Service for verifing a runtime token
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
