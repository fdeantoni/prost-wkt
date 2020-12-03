use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use prost_wkt::*;

#[derive(Clone, PartialEq, ::prost::Message, ::prost_wkt::MessageSerde, Serialize, Deserialize)]
#[prost(package="any.test")]
#[serde(default, rename_all="camelCase")]
pub struct Foo {
    #[prost(string, tag="1")]
    pub string: std::string::String,
    #[prost(message, optional, tag="2")]
    pub timestamp: ::std::option::Option<::prost_wkt::Timestamp>,
    #[prost(bool, tag="3")]
    pub boolean: bool,
    #[prost(message, optional, tag="4")]
    pub data: ::std::option::Option<::prost_wkt::Value>,
    #[prost(string, repeated, tag="5")]
    pub list: ::std::vec::Vec<std::string::String>,
    #[prost(message, optional, tag="6")]
    pub payload: ::std::option::Option<::prost_wkt::Any>,
}

fn create_struct() -> Value {
    let number: Value = Value::from(10.0);
    let null: Value = Value::null();
    let string: Value = Value::from(String::from("Hello"));
    let list = vec![Value::null(), Value::from(100.0)];
    let pb_list: Value = Value::from(list);
    let mut map: HashMap<String, Value> = HashMap::new();
    map.insert(String::from("number"), number);
    map.insert(String::from("null"), null);
    map.insert(String::from("string"), string);
    map.insert(String::from("list"), pb_list);
    Value::from(map)
}

#[test]
fn test_any_serialization() {
    
    let inner = Foo {
        string: String::from("inner"),
        timestamp: None,
        boolean: false,
        data: Some(create_struct()),
        list: vec!["een".to_string(), "twee".to_string()],
        payload: None
    };

    let msg = Foo {
        string: String::from("hello"),
        timestamp: Some(prost_wkt::Timestamp::new(99, 42)),
        boolean: true,
        data: Some(prost_wkt::Value::from("world".to_string())),
        list: vec!["one".to_string(), "two".to_string()],
        payload: Some(prost_wkt::Any::pack(inner))
    };
    println!("Serialized to string: {}", serde_json::to_string_pretty(&msg).unwrap());
    let erased = &msg as &dyn MessageSerde;
    let json = serde_json::to_string(erased).unwrap();
    println!("Erased json: {}", json);
}

#[test]
fn test_any_deserialize_string() {
    let data =
        r#"{
            "string":"hello",
            "timestamp":"1970-01-01T00:01:39.000000042Z",
            "boolean":true,
            "data": {
              "test_number": 1,
              "test_bool": true,
              "test_string": "hi there",
              "test_list": [1, 2, 3, 4],
              "test_inner_struct": {
                "one": 1,
                "two": 2
              }
            },
            "list": []
          }"#;
    let msg: Foo = serde_json::from_str(data).unwrap();
    println!("Deserialized from string: {:?}", msg);
}

#[test]
fn test_any_serialize_deserialize() {
    let inner = Foo {
        string: String::from("inner"),
        timestamp: None,
        boolean: false,
        data: None,
        list: vec!["een".to_string(), "twee".to_string()],
        payload: None
    };

    let original = Foo {
        string: String::from("original"),
        timestamp: Some(prost_wkt::Timestamp::new(99, 42)),
        boolean: true,
        data: Some(prost_wkt::Value::from("world".to_string())),
        list: vec!["one".to_string(), "two".to_string()],
        payload: Some(prost_wkt::Any::pack(inner))
    };

    let json = serde_json::to_string(&original).unwrap();
    println!("Serialized Foo: {}", json);
    let back: Foo = serde_json::from_str(&json).unwrap();
    println!("Deserialized Foo: {:?}", &back);
    assert_eq!(back, original)
}

#[test]
fn test_any_unpack() {
    let payload = Foo {
        string: String::from("hello payload"),
        timestamp: None,
        boolean: false,
        data: None,
        list: vec!["een".to_string(), "twee".to_string()],
        payload: None
    };
    let any = prost_wkt::Any::pack(payload);
    let unpacked = any.unpack().unwrap();
    let foo = unpacked.downcast_ref::<Foo>().unwrap();
    println!("Unpacked: {:?}", foo);
    assert_eq!(foo.string, "hello payload");
}