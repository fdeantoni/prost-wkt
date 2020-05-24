mod pbtime;
pub use crate::pbtime::*;

mod pbstruct;
pub use crate::pbstruct::*;

mod pbany;
pub use crate::pbany::*;

#[macro_use]
extern crate mopa;

pub use typetag;


#[typetag::serde(tag = "@type")]
pub trait MessageSerde: prost::Message + prost::MessageProto + mopa::Any {
    fn new_instance(&self, data: Vec<u8>) -> Result<Box<dyn MessageSerde>, prost::DecodeError>;
    fn encoded(&self) -> Vec<u8>;
}

mopafy!(MessageSerde);
