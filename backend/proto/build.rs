use std::{env, path::PathBuf};

fn main() -> std::io::Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("tstats_descriptor.bin"))
        .build_server(true)
        .emit_rerun_if_changed(true)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
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
                "../../proto/scores.proto",
                "../../proto/team.proto",
            ],
            &["../../proto/"],
        )?;

    Ok(())
}
