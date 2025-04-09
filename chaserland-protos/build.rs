fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_protos(
            &["./protos/chaserland/article/v1/article_service.proto"],
            &["./protos"],
        )?;
    Ok(())
}
