# Naming Conventions

This document outlines the required naming conventions for Protocol Buffers (Protobuf) files, messages, services, and folder structures.

---

## Messages

Use PascalCase (capitalized camel case) for message names.
Use snake_case (lower case with _) for field names.

### Example:

```protobuf
message HelloWorld {
  string value_one = 1;
  int64 value_two = 2;
}
```
---
## Services

For service names, use PascalCase for both the service and RPC method names. Each service definition should clearly describe the action it performs.

### Example: 

```protobuf
message PingRequest {
  int64 ping = 1;
}

message PongResponse {
  int64 pong = 1;
}

service PingService {
  rpc PingPong (PingRequest) returns (PongResponse) {}
}
```
---
## Folder Structure
To maintain an organized file structure, each service should have its own folder. File names should include the service name as a prefix to prevent conflicts when multiple services share the same method names.
The file of a service should always contain the service name as its prefix. This is to make it possible to have duplicate file names without a build error!

### Folder Structure Example:
```ascii-tree 
.
├── service_one (folder)
│   ├── service_one.file_one.proto (file)
│   └── service_one.file_two.proto (file)
└── service_two (folder)
    ├── service_two.file_one.proto (file)
    └── service_two.file_two.proto (file)
```

## File Header
Keep the following structure for the header of a proto file!

`syntax` -> we use proto3 (version 3)
`option ruby_package` -> required for ruby build
`package` -> required for rust build & proto itself
`imports` required inputs

### Example:
```protobuf
syntax = "proto3";

option ruby_package = "Tucana::Shared";

package shared;

import "google/protobuf/struct.proto";
```