use std::env;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

fn main() {
    #[cfg(feature = "vendored-protoc")]
    std::env::set_var("PROTOC", protobuf_src::protoc());

    let dir = PathBuf::from(env::var("OUT_DIR").unwrap());

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
        let file_descriptors = protox::compile([source.clone()], ["proto/".to_string()]).unwrap();
        std::fs::write(&descriptor_file, file_descriptors.encode_to_vec()).unwrap();
        prost_build.skip_protoc_run();
    }

    prost_build
        .compile_well_known_types()
        .enable_type_names()
        .type_name_domain(&["."], "type.googleapis.com")
        .type_attribute(
            "google.protobuf.Empty",
            "#[derive(serde_derive::Serialize, serde_derive::Deserialize)]",
        )
        .type_attribute(
            "google.protobuf.FieldMask",
            "#[derive(serde_derive::Serialize, serde_derive::Deserialize)]",
        )
        .message_attribute(".", "#[derive(::prost_wkt::MessageSerde)]")
        .file_descriptor_set_path(&descriptor_file)
        .out_dir(&out)
        .compile_protos(&[source], &["proto/".to_string()])
        .unwrap();
}
