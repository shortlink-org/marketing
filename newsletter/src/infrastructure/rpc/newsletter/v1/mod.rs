pub mod api;

pub mod proto {
    #![allow(dead_code)]
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("infrastructure.rpc.newsletter.v1");

    // Make the descriptor bytes available to main.rs for reflection:
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("infrastructure.rpc.newsletter.v1_descriptor");
}
