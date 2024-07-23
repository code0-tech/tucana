use std::io::Result;

fn main() -> Result<()> {
    let internal_prefix = "../../internal";

    let paths = &[
        format!("{internal_prefix}/configuration.proto"),
        format!("{internal_prefix}/flow.proto"),
        format!("{internal_prefix}/node.proto"),
        format!("{internal_prefix}/rule.proto"),
        format!("{internal_prefix}/type.proto"),
        format!("{internal_prefix}/variable.proto")
    ];

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .protoc_arg("--proto_path=../../internal")
        .compile(paths, &["src/lib.rs"])
        .expect("Cannot compile protos");

    Ok(())
}