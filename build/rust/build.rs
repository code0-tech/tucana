use std::fs::{self, create_dir};
use std::io::Result;
use std::path::{Path, PathBuf};

fn read_proto_folders(path: &str) -> (Vec<String>, Vec<String>) {
    let mut proto_folders: Vec<String> = Vec::new();
    let mut proto_files: Vec<String> = Vec::new();

    let proto_folder = std::fs::read_dir(path)
        .unwrap_or_else(|e| panic!("Cannot read the `proto` folder! Reason: {:?}", e));

    for entry in proto_folder {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        if meta.is_dir() {
            let new_path = entry
                .path()
                .into_os_string()
                .into_string()
                .unwrap_or_default();
            if new_path.is_empty() {
                continue;
            }

            proto_folders.push(new_path.clone());
            let (new_folders, new_files) = read_proto_folders(&new_path);
            proto_folders.extend(new_folders);
            proto_files.extend(new_files);
        } else if entry
            .path()
            .extension()
            .map(|e| e == "proto")
            .unwrap_or(false)
        {
            let file_name = entry.file_name().into_string().unwrap_or_default();
            proto_files.push(file_name);
        }
    }

    (proto_folders, proto_files)
}

fn collect_type_fqns_from_descriptor(desc_bytes: &[u8]) -> Vec<String> {
    use prost::Message;
    use prost_types::{DescriptorProto, FileDescriptorSet};

    fn walk_msg(pkg: &str, m: &DescriptorProto, out: &mut Vec<String>) {
        let name = m.name.as_deref().unwrap_or_default();
        let fqn = if pkg.is_empty() {
            format!(".{name}")
        } else {
            format!(".{pkg}.{name}")
        };
        out.push(fqn);
        for en in &m.enum_type {
            let en_name = en.name.as_deref().unwrap_or_default();
            let fqn = if pkg.is_empty() {
                format!(".{en_name}")
            } else {
                format!(".{pkg}.{en_name}")
            };
            out.push(fqn);
        }
        for nm in &m.nested_type {
            walk_msg(pkg, nm, out);
        }
    }

    let fds = FileDescriptorSet::decode(desc_bytes).expect("decode descriptor failed");
    let mut out = Vec::new();

    for file in &fds.file {
        let pkg = file.package.clone().unwrap_or_default();
        for en in &file.enum_type {
            let name = en.name.clone().unwrap_or_default();
            let fqn = if pkg.is_empty() {
                format!(".{name}")
            } else {
                format!(".{pkg}.{name}")
            };
            out.push(fqn);
        }
        for m in &file.message_type {
            walk_msg(&pkg, m, &mut out);
        }
    }

    out.sort();
    out.dedup();
    out
}

fn main() -> Result<()> {
    let path = "../../proto";
    let (proto_folders, proto_files) = read_proto_folders(path);
    let out_path = "src/generated";

    if !Path::new(out_path).exists() {
        create_dir(out_path).expect("Cannot create the `generated` folder!");
    }

    println!("cargo:rerun-if-changed={path}");
    for f in &proto_files {
        println!("cargo:rerun-if-changed={path}/{f}");
    }

    let descriptor_path = PathBuf::from(out_path).join("proto_descriptor.bin");
    let mut prost_cfg = prost_build::Config::new();
    prost_cfg
        .file_descriptor_set_path(&descriptor_path)
        .compile_well_known_types()
        .extern_path(".google.protobuf", "::pbjson_types")
        .protoc_arg("--experimental_allow_proto3_optional");

    prost_cfg
        .compile_protos(&proto_files, &proto_folders)
        .expect("prost-build descriptor generation failed");

    let build_result = tonic_prost_build::configure()
        .out_dir(out_path)
        .build_server(true)
        .build_client(true)
        .compile_protos(&proto_files, &proto_folders);

    if let Err(error) = build_result {
        panic!("Cannot build the proto files! Reason: {:?}", error);
    }

    let desc_bytes = fs::read(&descriptor_path).expect("failed to read descriptor set");
    let fqns = collect_type_fqns_from_descriptor(&desc_bytes);
    if fqns.is_empty() {
        panic!("No message/enum types found in descriptor — pbjson can't generate code");
    }

    pbjson_build::Builder::new()
        .out_dir(out_path)
        .register_descriptors(&desc_bytes)
        .expect("failed to generate serde impl")
        .build(&fqns)
        .expect("pbjson-build failed");

    Ok(())
}
