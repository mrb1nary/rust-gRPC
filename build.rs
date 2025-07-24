
fn main() {
    tonic_build::compile_protos("proto/indexer.proto").unwrap();
}
