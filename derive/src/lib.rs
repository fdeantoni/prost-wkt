extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use anyhow::{bail, Error};
use syn::{DeriveInput, Ident, Meta, Attribute, MetaList, NestedMeta, Data};
use quote::quote;

fn try_message_serde(input: DeriveInput) -> Result<TokenStream, Error> {

    let ident = input.ident;

    let pkg_name = prost_attrs(input.attrs.clone()).unwrap().iter().find( |meta| {
        meta.path().is_ident("package")
    }).and_then( |meta| {
        match meta {
            Meta::NameValue(v) => match &v.lit {
                syn::Lit::Str(lit) => Some(lit.value().clone()),
                _ => None
            },
            _ => None
        }
    }).unwrap_or_else( || String::from("prost"));

    let type_url = format!(
        "type.googleapis.com/{}.{}",
        pkg_name,
        ident
    );

    let dummy_const = Ident::new(&format!("IMPL_MESSAGE_SERDE_FOR_{}", ident), Span::call_site());
    let serde = quote! {
            const #dummy_const: () = {
                use ::prost_wkt::*;
                #[typetag::serde(name=#type_url)]
                impl ::prost_wkt::MessageSerde for #ident {
                    fn new_instance(&self, data: Vec<u8>) -> Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError> {
                        let mut target = Self::default();
                        ::prost::Message::merge(&mut target, data.as_slice())?;
                        let erased: Box<::prost_wkt::MessageSerde> = Box::new(target);
                        Ok(erased)
                    }
                    fn encoded(&self) -> Vec<u8> {
                        let mut buf = Vec::new();
                        buf.reserve(::prost::Message::encoded_len(self));
                        ::prost::Message::encode(self, &mut buf).unwrap();
                        buf
                    }
                }
            };
        };

    Ok(serde.into())
}

// Copied from https://github.com/danburkert/prost/blob/master/prost-derive/src/field/mod.rs
fn prost_attrs(attrs: Vec<Attribute>) -> Result<Vec<Meta>, Error> {
    Ok(attrs
        .iter()
        .flat_map(Attribute::parse_meta)
        .flat_map(|meta| match meta {
            Meta::List(MetaList { path, nested, .. }) => {
                if path.is_ident("prost") {
                    nested.into_iter().collect()
                } else {
                    Vec::new()
                }
            }
            _ => Vec::new(),
        })
        .flat_map(|attr| -> Result<_, _> {
            match attr {
                NestedMeta::Meta(attr) => Ok(attr),
                NestedMeta::Lit(lit) => bail!("invalid prost attribute: {:?}", lit),
            }
        })
        .collect())
}

#[proc_macro_derive(MessageSerde, attributes(prost))]
pub fn message_serde(input: TokenStream) -> TokenStream {
    let di: DeriveInput = syn::parse(input.clone()).unwrap();
    match di.data {
        Data::Struct(_) => try_message_serde(di).unwrap(),
        _ => input, // not struct so do not process further
    }
}



