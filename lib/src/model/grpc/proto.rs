use std::path::PathBuf;

use itertools::*;
use protobuf::{
    descriptor::FileDescriptorProto,
    reflect::{FileDescriptor, MessageDescriptor},
};

use crate::{StubrError, StubrResult};

pub fn parse_message_descriptor(msg: &str, proto_file: &PathBuf) -> StubrResult<MessageDescriptor> {
    let folder = proto_file
        .parent()
        .ok_or_else(|| StubrError::ProtobufFileNotFound(proto_file.to_path_buf()))?;
    let parsed = protobuf_parse::Parser::new()
        .pure()
        .includes([folder])
        .input(proto_file)
        .parse_and_typecheck()?;
    let proto_file_name = proto_file.file_name().and_then(|n| n.to_str());
    let (fd, dependencies): (Vec<FileDescriptorProto>, Vec<FileDescriptorProto>) =
        parsed.file_descriptors.into_iter().partition_map(|fd| {
            if fd.name.as_deref() == proto_file_name {
                Either::Left(fd)
            } else {
                Either::Right(fd)
            }
        });
    let fd = fd
        .first()
        .ok_or_else(|| StubrError::ProtobufFileNotFound(proto_file.to_path_buf()))?
        .clone();
    let dependencies = dependencies
        .into_iter()
        .filter_map(|d| FileDescriptor::new_dynamic(d, &[]).ok())
        .collect::<Vec<FileDescriptor>>();
    let fd = FileDescriptor::new_dynamic(fd, dependencies.as_slice())?;
    let fd = fd
        .message_by_package_relative_name(msg)
        .ok_or_else(|| StubrError::ProtoMessageNotFound(msg.to_string(), proto_file.clone()))?;
    Ok(fd)
}
