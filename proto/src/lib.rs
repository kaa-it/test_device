tonic::include_proto!("command");

pub mod reflection {
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("myservice_descriptor");
}
