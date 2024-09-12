use prost::Name;
use prost_wkt::*;
use prost_wkt_types::*;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize, MessageSerde)]
#[prost(package = "any.test")]
#[serde(rename_all = "camelCase")]
pub struct Foo {
    #[prost(string, tag = "1")]
    pub string: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::std::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(bool, tag = "3")]
    pub boolean: bool,
    #[prost(message, optional, tag = "4")]
    pub value_data: ::std::option::Option<::prost_wkt_types::Value>,
    #[prost(string, repeated, tag = "5")]
    pub list: ::std::vec::Vec<std::string::String>,
    #[prost(message, optional, tag = "6")]
    pub payload: ::std::option::Option<::prost_wkt_types::Any>,
}

impl Name for Foo {
    const PACKAGE: &'static str = "any.test";
    const NAME: &'static str = "Foo";

    fn type_url() -> String {
        "type.googleapis.com/any.test.Foo".to_string()
    }
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
        value_data: Some(create_struct()),
        list: vec!["een".to_string(), "twee".to_string()],
        payload: None,
    };

    let msg = Foo {
        string: String::from("hello"),
        timestamp: Some(prost_wkt_types::Timestamp {
            seconds: 99,
            nanos: 42,
        }),
        boolean: true,
        value_data: Some(prost_wkt_types::Value::from("world".to_string())),
        list: vec!["one".to_string(), "two".to_string()],
        payload: prost_wkt_types::Any::try_pack(inner).ok(),
    };
    println!(
        "Serialized to string: {}",
        serde_json::to_string_pretty(&msg).unwrap()
    );
    let any = Any::from_msg(&msg).unwrap();
    let json = serde_json::to_string(&any).unwrap();
    println!("Erased json: {json}");
}

#[test]
fn test_any_deserialize_string() {
    let data = r#"{
            "string":"hello",
            "timestamp":"1970-01-01T00:01:39.000000042Z",
            "boolean":true,
            "valueData": {
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
    println!("Deserialized from string: {msg:?}");
    let serialized = serde_json::to_string_pretty(&msg).expect("serialized Foo");
    println!("{serialized}")
}

#[test]
fn test_any_serialize_deserialize() {
    let inner = Foo {
        string: String::from("inner"),
        timestamp: None,
        boolean: false,
        value_data: None,
        list: vec!["een".to_string(), "twee".to_string()],
        payload: None,
    };

    let original = Foo {
        string: String::from("original"),
        timestamp: Some(prost_wkt_types::Timestamp {
            seconds: 99,
            nanos: 42,
        }),
        boolean: true,
        value_data: Some(prost_wkt_types::Value::from("world".to_string())),
        list: vec!["one".to_string(), "two".to_string()],
        payload: prost_wkt_types::Any::try_pack(inner).ok(),
    };

    let json = serde_json::to_string(&original).unwrap();
    println!("Serialized Foo: {json}");
    let back: Foo = serde_json::from_str(&json).unwrap();
    println!("Deserialized Foo: {:?}", &back);
    assert_eq!(back, original)
}

#[test]
#[allow(clippy::disallowed_names)]
fn test_any_unpack() {
    let payload = Foo {
        string: String::from("hello payload"),
        timestamp: None,
        boolean: false,
        value_data: None,
        list: vec!["een".to_string(), "twee".to_string()],
        payload: None,
    };
    let any = prost_wkt_types::Any::try_pack(payload).unwrap();
    let unpacked = any.try_unpack().unwrap();
    let foo = unpacked.downcast_ref::<Foo>().unwrap();
    println!("Unpacked: {foo:?}");
    assert_eq!(foo.string, "hello payload");
}
