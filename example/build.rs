fn main() {
    let mut prost_build = prost_build::Config::new();
    prost_build
        .type_attribute(".","#[derive(::prost_wkt::MessageSerde, Serialize, Deserialize)] #[serde(rename-all = \"snake_case\")]")
        .extern_path(".google.protobuf.Any", "::prost_wkt::Any")
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt::Timestamp")
        .extern_path(".google.protobuf.Value", "::prost_wkt::Value")
        .compile_protos(
            &[
                "proto/messages.proto"
            ],
            &["proto/"],
        )
        .unwrap();
}