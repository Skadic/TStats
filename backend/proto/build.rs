fn main() -> std::io::Result<()> {
    tonic_build::configure().build_server(true).compile(
        &[
            "../../proto/tournaments.proto",
            "../../proto/stages.proto",
            "../../proto/debug.proto",
        ],
        &["../../proto/"],
    )?;

    Ok(())
}
