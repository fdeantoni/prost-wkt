use prost_wkt_build::*;
use std::{env, path::PathBuf};

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let descriptor_file = out.join("descriptors.bin");
    let mut prost_build = prost_build::Config::new();
    prost_build
        .type_attribute(
            ".my.requests",
            "#[derive(serde::Serialize, serde::Deserialize)] #[serde(default, rename_all=\"camelCase\")]",
        )
        .type_attribute(
            ".my.messages.Foo",
            "#[derive(serde::Serialize, serde::Deserialize)] #[serde(default, rename_all=\"camelCase\")]",
        )
        .type_attribute(
            ".my.messages.Content",
            "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\")]",
        )
        .extern_path(".google.protobuf.Any", "::prost_wkt_types::Any")
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
        .extern_path(".google.protobuf.Value", "::prost_wkt_types::Value")
        .file_descriptor_set_path(&descriptor_file)
        .compile_protos(&["proto/messages.proto", "proto/requests.proto"], &["proto/"])
        .unwrap();

    let descriptor_bytes = std::fs::read(descriptor_file).unwrap();
    let descriptor = FileDescriptorSet::decode(&descriptor_bytes[..]).unwrap();

    prost_wkt_build::add_serde(out, descriptor);
}
