use tonic_build;

fn main() {
    tonic_build::compile_protos("proto/cli.proto").unwrap();
}
