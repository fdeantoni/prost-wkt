use std::env;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

use std::fs::File;
use std::io::Write;

use prost::Message;
use prost_types::FileDescriptorSet;

fn main() {

    let dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    download_prost_pbtime(&dir);

    build(&dir, "pbtime");
    build(&dir, "pbstruct");
    build(&dir, "pbany");
}

fn build(dir: &Path, proto: &str) {
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

fn download_prost_pbtime(dir: &Path) {
    let url = "https://raw.githubusercontent.com/tokio-rs/prost/v0.9.0/prost-types/src/lib.rs";
    let resp = reqwest::blocking::get(url).unwrap().text().unwrap();
    let lines: Vec<String> = resp.lines().map(|s| s.to_string()).collect();
    let selection = &lines[27..256];
    let mut string = String::new();
    for line in selection {
        string.push_str(line);
        string.push('\n');
    }

    let file = dir.join("prost_snippet.rs");
    File::create(file).unwrap().write_all(string.as_bytes()).unwrap();
}
