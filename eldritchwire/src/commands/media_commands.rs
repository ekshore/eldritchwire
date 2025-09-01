use crate::{commands::CommandData, EldritchError, Operation};
use eldritchwire_macros::CommandGroup;

#[derive(Clone, Debug, PartialEq, CommandGroup)]
pub enum MediaCommand {
    #[command(parameter(0x00), data_type(1), data(basic_codec, codec_varient))]
    Codec {
        operation: Operation,
        data: CodecData,
    },
    #[command(
        parameter(0x01),
        data_type(1),
        data(mode, speed, flags, slot_one_storage_medium, slot_two_storage_medium)
    )]
    TransportMode {
        operation: Operation,
        data: TransportModeData,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct CodecData {
    pub basic_codec: i8,
    pub codec_varient: i8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransportModeData {
    pub mode: i8,
    pub speed: i8,
    pub flags: i8,
    pub slot_one_storage_medium: i8,
    pub slot_two_storage_medium: i8,
}
