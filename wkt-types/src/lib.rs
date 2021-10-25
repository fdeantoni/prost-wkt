//! `prost-wkt` adds helper methods to deal with protobuf well known types.

mod pbtime;
pub use crate::pbtime::*;

mod pbstruct;
pub use crate::pbstruct::*;

mod pbany;
pub use crate::pbany::*;

pub use prost_wkt::MessageSerde;