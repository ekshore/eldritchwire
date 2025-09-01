use crate::{EldritchError, FixedPointDecimal, Operation, commands::CommandData};
use eldritchwire_macros::CommandGroup;

#[derive(Clone, Debug, PartialEq, CommandGroup)]
pub enum PtzControlCommand {
    #[command(parameter(0x00), data_type(0x80), data(pan_velocity, tilt_velocity))]
    PanTiltVelocity {
        operation: Operation,
        data: PanTiltVelocityData,
    },
    #[command(parameter(0x01), data_type(1), data(preset_command, preset_slot))]
    MemoryPreset {
        operation: Operation,
        data: MemoryPresetData,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PanTiltVelocityData {
    pan_velocity: FixedPointDecimal,
    tilt_velocity: FixedPointDecimal,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MemoryPresetData {
    preset_command: i8,
    preset_slot: i8,
}
