//! `prost-wkt` adds helper methods to deal with protobuf well known types.

mod pbtime;
pub use crate::pbtime::*;

mod pbstruct;
pub use crate::pbstruct::*;

mod pbany;
pub use crate::pbany::*;

#[macro_use]
extern crate mopa;

pub use typetag;

#[allow(unused_imports)]
#[macro_use]
extern crate prost_wkt_derive;
#[doc(hidden)]
pub use prost_wkt_derive::*;

/// Trait to support serialization and deserialization of `prost` messages. This trait
/// is implemented by the `MessageSerde` derive macro.
#[typetag::serde(tag = "@type")]
pub trait MessageSerde: prost::Message + prost::MessageDescriptor + mopa::Any {
    /// Creates a new instance of this message using the protobuf encoded data
    fn new_instance(&self, data: Vec<u8>) -> Result<Box<dyn MessageSerde>, prost::DecodeError>;
    /// Returns the encoded protobuf message as bytes
    fn encoded(&self) -> Vec<u8>;
}

mopafy!(MessageSerde);
