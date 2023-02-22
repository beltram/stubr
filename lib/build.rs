fn main() -> Result<(), Box<dyn std::error::Error>> {
    // otherwise fails while building Docker image
    #[cfg(not(target_env = "musl"))]
    {
        for proto in std::fs::read_dir("tests/grpc/protos").unwrap() {
            proto.map(|p| p.path()).ok().map(tonic_build::compile_protos).transpose()?;
        }
    }
    Ok(())
}
