fn main() {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/proto") // you can change the generated code's location
        .compile(
            &["proto/backend.proto"],
            &["proto/"],
        ).unwrap();
}