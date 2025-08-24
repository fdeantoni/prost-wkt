use chrono::prelude::*;

use prost_wkt_types::*;
use serde_json::json;

include!(concat!(env!("OUT_DIR"), "/my.messages.rs"));
include!(concat!(env!("OUT_DIR"), "/my.requests.rs"));

fn main() -> Result<(), AnyError> {

    let content: Content = Content { body: Some(content::Body::SomeBool(true)) };

    let foo_msg: Foo = Foo {
        data: "Hello World".to_string(),
        timestamp: Some(Utc::now().into()),
        content: Some(content),
        flavor: Flavor::Chocolate.into(),
    };

    let mut request: Request = Request::default();
    let any = Any::try_pack(foo_msg)?;
    request.request_id = "test1".to_string();
    request.payload = Some(any);

    let json = serde_json::to_string_pretty(&request).expect("Failed to serialize request");

    println!("JSON:\n{json}");

    let back: Request = serde_json::from_str(&json).expect("Failed to deserialize request");

    if let Some(payload) = back.payload {
        let unpacked: Box<dyn MessageSerde> = payload.try_unpack()?;
        let unpacked_foo: &Foo = unpacked
            .downcast_ref::<Foo>()
            .expect("Failed to downcast message");
        println!("Unpacked: {unpacked_foo:?}");
    }

    let foo_with_string_flavor = json!({
        "data": "Hello World",
        "timestamp": "2025-04-21T04:52:27.654981Z",
        "content": {
            "body": {
                "someBool": true
            }
        },
        "flavor": "CHOCOLATE"
    });

    match serde_json::from_value::<Foo>(foo_with_string_flavor) {
        Ok(value) => println!("Deserialized with string enum: {value:?}"),
        Err(error) => eprintln!("Failed to deserialize with string num: {error:?}"),
    }

    Ok(())
}
