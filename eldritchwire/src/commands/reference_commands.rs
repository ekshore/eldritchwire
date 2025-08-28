use crate::{ EldritchError, Operation, commands::CommandData };
use eldritchwire_macros::CommandGroup;

#[derive(Clone, Debug, PartialEq, CommandGroup)]
pub enum ReferenceCommand {
    #[command(parameter(0x00), data_type(1), bounds(lower(0), upper(1)))]
    Source {
        operation: Operation,
        data: i8,
    },
    #[command(parameter(0x01), data_type(3))]
    Offset {
        operation: Operation,
        data: i32,
    }
}
