use crate::{commands::CommandData, EldritchError, FixedPointDecimal, Operation};
use eldritchwire_macros::CommandGroup;

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Debug, PartialEq, CommandGroup)]
pub enum TallyCommand {
    #[command(parameter(0x00), data_type(128), bounds(lower(0.0), upper(1.0)))]
    TallyBrightness {
        operation: Operation,
        data: FixedPointDecimal,
    },
    #[command(parameter(0x01), data_type(128), bounds(lower(0.0), upper(1.0)))]
    FrontTallyBrightness {
        operation: Operation,
        data: FixedPointDecimal,
    },
    #[command(parameter(0x02), data_type(128), bounds(lower(0.0), upper(1.0)))]
    RearTallyBrightness {
        operation: Operation,
        data: FixedPointDecimal,
    },
}
