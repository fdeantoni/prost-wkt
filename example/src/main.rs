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