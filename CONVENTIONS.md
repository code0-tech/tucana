# Conventions

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

> Don't nest messages within messages.

*Don't*

```protobuf
message A {
  message B {
    int64 b = 1;
  }

  message C {
    int64 c = 1;
  }

  B b = 1;
  C c = 2;
}
```

*Do*

```protobuf

message B {
  int64 b = 1;
}

message C {
  int64 c = 1;
}

message A {
  B b = 1;
  C c = 2;
}
```
---

### Enum

> [_In proto3, the first value defined in an enum definition must have the value zero and should have the name ENUM_TYPE_NAME_UNSPECIFIED or ENUM_TYPE_NAME_UNKNOWN_](https://protobuf.dev/programming-guides/proto3/#enum-default)

In our case we just name it `UNKNOWN`

```
enum Variant {
  UNKNOWN = 0;
  PRIMITIVE = 1;
  TYPE = 2;
  OBJECT = 3;
  DATATYPE = 4;
  ARRAY = 5;
  ERROR = 6;
  NODE = 7;
}
```

Nesting for Enums is recomended due to Enum value scoping rules.

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
