use std::io::Result;

fn main() -> Result<()> {
    let internal_prefix = "../../internal";

    let paths = &[
        format!("{internal_prefix}/flow.proto"),
        format!("{internal_prefix}/node.proto"),
        format!("{internal_prefix}/rule.proto"),
        format!("{internal_prefix}/type.proto"),
        format!("{internal_prefix}/variable.proto")
    ];

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .type_attribute("Variable", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Node", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Parameter", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Rule", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("RuleType", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Type", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Flow", "#[derive(serde::Serialize, serde::Deserialize)]")
        .protoc_arg("--proto_path=../../internal")
        .compile(paths, &["src/lib.rs"])
        .expect("Cannot compile protos");

    Ok(())
}