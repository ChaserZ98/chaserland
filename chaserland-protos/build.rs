fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_protos(
            &["./chaserland/greeter.proto", "./chaserland/article.proto"],
            &["./chaserland"],
        )?;
    Ok(())
}
