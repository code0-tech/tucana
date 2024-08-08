use std::io::Result;

fn main() -> Result<()> {
    tonic_build::configure()
        .out_dir("src/internal")
        .build_server(true)
        .build_client(true)
        .type_attribute("Variable", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("RuleType", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Rule", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Type", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Parameter", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Node", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Flow", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(&[
            "variable.proto",
            "rule.proto",
            "type.proto",
            "node.proto",
            "flow.proto",
            "ping.proto",
        ], &["../../internal"])
        .expect("Cannot compile protos");

    Ok(())
}