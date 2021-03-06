# *PROST Well Known Types JSON Serialization and Deserialization* #

[Prost](https://github.com/danburkert/prost) is a [Protocol Buffers](https://developers.google.com/protocol-buffers/)
implementation for the [Rust Language](https://www.rust-lang.org/) that generates simple, idiomatic Rust code from 
`proto2` and `proto3` files.

It includes `prost-types` which gives basic support for protobuf Well-Known-Types (WKT), but support is basic. For 
example, it does not include packing or unpacking of messages in the `Any` type, nor much support in the way of JSON 
serialization and deserialization of that type.

This crate can help you if you need:
 - helper methods for packing and unpacking messages to/from an [Any](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Any),
 - helper methods for converting [chrono](https://github.com/chronotope/chrono) types to [Timestamp](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Timestamp) and back again,
 - helper methods for converting common rust types to [Value](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Value) and back again,
 - serde support for the types above.

To use it, include this crate along with prost:

```toml
[dependencies]
prost = { git = "https://github.com/fdeantoni/prost", branch = "meta" }
prost-wkt = { git = "https://github.com/fdeantoni/prost-wkt" }

[build-dependencies]
prost-build = { git = "https://github.com/fdeantoni/prost", branch = "meta" }
```

*Note*: the above uses a forked repo of prost with a pull request that `prost-wkt` requires. If this 
pull request gets accepted, the prost repo can be used instead.

In your `build.rs`, make sure to add the following options:
```rust
fn main() {
    let mut prost_build = prost_build::Config::new();
    prost_build
        .type_attribute(".","#[derive(::prost_wkt::MessageSerde, Serialize, Deserialize)] #[serde(default, rename_all=\"camelCase\")]")
        .extern_path(".google.protobuf.Any", "::prost_wkt::Any")
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt::Timestamp")
        .extern_path(".google.protobuf.Value", "::prost_wkt::Value")
        .compile_protos(
            &[
                "proto/messages.proto"
            ],
            &["proto/"],
        )
        .unwrap();
}
```

The above configuration will include `MessageSerde`, `Serialize`, and `Deserialize` on each generated struct. This will
allow you to use `serde` fully. Moreover, it ensures that the `Any` type is deserialized properly as JSON. For example, 
assume we have the following messages defined in our proto file:

```proto
syntax = "proto3";

import "google/protobuf/any.proto";
import "google/protobuf/timestamp.proto";

package my.messages;

message Request {
    string requestId = 1;
    google.protobuf.Any payload = 2;
}

message Foo {
    string data = 1;
    google.protobuf.Timestamp timestamp = 2;
}
```

After generating the rust structs for the above using `prost-build` with the above configuration, you will then be able
to do the following:

```rust
use serde::{Deserialize, Serialize};
use prost_wkt::*;
use chrono::prelude::*;

include!(concat!(env!("OUT_DIR"), "/my.messages.rs"));

fn main() {
    let mut foo: Foo = Foo::default();
    foo.data = "Hello World".to_string();
    foo.timestamp = Some(Utc::now().into());

    let mut request: Request = Request::default();
    let any = Any::pack(foo);
    request.request_id = "test1".to_string();
    request.payload = Some(any);

    let json = serde_json::to_string_pretty(&request).unwrap();
    println!("JSON:\n{}", json);

    let back: Request = serde_json::from_str(&json).unwrap();
    let unpacked: Box< dyn MessageSerde> = back.payload.unwrap().unpack().unwrap();
    let unpacked_foo: &Foo = unpacked.downcast_ref::<Foo>().unwrap();
    println!("Unpacked: {:?}", unpacked_foo);
}
```

The above will generate the following stdout:

```
JSON:
{
  "requestId": "test1",
  "payload": {
    "@type": "type.googleapis.com/my.messages.Foo",
    "data": "Hello World",
    "timestamp": "2020-05-25T12:19:57.755998Z"
  }
}
Unpacked: Foo { data: "Hello World", timestamp: Some(Timestamp { seconds: 1590409197, nanos: 755998000 }) }
```

Notice that the request message is properly serialized to JSON as per the [protobuf specification](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Any),
and that it can be deserialized as well.

See the `example` sub-project for a fully functioning example.

## Known Problems ##

Note that adding the `MessageSerde` derive will only work on messages that are converted to simple structs. It will
not work on protobuf `enum` and `oneOf` types. 

### Serde with Enum Types ### 
 
For the `enum` types, only add the `Serialize` and `Deserialize` derives. Do not add `MessageSerde`. For example, you 
can define your `build.rs` in the following way to ensure the `MessageSerde` derive is only added to `SomeMessage`, and
not to `SomeEnum`:
```rust
fn main() {
    let mut prost_build = prost_build::Config::new();
    prost_build
        .type_attribute(".my.messages.SomeEnum","#[derive(Serialize, Deserialize)]")
        .type_attribute(".my.messages.SomeMessage","#[derive(::prost_wkt::MessageSerde, Serialize, Deserialize)] #[serde(default, rename_all=\"camelCase\")]")
        .extern_path(".google.protobuf.Any", "::prost_wkt::Any")
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt::Timestamp")
        .extern_path(".google.protobuf.Value", "::prost_wkt::Value")
        .compile_protos(
            &[
                "proto/messages.proto"
            ],
            &["proto/"],
        )
        .unwrap();
}
```

In the above configuration we add `Serialize` and `Deserialize` to all, but `MessageSerde` only to `SomeMessage`.
 
### oneOf types ###

The way `prost-build` generates the `oneOf` type is to place it in a sub module, for example:

```proto
message SomeOne {
  oneof body {
    string some_string = 1;
    bool some_bool = 2;
    float some_float = 3;
  }
}
```

is converted to rust as follows:
```rust
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
#[prost(package="my.messages")]
pub struct SomeOne {
    #[prost(oneof="some_one::Body", tags="1, 2, 3")]
    pub body: ::core::option::Option<some_one::Body>,
}
/// Nested message and enum types in `SomeOne`.
pub mod some_one {
    #[derive(Serialize, Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Body {
        #[prost(string, tag="1")]
        SomeString(::prost::alloc::string::String),
        #[prost(bool, tag="2")]
        SomeBool(bool),
        #[prost(float, tag="3")]
        SomeFloat(f32),
    }
}
```

However, rust requires the importation of macros in each module, so each should have the following added:
```rust
use serde::{Serialize, Deserialize};
```

In the generated code snippet, the above statement is missing in the `some_one` module, and the rust compiler will 
complain about it. To fix it, we would have to add the appropriate use statement in the `some_one` module like so:
```rust
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
#[prost(package="my.messages")]
pub struct SomeOne {
    #[prost(oneof="some_one::Body", tags="1, 2, 3")]
    pub body: ::core::option::Option<some_one::Body>,
}
/// Nested message and enum types in `SomeOne`.
pub mod some_one {
    use serde::{Serialize, Deserialize};
    #[derive(Serialize, Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Body {
        #[prost(string, tag="1")]
        SomeString(::prost::alloc::string::String),
        #[prost(bool, tag="2")]
        SomeBool(bool),
        #[prost(float, tag="3")]
        SomeFloat(f32),
    }
}
```

Luckily, you can achieve the above by tweaking the `build.rs` again. The configuration below, for example, will add the 
required serde import to the `some_one` module as needed:
```rust
fn main() {
    let mut prost_build = prost_build::Config::new();
    prost_build
        .type_attribute(".my.messages.MyEnum","#[derive(Serialize, Deserialize)]")
        .type_attribute(".my.messages.MyMessage","#[derive(::prost_wkt::MessageSerde, Serialize, Deserialize)] #[serde(default, rename_all=\"camelCase\")]")
        .type_attribute(".my.messages.SomeOne.body","use serde::{Serialize, Deserialize}; #[derive(Serialize, Deserialize)]")
        .extern_path(".google.protobuf.Any", "::prost_wkt::Any")
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt::Timestamp")
        .extern_path(".google.protobuf.Value", "::prost_wkt::Value")
        .compile_protos(
            &[
                "proto/messages.proto"
            ],
            &["proto/"],
        )
        .unwrap();
}
```

## License ##

`prost-wkt` is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](LICENSE) for details.

Copyright 2020 Ferdinand de Antoni
