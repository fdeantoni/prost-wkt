use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(MessageSerde)]
pub fn message_serde_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        const _: () = {
            impl ::prost_wkt::MessageSerde for #name {
                fn type_url(&self) -> String { <#name as ::prost::Name>::type_url() }
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
                ::prost_wkt::MessageSerdeDecoderEntry::new::<#name>()
            }
        };
    };
    gen.into()
}
