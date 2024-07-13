use std::io::Result;
use prost_build::*;

fn main() -> Result<()> {
    let internal_prefix = "../../internal";

    compile_protos(&[
        format!("{internal_prefix}/configuration.proto"), 
        format!("{internal_prefix}/flow.proto"), 
        format!("{internal_prefix}/node.proto"), 
        format!("{internal_prefix}/rule.proto"), 
        format!("{internal_prefix}/type.proto"), 
        format!("{internal_prefix}/variable.proto")
    ], &["src/lib.rs"]).expect("Cannot compile protos");

    Ok(())
}