use itertools::*;
use protobuf::{
    descriptor::FileDescriptorProto,
    reflect::{FileDescriptor, MessageDescriptor},
};
use std::path::PathBuf;

pub fn parse_message_descriptor(msg: &str, proto_file: &PathBuf) -> MessageDescriptor {
    let folder = proto_file.parent().unwrap();
    let parsed = protobuf_parse::Parser::new()
        .pure()
        .includes([folder])
        .input(proto_file)
        .parse_and_typecheck()
        .unwrap();
    let proto_file_name = proto_file.file_name().and_then(|n| n.to_str());
    let (fd, dependencies): (Vec<FileDescriptorProto>, Vec<FileDescriptorProto>) =
        parsed.file_descriptors.into_iter().partition_map(|fd| {
            if fd.name.as_deref() == proto_file_name {
                Either::Left(fd)
            } else {
                Either::Right(fd)
            }
        });
    let fd = fd.first().unwrap().clone();
    let dependencies = dependencies
        .into_iter()
        .filter_map(|d| FileDescriptor::new_dynamic(d, &[]).ok())
        .collect::<Vec<FileDescriptor>>();
    FileDescriptor::new_dynamic(fd, dependencies.as_slice())
        .unwrap()
        .message_by_package_relative_name(msg)
        .unwrap()
}
