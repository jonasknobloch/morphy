use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["src/morfessor/morfessor.proto"], &["src/morfessor/"])?;
    Ok(())
}