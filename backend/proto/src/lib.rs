pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("tstats_descriptor");

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
    pub type OsuUser = User;
    tonic::include_proto!("osu");
}

pub mod pool {
    tonic::include_proto!("pool");
}

pub mod debug_data {
    tonic::include_proto!("debug");
}
