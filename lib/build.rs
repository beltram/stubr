// otherwise fails while building Docker image
#[cfg(not(target_env = "musl"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    for proto in std::fs::read_dir("tests/grpc/protos")? {
        tonic_build::compile_protos(proto?.path())?;
    }
    Ok(())
}

#[cfg(target_env = "musl")]
fn main() -> Result<(), Box<dyn std::error::Error>> {}
