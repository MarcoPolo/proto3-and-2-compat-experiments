use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["src/circuitv2proto2.proto"], &["src/"])?;
    prost_build::compile_protos(&["src/circuitv2proto3.proto"], &["src/"])?;
    Ok(())
}
