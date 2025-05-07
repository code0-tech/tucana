use std::fs::create_dir;
use std::io::Result;

fn read_proto_folders(path: &str) -> (Vec<String>, Vec<String>) {
    let mut proto_folders: Vec<String> = Vec::new();
    let mut proto_files: Vec<String> = Vec::new();

    let proto_folder = match std::fs::read_dir(path) {
        Ok(entries) => entries,
        Err(error) => panic!("Cannot read the `proto` folder! Reason: {:?}", error),
    };

    for entry in proto_folder {
        let real_entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let meta = match real_entry.metadata() {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };

        if meta.is_dir() {
            let new_path = match real_entry.path().into_os_string().into_string() {
                Ok(path) => path,
                Err(_) => continue,
            };

            proto_folders.push(new_path.clone());
            let (new_folders, new_files) = read_proto_folders(new_path.as_str());

            proto_folders.extend(new_folders);
            proto_files.extend(new_files);
        } else {
            let file_name = match real_entry.file_name().into_string() {
                Ok(name) => name,
                Err(_) => continue,
            };

            proto_files.push(file_name);
        }
    }

    (proto_folders, proto_files)
}

fn main() -> Result<()> {
    let path = "../../proto";
    let (proto_folders, proto_files) = read_proto_folders(path);

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
        .type_attribute("value", serde_attribute)
        .type_attribute("config", serde_attribute)
        .type_attribute("ReferencePath", serde_attribute)
        .type_attribute("ReferenceValue", serde_attribute)
        .type_attribute("NullValue", serde_attribute)
        .type_attribute("Value", serde_attribute)
        .type_attribute("ListValue", serde_attribute)
        .type_attribute("Struct", serde_attribute)
        .type_attribute("Translation", serde_attribute)
        .type_attribute("DataTypeInputType", serde_attribute)
        .type_attribute("DataTypeInputTypesRuleConfig", serde_attribute)
        .type_attribute("DataTypeReturnTypeRuleConfig", serde_attribute)
        .type_attribute("DataTypeContainsKeyRuleConfig", serde_attribute)
        .type_attribute("DataTypeContainsTypeRuleConfig", serde_attribute)
        .type_attribute("DataTypeItemOfCollectionRuleConfig", serde_attribute)
        .type_attribute("DataTypeNumberRangeRuleConfig", serde_attribute)
        .type_attribute("DataTypeRegexRuleConfig", serde_attribute)
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
        .compile_protos(&proto_files, &proto_folders);

    match build_result {
        Ok(_) => Ok(()),
        Err(error) => panic!("Cannot build the proto files! Reason: {:?}", error),
    }
}
