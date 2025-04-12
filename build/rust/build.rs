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
        "sagittarius.ping.proto",
        "sagittarius.runtime_function.proto",
        // shared
        "shared.datatype.proto",
        "shared.flow.proto",
        "shared.runtime_function.proto",
        "shared.translation.proto",
        "shared.struct.proto",
        "shared.event.proto",
    ];

    let inclusions = &[
        "../../proto/shared",
        "../../proto/sagittarius",
        "../../proto/aquila",
    ];

    let out_path = "src/generated";
    let serde_attribute = "#[derive(serde::Serialize, serde::Deserialize)]";

    if !std::path::Path::new(&out_path).exists() {
        match create_dir(out_path) {
            Err(error) => panic!("Cannot create the `generated` folder! Reason: {:?}", error),
            _ => {}
        };
    }

    let build_result = tonic_build::configure()
        .out_dir(out_path)
        .build_server(true)
        .build_client(true)
        .type_attribute("kind", serde_attribute)
        .type_attribute("NullValue", serde_attribute)
        .type_attribute("Value", serde_attribute)
        .type_attribute("ListValue", serde_attribute)
        .type_attribute("Struct", serde_attribute)
        .type_attribute("Translation", serde_attribute)
        .type_attribute("DataTypeRule", serde_attribute)
        .type_attribute("DataType", serde_attribute)
        .type_attribute("RuntimeParameterDefinition", serde_attribute)
        .type_attribute("RuntimeFunctionDefinition", serde_attribute)
        .type_attribute("FlowSettingDefinition", serde_attribute)
        .type_attribute("FlowSetting", serde_attribute)
        .type_attribute("NodeParameterDefinition", serde_attribute)
        .type_attribute("NodeFunctionDefinition", serde_attribute)
        .type_attribute("NodeParameter", serde_attribute)
        .type_attribute("NodeFunction", serde_attribute)
        .type_attribute("Flow", serde_attribute)
        .type_attribute("Flows", serde_attribute)
        .compile_protos(proto, inclusions);

    match build_result {
        Ok(_) => Ok(()),
        Err(error) => panic!("Cannot build the proto files! Reason: {:?}", error),
    }
}
