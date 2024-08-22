use std::fs::create_dir;
use std::io::Result;

fn main() -> Result<()> {

    let internal_path = "src/internal";
    let external_path = "src/external";

    let internal_proto = &[
        "variable.proto",
        "rule.proto",
        "type.proto",
        "node.proto",
        "flow.proto",
    ];

    let external_proto = &[
        "definitions.proto",
        "action.proto",
        "transfer.proto"
    ];

    if !std::path::Path::new(&internal_path).exists() {
        create_dir(internal_path)?;
    }

    if !std::path::Path::new(&external_path).exists() {
        create_dir(external_path)?;
    }

    tonic_build::configure()
        .out_dir(internal_path)
        .build_server(true)
        .build_client(true)
        .type_attribute("Variable", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("RuleType", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Rule", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Type", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Parameter", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Node", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Flow", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(internal_proto, &["../../internal"])
        .expect("Cannot compile internal protos");

    tonic_build::configure()
        .out_dir(external_path)
        .build_server(true)
        .build_client(true)
        .compile(external_proto, &["../../external"])
        .expect("Cannot compile external protos");

    Ok(())
}