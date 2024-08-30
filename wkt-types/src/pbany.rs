use prost_wkt::MessageSerde;
use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};

include!(concat!(env!("OUT_DIR"), "/pbany/google.protobuf.rs"));

use prost::{DecodeError, EncodeError, Message, Name};
use serde_value::ValueDeserializer;

use std::{borrow::Cow, fmt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnyError {
    description: Cow<'static, str>,
}

impl AnyError {
    pub fn new<S>(description: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        AnyError {
            description: description.into(),
        }
    }
}

impl std::error::Error for AnyError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for AnyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("failed to convert Value: ")?;
        f.write_str(&self.description)
    }
}

impl From<prost::DecodeError> for AnyError {
    fn from(error: DecodeError) -> Self {
        AnyError::new(format!("Error decoding message: {error:?}"))
    }
}

impl From<prost::EncodeError> for AnyError {
    fn from(error: prost::EncodeError) -> Self {
        AnyError::new(format!("Error encoding message: {error:?}"))
    }
}

impl Any {
    //#[deprecated(since = "0.5.0", note = "please use `from_msg` instead")]
    /// Packs a message into an `Any` containing a `type_url` which will take the format
    /// of `type.googleapis.com/package_name.struct_name`, and a value containing the
    /// encoded message.
    pub fn try_pack<T>(message: T) -> Result<Self, AnyError>
    where
        T: Message + MessageSerde + Default,
    {
        let type_url = MessageSerde::type_url(&message).to_string();
        // Serialize the message into a value
        let mut buf = Vec::with_capacity(message.encoded_len());
        message.encode(&mut buf)?;
        let encoded = Any {
            type_url,
            value: buf,
        };
        Ok(encoded)
    }

    //#[deprecated(since = "0.5.0", note = "please use `to_msg` instead")]
    /// Unpacks the contents of the `Any` into the provided message type. Example usage:
    ///
    /// ```ignore
    /// let back: Foo = any.unpack_as(Foo::default())?;
    /// ```
    pub fn unpack_as<T: Message>(self, mut target: T) -> Result<T, AnyError> {
        let instance = target.merge(self.value.as_slice()).map(|_| target)?;
        Ok(instance)
    }

    /// Unpacks the contents of the `Any` into the `MessageSerde` trait object. Example
    /// usage:
    ///
    /// ```ignore
    /// let back: Box<dyn MessageSerde> = any.try_unpack()?;
    /// ```
    pub fn try_unpack(self) -> Result<Box<dyn prost_wkt::MessageSerde>, AnyError> {
        ::prost_wkt::inventory::iter::<::prost_wkt::MessageSerdeDecoderEntry>
            .into_iter()
            .find(|entry| self.type_url == entry.type_url)
            .ok_or_else(|| format!("Failed to deserialize {}. Make sure prost-wkt-build is executed.", self.type_url))
            .and_then(|entry| {
                (entry.decoder)(&self.value).map_err(|error| {
                    format!(
                        "Failed to deserialize {}. Make sure it implements prost::Message. Error reported: {}",
                        self.type_url,
                        error
                    )
                })
            })
            .map_err(AnyError::new)
    }

    /// From Prost's [`Any`] implementation.
    /// Serialize the given message type `M` as [`Any`].
    pub fn from_msg<M>(msg: &M) -> Result<Self, EncodeError>
    where
        M: Name,
    {
        let type_url = M::type_url();
        let mut value = Vec::new();
        Message::encode(msg, &mut value)?;
        Ok(Any { type_url, value })
    }

    /// From Prost's [`Any`] implementation.
    /// Decode the given message type `M` from [`Any`], validating that it has
    /// the expected type URL.
    #[allow(clippy::all)]
    pub fn to_msg<M>(&self) -> Result<M, DecodeError>
    where
        M: Default + Name + Sized,
    {
        let expected_type_url = M::type_url();

        match (
            TypeUrl::new(&expected_type_url),
            TypeUrl::new(&self.type_url),
        ) {
            (Some(expected), Some(actual)) => {
                if expected == actual {
                    return Ok(M::decode(&*self.value)?);
                }
            }
            _ => (),
        }

        let mut err = DecodeError::new(format!(
            "expected type URL: \"{}\" (got: \"{}\")",
            expected_type_url, &self.type_url
        ));
        err.push("unexpected type URL", "type_url");
        Err(err)
    }
}

impl Serialize for Any {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Any", 3)?;
        state.serialize_field("@type", &self.type_url)?;
        match self.clone().try_unpack() {
            Ok(result) => {
                state.serialize_field("value", result.as_erased_serialize())?;
            }
            Err(_) => {
                state.serialize_field("value", &self.value)?;
            }
        }
        state.end()
    }
}

struct AnyVisitor;

