pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("tstats_descriptor");

mod implementation;

pub mod osu_auth {
    tonic::include_proto!("osu.auth");
}

pub mod keys {
    tonic::include_proto!("keys");
}

pub mod tournaments {
    tonic::include_proto!("tournaments");
}

pub mod stages {
    tonic::include_proto!("stages");
}

pub mod osu {
    pub use crate::implementation::osu::api;
    pub type OsuUser = User;
    tonic::include_proto!("osu");
}

pub mod pool {
    tonic::include_proto!("pool");
}

pub mod debug_data {
    tonic::include_proto!("debug");
}

pub mod utils {
    tonic::include_proto!("utils");
}

pub mod scores {
    tonic::include_proto!("scores");
}

pub mod team {
    tonic::include_proto!("team");
}
