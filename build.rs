fn main() -> Result<(), Box<dyn std::error::Error>> {
    let status_proto_file = "./src/proto/presence.proto";
    let auth_proto_file = "./src/proto/auth.proto";

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional") // for older systems
        .build_client(true)
        .build_server(false)
        .out_dir("./src/stubs")
        .compile(&[status_proto_file, auth_proto_file], &["proto"])?;

    Ok(())
}
