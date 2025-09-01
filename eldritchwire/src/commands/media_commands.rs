use crate::{commands::CommandData, EldritchError, Operation};
use eldritchwire_macros::CommandGroup;

#[derive(Clone, Debug, PartialEq, CommandGroup)]
pub enum MediaCommand {
    #[command(parameter(0x00), data_type(1), data(basic_codec, codec_varient))]
    Codec { operation: Operation, data: CodecData },
    #[command(
        parameter(0x01),
        data_type(1),
        data(mode, speed, flags, slot_one_storage_medium, slot_two_storage_medium)
    )]
    TransportMode { operation: Operation, data: TransportModeData },
}

#[derive(Clone, Debug, PartialEq)]
pub struct CodecData {
    basic_codec: i8,
    codec_varient: i8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransportModeData {
    mode: i8,
    speed: i8,
    flags: i8,
    slot_one_storage_medium: i8,
    slot_two_storage_medium: i8,
}
