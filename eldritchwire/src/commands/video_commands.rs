#[cfg(not(feature = "ignore-nd-filter"))]
use crate::FixedPointDecimal;
use crate::{error::EldritchError, Operation};
use eldritchwire_macros::CommandGroup;

use super::CommandData;

#[derive(Clone, Debug, PartialEq, CommandGroup)]
pub enum VideoCommand {
    #[command(
        parameter(0x00),
        data_type(1),
        data(frame_rate, m_rate, dimensions, interlaced, color_space)
    )]
    VideoMode {
        operation: Operation,
        data: VideoModeData,
    },
    #[command(parameter(0x01), data_type(1), bounds(lower(1), upper(16)))]
    GainUpToCamera49 { operation: Operation, data: i8 },
    #[command(parameter(0x02), data_type(2), data(color_temp, tint))]
    ManualWhiteBalance {
        operation: Operation,
        data: ManualWhiteBalanceData,
    },
    #[command(parameter(0x03))]
    SetAutoWB,
    #[command(parameter(0x04))]
    RestoreAutoWB,
    #[command(parameter(0x05), data_type(3), bounds(lower(1), upper(42000)))]
    ExposureUS { operation: Operation, data: i32 },
    #[command(parameter(0x06), data_type(2), bounds(lower(0)))]
    ExposureOrdinal { operation: Operation, data: i16 },
    #[command(parameter(0x07), data_type(1), bounds(lower(0), upper(1)))]
    DynamicRageMode { operation: Operation, data: i8 },
    #[command(parameter(0x08), data_type(1), bounds(lower(0), upper(3)))]
    VideoSharpeningLevel { operation: Operation, data: i8 },
    #[command(
        parameter(0x09),
        data_type(2),
        data(file_frame_rate, sensor_frame_rate, frame_width, frame_height, flags)
    )]
    RecordingFormat {
        operation: Operation,
        data: RecordingFormatData,
    },
    #[command(parameter(0x0a), data_type(1), bounds(lower(0), upper(4)))]
    SetAutoExposureMode { operation: Operation, data: i8 },
    #[command(parameter(0x0b), data_type(3), bounds(lower(100), upper(36000)))]
    ShutterAngle { operation: Operation, data: i32 },
    #[command(parameter(0x0c), data_type(3), bounds(lower(24), upper(2000)))]
    ShutterSpeed { operation: Operation, data: i32 },
    #[command(parameter(0x0d), data_type(1), bounds(lower(-128), upper(127)))]
    Gain { operation: Operation, data: i8 },
    #[allow(clippy::upper_case_acronyms)]
    #[command(parameter(0x0e), data_type(3), bounds(lower(0), upper(2147483647)))]
    ISO { operation: Operation, data: i32 },
    #[command(parameter(0x0f), data_type(1), data(selected, enabled))]
    DisplayLUT {
        operation: Operation,
        data: DisplayLUTData,
    },
    #[cfg(not(feature = "ignore-nd-filter"))]
    #[command(parameter(0x10), data_type(128), data(stop, display_mode))]
    NDFilterStop {
        operation: Operation,
        data: NDFilterStopData,
    },
    #[cfg(feature = "ignore-nd-filter")]
    #[command(parameter(0x10))]
    NDFilterStop,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VideoModeData {
    pub frame_rate: i8,
    pub m_rate: i8,
    pub dimensions: i8,
    pub interlaced: i8,
    pub color_space: i8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ManualWhiteBalanceData {
    pub color_temp: i16,
    pub tint: i16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecordingFormatData {
    pub file_frame_rate: i16,
    pub sensor_frame_rate: i16,
    pub frame_width: i16,
    pub frame_height: i16,
    pub flags: i16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DisplayLUTData {
    selected: i8,
    enabled: i8,
}

#[cfg(not(feature = "ignore-nd-filter"))]
#[derive(Clone, Debug, PartialEq)]
pub struct NDFilterStopData {
    stop: FixedPointDecimal,
    display_mode: FixedPointDecimal,
}
