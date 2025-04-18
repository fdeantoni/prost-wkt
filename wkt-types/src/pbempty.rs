include!(concat!(env!("OUT_DIR"), "/pbempty/google.protobuf.rs"));

const EMPTY: Empty = Empty {};

impl From<()> for Empty {
    fn from(_value: ()) -> Self {
        EMPTY
    }
}

#[cfg(feature = "schemars")]
mod schemars_impl {
    use super::Empty;
    use std::borrow::Cow;
    use schemars::JsonSchema;
    use schemars::gen::SchemaGenerator;
    use schemars::schema::{InstanceType, Schema, SchemaObject};

    impl JsonSchema for Empty {
        fn schema_name() -> String {
            "Empty".to_string()
        }

        fn schema_id() -> Cow<'static, str> {
            Cow::Borrowed("prost_wkt_types::Empty")
        }

        fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
            let mut schema = SchemaObject {
                instance_type: Some(InstanceType::Object.into()),
                ..Default::default()
            };

            schema.metadata().description = Some("Represents an empty message".to_string());

            Schema::Object(schema)
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::pbempty::*;

    #[test]
    fn serialize_empty() {
        let msg = EMPTY;
        println!(
            "Serialized to string: {}",
            serde_json::to_string_pretty(&msg).unwrap()
        );
    }

    #[test]
    fn deserialize_empty() {
        let msg: Empty = serde_json::from_str("{}").expect("Could not deserialize `{}` to an Empty struct!");
        assert_eq!(msg, EMPTY);
    }

    #[test]
    fn convert_unit() {
        let msg: Empty = ().into();
        assert_eq!(msg, Empty {});
    }
}
