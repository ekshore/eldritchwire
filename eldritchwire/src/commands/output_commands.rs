use crate::{commands::CommandData, EldritchError, FixedPointDecimal, Operation};
use eldritchwire_macros::CommandGroup;

#[derive(Clone, Debug, PartialEq, CommandGroup)]
pub enum OutputCommand {
    #[command(parameter(0x00), data_type(2))]
    OverlayEnabled { operation: Operation, data: i16 },
    #[command(parameter(0x01), data_type(1), bounds(lower(0), upper(8)))]
    FrameGuideStyles { operation: Operation, data: i8 },
    #[command(parameter(0x02), data_type(128), bounds(lower(0.1), upper(1.0)))]
    FrameGuidesOpacity {
        operation: Operation,
        data: FixedPointDecimal,
    },
    #[command(
        parameter(0x03),
        data_type(1),
        data(
            frame_guide_style,
            frame_guide_opacity,
            safe_area_percentage,
            grid_style
        )
    )]
    Overlays {
        operation: Operation,
        data: OverlaysData,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct OverlaysData {
    pub frame_guide_style: i8,
    pub frame_guide_opacity: i8,
    pub safe_area_percentage: i8,
    pub grid_style: i8,
}
