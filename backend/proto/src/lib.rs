pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("tstats_descriptor");

pub mod keys {
    tonic::include_proto!("keys");
}

pub mod tournaments {
    tonic::include_proto!("tournaments");
}

pub mod stages {
    tonic::include_proto!("stages");
}

pub mod pool_brackets {
    tonic::include_proto!("pool_brackets");
}
pub mod debug_data {
    tonic::include_proto!("debug");
}
