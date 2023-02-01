# Circuit V2 proto3 vs proto2 Compatibility Tests

Given the following proto2 file, what's the proto3 equivalent that is backwards compatible?

```proto
syntax = "proto2";

package p2;

message HopMessage {
  enum Type {
    RESERVE = 0;
    CONNECT = 1;
    STATUS = 2;
  }

  required Type type = 1;

  optional Peer peer = 2;
  optional Reservation reservation = 3;
  optional Limit limit = 4;

  optional Status status = 5;
}

message StopMessage {
  enum Type {
    CONNECT = 0;
    STATUS = 1;
  }

  required Type type = 1;

  optional Peer peer = 2;
  optional Limit limit = 3;

  optional Status status = 4;
}

message Peer {
  required bytes id = 1;
  repeated bytes addrs = 2;
}

message Reservation {
  required uint64 expire = 1; // Unix expiration time (UTC)
  repeated bytes addrs = 2;   // relay addrs for reserving peer
  optional bytes voucher = 3; // reservation voucher
}

message Limit {
  optional uint32 duration = 1; // seconds
  optional uint64 data = 2;     // bytes
}

enum Status {
  OK                      = 100;
  RESERVATION_REFUSED     = 200;
  RESOURCE_LIMIT_EXCEEDED = 201;
  PERMISSION_DENIED       = 202;
  CONNECTION_FAILED       = 203;
  NO_RESERVATION          = 204;
  MALFORMED_MESSAGE       = 400;
  UNEXPECTED_MESSAGE      = 401;
}
```

Considerations: 

- We have to be careful with `required` fields since the message will fail to
parse if they are missing from the wire encoding. We can make sure proto3 always
serializes this by setting `optional` in proto3.
- Enums need zero values. So we need to add a new zero value for Status

Proto3 version:

```proto
syntax = "proto3";

package p3;

message HopMessage {
  enum Type {
    RESERVE = 0;
    CONNECT = 1;
    STATUS = 2;
  }

  // This field is marked optional for backwards compatibility with proto2. Users should make sure to always set this.
  optional Type type = 1;

  optional Peer peer = 2;
  optional Reservation reservation = 3;
  optional Limit limit = 4;

  optional Status status = 5;
}

message StopMessage {
  enum Type {
    CONNECT = 0;
    STATUS = 1;
  }

  // This field is marked optional for backwards compatibility with proto2. Users should make sure to always set this.
  optional Type type = 1;

  optional Peer peer = 2;
  optional Limit limit = 3;

  optional Status status = 4;
}

message Peer {
  // This field is marked optional for backwards compatibility with proto2. Users should make sure to always set this.
  optional bytes id = 1;
  repeated bytes addrs = 2;
}

message Reservation {
  // This field is marked optional for backwards compatibility with proto2. Users should make sure to always set this.
  optional uint64 expire = 1; // Unix expiration time (UTC)
  repeated bytes addrs = 2;   // relay addrs for reserving peer
  optional bytes voucher = 3; // reservation voucher
}

message Limit {
  optional uint32 duration = 1; // seconds
  optional uint64 data = 2;     // bytes
}

enum Status {
  // zero value field required for proto3 compatibility
  UNUSED             = 0;
  OK                      = 100;
  RESERVATION_REFUSED     = 200;
  RESOURCE_LIMIT_EXCEEDED = 201;
  PERMISSION_DENIED       = 202;
  CONNECTION_FAILED       = 203;
  NO_RESERVATION          = 204;
  MALFORMED_MESSAGE       = 400;
  UNEXPECTED_MESSAGE      = 401;
}
```

The diff:

```diff
1c1
< syntax = "proto2";
---
> syntax = "proto3";
3c3
< package p2;
---
> package p3;
12c12,13
<   required Type type = 1;
---
>   // This field is marked optional for backwards compatibility with proto2. Users should make sure to always set this.
>   optional Type type = 1;
27c28,29
<   required Type type = 1;
---
>   // This field is marked optional for backwards compatibility with proto2. Users should make sure to always set this.
>   optional Type type = 1;
36c38,39
<   required bytes id = 1;
---
>   // This field is marked optional for backwards compatibility with proto2. Users should make sure to always set this.
>   optional bytes id = 1;
41c44,45
<   required uint64 expire = 1; // Unix expiration time (UTC)
---
>   // This field is marked optional for backwards compatibility with proto2. Users should make sure to always set this.
>   optional uint64 expire = 1; // Unix expiration time (UTC)
51a56,57
>   // zero value field required for proto3 compatibility
>   UNUSED             = 0;
```

# The experiment

To make sure we aren't missing any edge cases, this repo includes an experiment
that generates random messages, then parses those messages to make sure they
mean the same thing whether interpreted as a proto2 or proto3 message.

The generation Uses Rust + Prost to generate 10k random `Hop` and `Stop`
messages. These are saved in the files `randomHopMsgs.bin` `randomStopMsgs.bin`.
The format is a varint prefix followed by the protobuf message.

The random messages are then parsed as both proto2 and proto3 messages. The
structs are inspected to make sure each value matches across definitions. 

Results are also checked in CI.