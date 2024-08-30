use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(MessageSerde)]
pub fn message_serde_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        const _: () = {
            const TYPE_URL: &str = ::prost_wkt::const_format::concatcp!(
                "type.googleapis.com/",
                <#name as ::prost::Name>::PACKAGE,
                ".",
                <#name as ::prost::Name>::NAME,
            );

            impl ::prost_wkt::MessageSerde for #name {
                fn package_name(&self) -> &'static str {
                    <Self as ::prost::Name>::PACKAGE
                }
                fn message_name(&self) -> &'static str {
                    <Self as ::prost::Name>::NAME
                }
                fn type_url(&self) -> &'static str {
                    TYPE_URL
                }
                fn new_instance(&self, data: Vec<u8>) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError> {
                    let mut target = Self::default();
                    ::prost::Message::merge(&mut target, data.as_slice())?;
                    let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> = ::std::boxed::Box::new(target);
                    Ok(erased)
                }
                fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
                    let mut buf = ::std::vec::Vec::with_capacity(::prost::Message::encoded_len(self));
                    ::prost::Message::encode(self, &mut buf)?;
                    Ok(buf)
                }
                fn as_erased_serialize(&self) -> &dyn ::prost_wkt::erased_serde::Serialize {
                    self
                }
            }

            ::prost_wkt::inventory::submit!{
                ::prost_wkt::MessageSerdeDecoderEntry {
                    type_url: TYPE_URL,
                    decoder: |buf: &[u8]| {
                        let msg: #name = ::prost::Message::decode(buf)?;
                        Ok(::std::boxed::Box::new(msg))
                    },
                    deserializer: |de: &mut dyn ::prost_wkt::erased_serde::Deserializer| {
                        ::prost_wkt::erased_serde::deserialize::<#name>(de)
                            .map(|v| Box::new(v) as Box<dyn ::prost_wkt::MessageSerde>)
                    },
                }
            }
        };
    };
    gen.into()
}
