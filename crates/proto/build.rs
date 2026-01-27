fn main() {
    tonic_prost_build::configure()
        .build_client(false)
        .compile_protos(&["proto/sample.proto"], &["proto/"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
}
