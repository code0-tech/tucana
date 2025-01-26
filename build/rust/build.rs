use std::fs::create_dir;
use std::io::Result;

fn main() -> Result<()> {
    let proto = &[
        // aquila
        "aquila.action.proto",
        "aquila.execution.proto",
        // sagittarius
        "sagittarius.action.proto",
        "sagittarius.datatype.proto",
        "sagittarius.flow.proto",
        "sagittarius.flow_definition.proto",
        "sagittarius.node.proto",
        "sagittarius.ping.proto",
        // shared
        "shared.datatype.proto",
        "shared.runtime_function.proto",
        "shared.translation.proto",
        "shared.event.proto"
    ];

    let inclusions = &[
        "../../proto/shared",
        "../../proto/sagittarius",
        "../../proto/aquila",
    ];

    let out_path = "src/generated";

    if !std::path::Path::new(&out_path).exists() {
        create_dir(out_path)?;
    }

    tonic_build::configure()
        .out_dir(out_path)
        .build_server(true)
        .build_client(true)
        .type_attribute("FlowDefinition", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("RuntimeFunctionDefinition", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("RuntimeParameterDefinition", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Parameter", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Node", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Flow", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(proto, inclusions)
        .expect("Cannot compile internal protos");

    Ok(())
}