impl<'de> Visitor<'de> for AnyVisitor {
    type Value = Box<dyn MessageSerde>;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("type.googleapis.com/google.protobuf.any")
    }
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut cached_type_url: Option<String> = None;
        let mut cached_value: Option<serde_value::Value> = None;
        while let Some(key) = map.next_key::<String>()? {
            match &*key {
                "@type" => {
                    if cached_type_url.is_some() {
                        return Err(serde::de::Error::duplicate_field("@type"));
                    }
                    cached_type_url.replace(map.next_value()?);
                }
                "value" => {
                    if cached_value.is_some() {
                        return Err(serde::de::Error::duplicate_field("value"));
                    }
                    cached_value.replace(map.next_value()?);
                }
                _ => return Err(serde::de::Error::unknown_field(&key, &["@type", "value"])),
            };
        }
        let type_url = cached_type_url.ok_or_else(|| serde::de::Error::missing_field("@type"))?;
        let raw_value = cached_value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
        let entry = ::prost_wkt::inventory::iter::<::prost_wkt::MessageSerdeDecoderEntry>
            .into_iter()
            .find(|entry| type_url == entry.type_url)
            .ok_or_else(|| {
                serde::de::Error::invalid_type(
                    serde::de::Unexpected::Str(&type_url),
                    &"a typeurl registered by deriving SerdeMessage" as &dyn serde::de::Expected,
                )
            })?;
        let mut deserializer =
            <dyn erased_serde::Deserializer>::erase(ValueDeserializer::<A::Error>::new(raw_value));
        (entry.deserializer)(&mut deserializer).map_err(|err| serde::de::Error::custom(err))
    }
}

impl<'de> Deserialize<'de> for Any {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let erased = deserializer.deserialize_struct("Any", &["@type", "value"], AnyVisitor)?;
        let type_url = erased.type_url().to_string();
        let value = erased.try_encoded().map_err(|err| {
            serde::de::Error::custom(format!("Failed to encode message: {err:?}"))
        })?;
        Ok(Any { type_url, value })
    }
}

/// URL/resource name that uniquely identifies the type of the serialized protocol buffer message,
/// e.g. `type.googleapis.com/google.protobuf.Duration`.
///
/// This string must contain at least one "/" character.
///
/// The last segment of the URL's path must represent the fully qualified name of the type (as in
/// `path/google.protobuf.Duration`). The name should be in a canonical form (e.g., leading "." is
/// not accepted).
///
/// If no scheme is provided, `https` is assumed.
///
/// Schemes other than `http`, `https` (or the empty scheme) might be used with implementation
/// specific semantics.
#[derive(Debug, Eq, PartialEq)]
struct TypeUrl<'a> {
    /// Fully qualified name of the type, e.g. `google.protobuf.Duration`
    full_name: &'a str,
}

impl<'a> TypeUrl<'a> {
    fn new(s: &'a str) -> core::option::Option<Self> {
        // Must contain at least one "/" character.
        let slash_pos = s.rfind('/')?;

        // The last segment of the URL's path must represent the fully qualified name
        // of the type (as in `path/google.protobuf.Duration`)
        let full_name = s.get((slash_pos + 1)..)?;

        // The name should be in a canonical form (e.g., leading "." is not accepted).
        if full_name.starts_with('.') {
            return None;
        }

        Some(Self { full_name })
    }
}

#[cfg(test)]
mod tests {
    use crate::pbany::*;
    use serde_derive::*;
    use serde_json::json;

    #[derive(
        Clone, Eq, PartialEq, ::prost::Message, Serialize, Deserialize, ::prost_wkt::MessageSerde,
    )]
    #[serde(default, rename_all = "camelCase")]
    pub struct Foo {
        #[prost(string, tag = "1")]
        pub string: std::string::String,
    }

    impl Name for Foo {
        const NAME: &'static str = "Foo";
        const PACKAGE: &'static str = "any.test";
    }

    #[test]
    fn pack_unpack_test() {
        let msg = Foo {
            string: "Hello World!".to_string(),
        };
        let any = Any::try_pack(msg.clone()).unwrap();
        println!("{any:?}");
        let unpacked = any.unpack_as(Foo::default()).unwrap();
        println!("{unpacked:?}");
        assert_eq!(unpacked, msg)
    }

    #[test]
    fn pack_unpack_with_downcast_test() {
        let msg = Foo {
            string: "Hello World!".to_string(),
        };
        let any = Any::try_pack(msg.clone()).unwrap();
        println!("{any:?}");
        let unpacked: &dyn MessageSerde = &any.unpack_as(Foo::default()).unwrap();
        let downcast = unpacked.downcast_ref::<Foo>().unwrap();
        assert_eq!(downcast, &msg);
    }

    #[test]
    fn deserialize_default_test() {
        let type_url = "type.googleapis.com/any.test.Foo";
        let data = json!({
            "@type": type_url,
            "value": {}
        });
        let any: Any = serde_json::from_value(data).unwrap();
        let foo: Foo = any.to_msg().unwrap();
        println!("Deserialize default: {foo:?}");
        assert_eq!(foo, Foo::default())
    }

    #[test]
    fn check_prost_any_serialization() {
        let message = crate::Timestamp::date(2000, 1, 1).unwrap();
        let any = Any::from_msg(&message).unwrap();
        assert_eq!(
            &any.type_url,
            "type.googleapis.com/google.protobuf.Timestamp"
        );

        let message2 = any.to_msg::<crate::Timestamp>().unwrap();
        assert_eq!(message, message2);

        // Wrong type URL
        assert!(any.to_msg::<crate::Duration>().is_err());
    }
}
