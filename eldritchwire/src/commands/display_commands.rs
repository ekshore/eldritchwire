use crate::{commands::CommandData, EldritchError, FixedPointDecimal, Operation};
use eldritchwire_macros::CommandGroup;

#[derive(Clone, Debug, PartialEq, CommandGroup)]
pub enum DisplayCommand {
    #[command(parameter(0x00), data_type(0x80), bounds(lower(0.0), upper(1.0)))]
    Brightness {
        operation: Operation,
        data: FixedPointDecimal,
    },
    #[command(parameter(0x01), data_type(0x02))]
    OverlaysEnabled { operation: Operation, data: i16 },
    #[command(parameter(0x02), data_type(128), bounds(lower(0.0), upper(1.0)))]
    ZebraLevel {
        operation: Operation,
        data: FixedPointDecimal,
    },
    #[command(parameter(0x03), data_type(128), bounds(lower(0.0), upper(1.0)))]
    PeakingLevel {
        operation: Operation,
        data: FixedPointDecimal,
    },
    #[command(parameter(0x04), data_type(1), bounds(lower(0), upper(30)))]
    ColorBarsDisplayTime { operation: Operation, data: i8 },
    #[command(
        parameter(0x05),
        data_type(1),
        data(focus_assist_method, focus_line_color)
    )]
    FocusAssist {
        operation: Operation,
        data: FocusAssistData,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct FocusAssistData {
    pub focus_assist_method: i8,
    pub focus_line_color: i8,
}
