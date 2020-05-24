use std::env;
use std::fs::create_dir_all;

fn main() {

    let dir: String = env::var_os("OUT_DIR").unwrap().to_str().unwrap().to_string();

    build(&dir, "pbtime");
    build(&dir, "pbstruct");
    build(&dir, "pbany");
}

fn build(dir: &str, proto: &str) {
    let out = format!("{}/{}", dir, proto);
    create_dir_all(&out).unwrap();
    let source = format!("proto/{}.proto", proto);
    let mut prost_build = prost_build::Config::new();
    prost_build
        .compile_well_known_types()
        .out_dir(&out)
        .compile_protos(
            &[
                source
            ],
            &["proto/".to_string()],
        )
        .unwrap();
}



