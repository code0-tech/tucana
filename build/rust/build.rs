use std::io::Result;

fn main() -> Result<()> {
    let paths = &[
        "flow.proto",
        "node.proto",
        "rule.proto",
        "type.proto",
        "variable.proto"
    ];

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .type_attribute("Variable", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("RuleType", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Rule", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Type", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Parameter", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Node", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Flow", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(paths, &["../../internal"])
        .expect("Cannot compile protos");

    Ok(())
}