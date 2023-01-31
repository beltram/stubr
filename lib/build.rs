fn main() -> Result<(), Box<dyn std::error::Error>> {
    for proto in std::fs::read_dir("tests/grpc/protos").unwrap() {
        proto.map(|p| p.path()).ok().map(tonic_build::compile_protos).transpose()?;
    }
    Ok(())
}
