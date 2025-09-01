use super::CommandData;
use crate::{EldritchError, FixedPointDecimal, Operation};
use eldritchwire_macros::CommandGroup;

#[derive(Clone, Debug, PartialEq, CommandGroup)]
pub enum AudioCommand {
    #[command(parameter(0x00), data_type(0x80), bounds(lower(0.0), upper(1.0)))]
    MicLevel {
        operation: Operation,
        data: FixedPointDecimal,
    },
    #[command(parameter(0x01), data_type(0x80), bounds(lower(0.0), upper(1.0)))]
    HeadphoneLevel {
        operation: Operation,
        data: FixedPointDecimal,
    },
    #[command(parameter(0x02), data_type(0x80), bounds(lower(0.0), upper(1.0)))]
    HeadphoneProgramMix {
        operation: Operation,
        data: FixedPointDecimal,
    },
    #[command(parameter(0x03), data_type(0x80), bounds(lower(0.0), upper(1.0)))]
    SpeakerLevel {
        operation: Operation,
        data: FixedPointDecimal,
    },
    #[command(parameter(0x04), data_type(1), bounds(lower(0), upper(3)))]
    InputType { operation: Operation, data: i8 },
    #[command(parameter(0x05), data_type(0x80), data(channel_one, channel_two))]
    InputLevels {
        operation: Operation,
        data: InputLevelsData,
    },
    #[command(parameter(0x06), data_type(0))]
    PhantomPower { operation: Operation, data: bool },
}

#[derive(Clone, Debug, PartialEq)]
pub struct InputLevelsData {
    pub channel_one: FixedPointDecimal,
    pub channel_two: FixedPointDecimal,
}
