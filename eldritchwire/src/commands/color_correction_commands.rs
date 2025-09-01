use crate::{commands::CommandData, EldritchError, FixedPointDecimal, Operation};
use eldritchwire_macros::CommandGroup;

#[derive(Clone, Debug, PartialEq, CommandGroup)]
pub enum ColorCorrectionCommand {
    #[command(parameter(0x00), data_type(0x80), data(red, green, blue, luma))]
    LiftAdjust {
        operation: Operation,
        data: RedGreenBlueLuma,
    },
    #[command(parameter(0x01), data_type(0x80), data(red, green, blue, luma))]
    GammaAdjust {
        operation: Operation,
        data: RedGreenBlueLuma,
    },
    #[command(parameter(0x02), data_type(0x80), data(red, green, blue, luma))]
    GainAdjust {
        operation: Operation,
        data: RedGreenBlueLuma,
    },
    #[command(parameter(0x03), data_type(0x80), data(red, green, blue, luma))]
    OffsetAdjust {
        operation: Operation,
        data: RedGreenBlueLuma,
    },
    #[command(parameter(0x04), data_type(0x80), data(pivot, adj))]
    ContrastAdjust {
        operation: Operation,
        data: ContrastAdjustData,
    },
    #[command(parameter(0x05), data_type(0x80), bounds(lower(0.0), upper(1.0)))]
    LumaMix {
        operation: Operation,
        data: FixedPointDecimal,
    },
    #[command(parameter(0x06), data_type(0x80), data(hue, sat))]
    ColorAdjust {
        operation: Operation,
        data: ColorAdjustData,
    },
    #[command(parameter(0x07))]
    CorrectionResetDefault,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RedGreenBlueLuma {
    pub red: FixedPointDecimal,
    pub green: FixedPointDecimal,
    pub blue: FixedPointDecimal,
    pub luma: FixedPointDecimal,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContrastAdjustData {
    pub pivot: FixedPointDecimal,
    pub adj: FixedPointDecimal,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColorAdjustData {
    pub hue: FixedPointDecimal,
    pub sat: FixedPointDecimal,
}

pub type LiftAdjustData = RedGreenBlueLuma;
pub type GammaAdjustData = RedGreenBlueLuma;
pub type GainAdjustData = RedGreenBlueLuma;
pub type OffsetAdjustData = RedGreenBlueLuma;
