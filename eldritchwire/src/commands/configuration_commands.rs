use crate::{commands::CommandData, EldritchError, Operation};
use eldritchwire_macros::CommandGroup;

#[derive(Clone, Debug, PartialEq, CommandGroup)]
pub enum ConfigurationCommand {
    #[command(parameter(0x00), data_type(3), data(time, date))]
    RealTimeClock {
        operation: Operation,
        data: RealTimeClockData,
    },
    #[command(parameter(0x01), data_type(5))]
    SystemLanguage { operation: Operation, data: String },
    #[command(parameter(0x03), data_type(3))]
    TimeZone { operation: Operation, data: i32 },
    #[command(parameter(0x04), data_type(4), data(laditude, longitude))]
    Location {
        operation: Operation,
        data: LocationData,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct RealTimeClockData {
    pub time: i32,
    pub date: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LocationData {
    pub laditude: i64,
    pub longitude: i64,
}
