use crate::{commands::CommandData, EldritchError, FixedPointDecimal, Operation};
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
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct PanTiltVelocityData {
    pub pan_velocity: FixedPointDecimal,
    pub tilt_velocity: FixedPointDecimal,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MemoryPresetData {
    pub preset_command: i8,
    pub preset_slot: i8,
}
