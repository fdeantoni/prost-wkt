use heck::ToUpperCamelCase;
use quote::{format_ident, quote};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub use prost::Message;
pub use prost_types::FileDescriptorSet;

use prost_build::Module;

pub struct SerdeOptions {
    type_url_generator: Box<dyn Fn(&str, &str) -> String + 'static>,
    type_attributes: Vec<(String, String)>,
}

pub fn add_serde(out: PathBuf, descriptor: FileDescriptorSet) {
    add_serde_with_options(out, descriptor, SerdeOptions::default())
}

pub fn add_serde_with_options(out: PathBuf, descriptor: FileDescriptorSet, options: SerdeOptions) {
    for fd in &descriptor.file {
        let package_name = match fd.package {
            Some(ref pkg) => pkg,
            None => continue,
        };

        let rust_path = out
            .join(Module::from_protobuf_package_name(package_name).to_file_name_or(package_name));

        // In some cases the generated file would be in empty. These files are no longer created by Prost, so
        // we'll create here. Otherwise we append.
        let mut rust_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(rust_path)
            .unwrap();

        for msg in &fd.message_type {
            let message_name = match msg.name {
                Some(ref name) => name,
                None => continue,
            };

            let type_url = (options.type_url_generator)(package_name, message_name);

            gen_trait_impl(
                &mut rust_file,
                package_name,
                message_name,
                &type_url,
                &options.type_attributes,
            );
        }
    }
}

// This method uses the `heck` crate (the same that prost uses) to properly format the message name
// to UpperCamelCase as the prost_build::ident::{to_snake, to_upper_camel} methods
// in the `ident` module of prost_build is private.
fn gen_trait_impl(
    rust_file: &mut File,
    package_name: &str,
    message_name: &str,
    type_url: &str,
    type_attributes: &[(String, String)],
) {
    let type_name = message_name.to_upper_camel_case();
    let type_name = format_ident!("{}", type_name);
    let dotted_path = format!(".{}.{}", package_name, message_name);
    let bare_path = format!("{}.{}", package_name, message_name);
    let custom_attributes: Vec<&str> = type_attributes
        .iter()
        .filter(|(path, _)| *path == dotted_path || *path == bare_path)
        .map(|(_, attribute)| attribute.as_str())
        .collect();

    let tokens = quote! {
        #[allow(dead_code)]
        const _: () = {
            use ::prost_wkt::typetag;
            #[typetag::serde(name=#type_url)]
            impl ::prost_wkt::MessageSerde for #type_name {
                fn package_name(&self) -> &'static str {
                    #package_name
                }
                fn message_name(&self) -> &'static str {
                    #message_name
                }
                fn type_url(&self) -> &'static str {
                    #type_url
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
            }

            ::prost_wkt::inventory::submit!{
                ::prost_wkt::MessageSerdeDecoderEntry {
                    type_url: #type_url,
                    decoder: |buf: &[u8]| {
                        let msg: #type_name = ::prost::Message::decode(buf)?;
                        Ok(::std::boxed::Box::new(msg))
                    }
                }
            }

            impl ::prost::Name for #type_name {
                const PACKAGE: &'static str = #package_name;
                const NAME: &'static str = #message_name;

                fn type_url() -> String {
                    #type_url.to_string()
                }
            }
        };
    };

    for attr in &custom_attributes {
        writeln!(rust_file, "{}", attr).unwrap();
    }
    writeln!(rust_file).unwrap();
    writeln!(rust_file, "{}", &tokens).unwrap();
}

impl Default for SerdeOptions {
    fn default() -> Self {
        Self {
            type_url_generator: Box::new(|package, message| {
                format!("type.googleapis.com/{}.{}", package, message)
            }),
            type_attributes: Vec::new(),
        }
    }
}

impl SerdeOptions {
    /// Add a custom attribute to the `MessageSerde` impl generated for the given type.
    ///
    /// The `path` is the fully-qualified protobuf type path, with or without a leading
    /// dot (e.g. `google.protobuf.Timestamp` or `.google.protobuf.Timestamp`). The
    /// `attribute` is inserted verbatim before the generated impl block.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use prost_wkt_build::SerdeOptions;
    /// let options = SerdeOptions::default()
    ///     .type_attribute("google.protobuf.Timestamp", "#[allow(dead_code)]");
    /// # let _ = options;
    /// ```
    pub fn type_attribute(
        mut self,
        path: impl AsRef<str>,
        attribute: impl AsRef<str>,
    ) -> Self {
        self
            .type_attributes
            .push((path.as_ref().to_string(), attribute.as_ref().to_string()));
        self
    }

    /// Set a custom type url generator.
    ///
    /// The generator is a function that takes a package name and a message name and returns a type url.
    /// I.e by default the type url is will be `type.googleapis.com/{package}.{message}` but you can change it to anything you want according to your needs.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use prost_wkt_build::SerdeOptions;
    /// let options = SerdeOptions::default().with_custom_type_url_generator(|package, message| format!("my.custom.type.url/{}.{}", package, message));
    /// ```
    ///
    ///
    pub fn with_custom_type_url_generator<F: Fn(&str, &str) -> String + 'static>(
        mut self,
        generator: F,
    ) -> Self {
        self.type_url_generator = Box::new(generator);
        self
    }
}
