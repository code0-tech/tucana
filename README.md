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

## Project Structure

The project is organized with services functioning as servers. Each protocol in the Sagittarius folder corresponds to
services that Sagittarius must implement as a server.

```ascii-tree
.
├── aquila
│   ├── action - Action service (emits events, manages configs, and handles executions)
│   ├── data_type - DataType service
│   ├── execution - Execution service (handles internal execution calls)
│   ├── flow_definition - FlowType service (handles flow_type updates)
│   └── runtime_function - Service for updating the runtime functions
├── sagittarius
│   ├── action - Action service (manages logon/logoff requests for action configurations)
│   ├── data_type - DataType service
│   ├── flow - Flow service (handles flow updates)
│   ├── flow_definition - FlowType service (handles flow_type updates)
│   ├── ping - Ping service (performs life checks)
│   ├── runtime_function - Service for updating the runtime functions
│   └── test_execution - Service and Types for the test execution
└── shared
    ├── data_type - Defines types for data types
    ├── event - Defines types for events
    ├── flow - Defines types for flows
    ├── flow_definition - Defines a definition for a Flow
    ├── runtime_function_definition - Defines types for runtime functions
    ├── struct - Defines types for json representations
    └── translation - Contains translations with country codes and translated messages
```
