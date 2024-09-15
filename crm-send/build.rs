use std::fs;

use anyhow::Result;
// use proto_builder_trait::tonic::BuilderAttributes;

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;
    let builder = tonic_build::configure();

    builder
        .out_dir("src/pb")
        .compile(&["send_notification.proto"], &["."])
        .unwrap();

    Ok(())
}
