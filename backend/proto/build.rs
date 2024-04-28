use std::{env, path::PathBuf};

fn main() -> std::io::Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("tstats_descriptor.bin"))
        .build_server(true)
        .compile(
            &[
                "../../proto/keys.proto",
                "../../proto/tournaments.proto",
                "../../proto/stages.proto",
                "../../proto/debug.proto",
                "../../proto/osu.proto",
                "../../proto/osuauth.proto",
                "../../proto/pool.proto",
                "../../proto/utils.proto",
            ],
            &["../../proto/"],
        )?;

    Ok(())
}
