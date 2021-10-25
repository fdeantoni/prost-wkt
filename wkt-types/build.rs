use std::env;
use std::fs::create_dir_all;
use std::path::PathBuf;

use prost::Message;
use prost_types::FileDescriptorSet;

fn main() {
    let dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    build(&dir, "pbtime");
    build(&dir, "pbstruct");
    build(&dir, "pbany");
}

fn build(dir: &PathBuf, proto: &str) {
    let out = dir.join(proto);
    create_dir_all(&out).unwrap();
    let source = format!("proto/{}.proto", proto);
    let descriptor_file = out.join("descriptors.bin");
    let mut prost_build = prost_build::Config::new();
    prost_build
        .compile_well_known_types()
        .type_attribute("google.protobuf.Struct","#[derive(serde_derive::Serialize, serde_derive::Deserialize)] #[serde(default, rename_all=\"camelCase\")]")
        .type_attribute("google.protobuf.ListValue","#[derive(serde_derive::Serialize, serde_derive::Deserialize)] #[serde(default, rename_all=\"camelCase\")]")
        .type_attribute("google.protobuf.Duration","#[derive(serde_derive::Serialize, serde_derive::Deserialize)] #[serde(default, rename_all=\"camelCase\")]")
        .file_descriptor_set_path(&descriptor_file)
        .out_dir(&out)
        .compile_protos(
            &[
                source
            ],
            &["proto/".to_string()],
        )
        .unwrap();

    let descriptor_bytes = std::fs::read(descriptor_file).unwrap();
    let descriptor = FileDescriptorSet::decode(&descriptor_bytes[..]).unwrap();

    prost_wkt_build::add_serde(out, descriptor);
}
