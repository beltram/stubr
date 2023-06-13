fn main() -> Result<(), Box<dyn std::error::Error>> {
    // otherwise fails while building Docker image
    if std::env::var("DOCKER_BUILD").is_err() {
        for proto in std::fs::read_dir("tests/grpc/protos")? {
            tonic_build::compile_protos(proto?.path())?;
        }
    }
    Ok(())
}
