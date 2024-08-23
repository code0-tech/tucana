use std::fs::create_dir;
use std::io::Result;

fn main() -> Result<()> {

    let proto = &[
        "definitions.proto",
        "variable.proto",
        "rule.proto",
        "type.proto",
        "node.proto",
        "flow.proto",
        "action.proto",
        "transfer.proto"
    ];

    let inclusions = &[
        "../../shared",
        "../../internal",
        "../../external",
    ];

    let out_path = "src/generated";

    if !std::path::Path::new(&out_path).exists() {
        create_dir(out_path)?;
    }

    tonic_build::configure()
        .out_dir(out_path)
        .build_server(true)
        .build_client(true)
        .type_attribute("Variable", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("RuleType", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Rule", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Type", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Parameter", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Node", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Flow", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(proto, inclusions)
        .expect("Cannot compile internal protos");

    Ok(())
}