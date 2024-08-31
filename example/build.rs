use std::{env, path::PathBuf};

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let descriptor_file = out.join("descriptors.bin");
    let mut prost_build = prost_build::Config::new();
    prost_build
        .enable_type_names()
        .type_name_domain(&[".my.requests", ".my.messages"], "type.googleapis.com")
        .type_attribute(
            ".my.requests",
            "#[derive(serde::Serialize, serde::Deserialize, ::prost_wkt::MessageSerde)] #[serde(default, rename_all=\"camelCase\")]",
        )
        .type_attribute(
            ".my.messages.Foo",
            "#[derive(serde::Serialize, serde::Deserialize)] #[serde(default, rename_all=\"camelCase\")]",
        )
        .message_attribute(".my.messages.Foo", "#[derive(::prost_wkt::MessageSerde)]")
        .type_attribute(
            ".my.messages.Content",
            "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\")]",
        )
        .message_attribute(".my.messages.Content", "#[derive(::prost_wkt::MessageSerde)]")
        .extern_path(".google.protobuf.Any", "::prost_wkt_types::Any")
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
        .extern_path(".google.protobuf.Value", "::prost_wkt_types::Value")
        .file_descriptor_set_path(&descriptor_file)
        .compile_protos(&["proto/messages.proto", "proto/requests.proto"], &["proto/"])
        .unwrap();
}
