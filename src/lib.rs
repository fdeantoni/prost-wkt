#[macro_use]
extern crate mopa;

pub use typetag;

/// Trait to support serialization and deserialization of `prost` messages.
#[typetag::serde(tag = "@type")]
pub trait MessageSerde: prost::Message + mopa::Any {
    /// message name as in proto file
    fn message_name(&self) -> &'static str;
    /// package name as in proto file
    fn package_name(&self) -> &'static str;
    /// the message proto type url e.g. type.googleapis.com/my.package.MyMessage
    fn type_url(&self) -> &'static str;
    /// Creates a new instance of this message using the protobuf encoded data
    fn new_instance(&self, data: Vec<u8>) -> Result<Box<dyn MessageSerde>, prost::DecodeError>;
    /// Returns the encoded protobuf message as bytes
    fn encoded(&self) -> Vec<u8>;
}

mopafy!(MessageSerde);
