fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(
            &["proto/services.proto"], // Path to your proto file
            &["proto"],                // Directory where the proto file is located
        )?;
    Ok(())
}