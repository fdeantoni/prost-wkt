pub use inventory;

pub use const_format;
pub use erased_serde;
pub use prost_wkt_derive::MessageSerde;

/// Trait to support serialization and deserialization of `prost` messages.
pub trait MessageSerde: prost::Message + std::any::Any + erased_serde::Serialize {
    /// the message proto type url e.g. type.googleapis.com/my.package.MyMessage
    fn type_url(&self) -> String;
    /// Returns the encoded protobuf message as bytes
    fn try_encoded(&self) -> Result<Vec<u8>, prost::EncodeError>;
    /// Returns an erased serialize dynamic reference
    fn as_erased_serialize(&self) -> &dyn erased_serde::Serialize;
}

/// The implementation here is a direct copy of the `impl dyn` of [`std::any::Any`]!
impl dyn MessageSerde {
    /// Returns `true` if the inner type is the same as `T`.
    #[inline]
    pub fn is<T: MessageSerde>(&self) -> bool {
        // Get `TypeId` of the type this function is instantiated with.
        let t = std::any::TypeId::of::<T>();

        // Get `TypeId` of the type in the trait object (`self`).
        let concrete = self.type_id();

        // Compare both `TypeId`s on equality.
        t == concrete
    }

    /// Returns some reference to the inner value if it is of type `T`, or
    /// `None` if it isn't.
    #[inline]
    pub fn downcast_ref<T: MessageSerde>(&self) -> Option<&T> {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented Any for all types; no other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(self.downcast_ref_unchecked()) }
        } else {
            Option::None
        }
    }

    /// Returns some mutable reference to the boxed value if it is of type `T`,
    /// or `None` if it isn't.
    #[inline]
    pub fn downcast_mut<T: MessageSerde>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented Any for all types; no other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(self.downcast_mut_unchecked()) }
        } else {
            Option::None
        }
    }

    /// Returns a reference to the inner value as type `dyn T`.
    ///
    /// # Safety
    ///
    /// The contained value must be of type `T`. Calling this method
    /// with the incorrect type is *undefined behavior*.
    #[inline]
    pub unsafe fn downcast_ref_unchecked<T: MessageSerde>(&self) -> &T {
        debug_assert!(self.is::<T>());
        // SAFETY: caller guarantees that T is the correct type
        unsafe { &*(self as *const dyn MessageSerde as *const T) }
    }

    /// Returns a mutable reference to the inner value as type `dyn T`.
    ///
    /// # Safety
    ///
    /// The contained value must be of type `T`. Calling this method
    /// with the incorrect type is *undefined behavior*.
    #[inline]
    pub unsafe fn downcast_mut_unchecked<T: MessageSerde>(&mut self) -> &mut T {
        &mut *(self as *mut Self as *mut T)
    }
}

type MessageSerdeTypeUrlFn = fn() -> String;
type MessageSerdeDecoderFn = fn(&[u8]) -> Result<Box<dyn MessageSerde>, ::prost::DecodeError>;
type MessageSerdeDeserializerFn =
    fn(&mut dyn erased_serde::Deserializer) -> Result<Box<dyn MessageSerde>, erased_serde::Error>;

pub struct MessageSerdeDecoderEntry {
    pub type_url: MessageSerdeTypeUrlFn,
    pub decoder: MessageSerdeDecoderFn,
    pub deserializer: MessageSerdeDeserializerFn,
}

impl MessageSerdeDecoderEntry {
    pub const fn new<M>() -> Self
    where
        for<'a> M: MessageSerde + prost::Name + Default + serde::Deserialize<'a>,
    {
        Self {
            type_url: <M as prost::Name>::type_url,
            decoder: |buf| {
                let msg: M = prost::Message::decode(buf)?;
                Ok(Box::new(msg))
            },
            deserializer: |de| erased_serde::deserialize::<M>(de).map(|v| Box::new(v) as _),
        }
    }
}

inventory::collect!(MessageSerdeDecoderEntry);
