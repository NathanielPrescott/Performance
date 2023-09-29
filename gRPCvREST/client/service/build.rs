use std::io::Result;

fn main() -> Result<()> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(&["imagestorage.proto"], &["../../proto"])
}
