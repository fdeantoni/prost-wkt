use std::env;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

use std::fs::File;
use std::io::Write;

use prost::Message;
use prost_types::FileDescriptorSet;

use regex::Regex;

fn main() {
    #[cfg(feature = "vendored-protoc")]
    std::env::set_var("PROTOC", protobuf_src::protoc());

    let dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    process_prost_pbtime(&dir);

    build(&dir, "pbtime");
    build(&dir, "pbstruct");
    build(&dir, "pbany");
    build(&dir, "pbempty");
    build(&dir, "pbmask");
}

fn build(dir: &Path, proto: &str) {
    let out = dir.join(proto);
    create_dir_all(&out).unwrap();
    let source = format!("proto/{proto}.proto");
    let descriptor_file = out.join("descriptors.bin");
    let mut prost_build = prost_build::Config::new();

    #[cfg(feature = "vendored-protox")]
    {
        let file_descriptors = protox::compile(&[source.clone()], &["proto/".to_string()]).unwrap();
        std::fs::write(&descriptor_file, file_descriptors.encode_to_vec()).unwrap();
        prost_build.skip_protoc_run();
    }

    prost_build
        .compile_well_known_types()
        .type_attribute(
            "google.protobuf.Duration",
            "#[derive(serde_derive::Serialize, serde_derive::Deserialize)] #[serde(default)]",
        )
        .type_attribute(
            "google.protobuf.Empty",
            "#[derive(serde_derive::Serialize, serde_derive::Deserialize)]",
        )
        .type_attribute(
            "google.protobuf.FieldMask",
            "#[derive(serde_derive::Serialize, serde_derive::Deserialize)]",
        )
        .file_descriptor_set_path(&descriptor_file)
        .out_dir(&out)
        .compile_protos(&[source], &["proto/".to_string()])
        .unwrap();

    let descriptor_bytes = std::fs::read(descriptor_file).unwrap();
    let descriptor = FileDescriptorSet::decode(&descriptor_bytes[..]).unwrap();

    prost_wkt_build::add_serde(out, descriptor);
}

fn process_prost_pbtime(dir: &Path) {
    process_prost_types_lib(dir);
    process_prost_types_datetime(dir);
}

fn process_prost_types_lib(dir: &Path) {
    let source: String = std::fs::read_to_string("./resources/lib.rs").unwrap();
    let lines: Vec<&str> = source.split('\n').collect();
    let selection = &lines[28..423];
    let mut string = String::new();
    string.push_str("mod datetime;");
    string.push('\n');
    for line in selection {
        string.push_str(line);
        string.push('\n');
    }

    let file = dir.join("prost_snippet.rs");
    File::create(file)
        .unwrap()
        .write_all(string.as_bytes())
        .unwrap();
}

fn process_prost_types_datetime(dir: &Path) {
    let source: String = std::fs::read_to_string("./resources/datetime.rs").unwrap();
    let lines: Vec<&str> = source.split('\n').collect();
    let selection = &lines[0..585];
    let mut string = String::new();
    for line in selection {
        string.push_str(line);
        string.push('\n');
    }

    let re = Regex::new(r"crate").unwrap();
    let result = re.replace_all(&string, "super").to_string();
    let file = dir.join("datetime.rs");
    File::create(file)
        .unwrap()
        .write_all(result.as_bytes())
        .unwrap();
}
