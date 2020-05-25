[![Dependency Status](https://deps.rs/repo/github/fdeantoni/prost-wkt/status.svg)](https://deps.rs/repo/github/fdeantoni/prost-wkt)

# *PROST Well Known Types JSON Serialization and Deserialization* #

[Prost](https://github.com/danburkert/prost) is a [Protocol Buffers](https://developers.google.com/protocol-buffers/)
implementation for the [Rust Language](https://www.rust-lang.org/) that generates simple, idiomatic Rust code from 
`proto2` and `proto3` files.

It includes `prost-types` which gives basic support for protobuf Well-Known-Types (WKT). As support is basic, it
does not include packing or unpacking of messages in the `Any` type, nor much support in the way of JSON serialization
and deserialization.

This is a helper crate that can be used on top of prost to provide the above missing features.

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

In your `bulid.rs`, make sure to add the following options:
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
use prost::*;
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

See the `example` sub-project for a fully functioning example.

## License ##

`prost-wkt` is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](LICENSE) for details.

Copyright 2020 Ferdinand de Antoni
