use crate::{FixedPointDecimal, Operation};

#[derive(Clone, Debug, PartialEq)]
pub enum LensCommand {
    Focus(Operation, FixedPointDecimal),
    InstantaneousAutoFocus,
    ApatureFStop(FixedPointDecimal),
    ApatureNormalized(FixedPointDecimal),
    OpticalImageStabalization(Operation, bool),
    NoOp,
}

pub fn parse_lens_command(command_data: &[u8]) -> LensCommand {
    type Command = LensCommand;
    let param_val = command_data.get(1).expect("Should have param_val byte");

    match param_val {
        0x00 => {
            // TODO: I really don't like this implimentation and this needs to be fixed.
            let (data, _rest) = command_data[4..6].split_at(size_of::<u16>());
            Command::Focus(
                if command_data[3] == 0 {
                    Operation::Assign
                } else {
                    Operation::Increment
                },
                FixedPointDecimal::from_data(
                    data.try_into()
                        .expect("Data should have been size of usize"),
                ),
            )
        }
        0x01 => Command::InstantaneousAutoFocus,
        0x06 => Command::OpticalImageStabalization(
            if command_data[3] == 0 {
                Operation::Assign
            } else {
                Operation::Toggle
            },
            command_data[4] != 0,
        ),
        _ => Command::NoOp,
    }
}

#[cfg(test)]
mod lens_commands {
    use super::*;

    #[test]
    fn parse_focus_command() {
        let command_packet_data: [u8; 6] = [0x00, 0x00, 0x80, 0x01, 0x9a, 0xfd];
        let command = parse_lens_command(&command_packet_data);
        assert_eq!(
            command,
            LensCommand::Focus(
                Operation::Increment,
                FixedPointDecimal {
                    raw_val: 0xfd9au16 as i16
                }
            )
        );
    }

    #[test]
    fn parse_auto_focus_command() {
        let command_packet_data = [0x00, 0x01, 0x00, 0x00];
        let command = parse_lens_command(&command_packet_data);
        assert_eq!(command, LensCommand::InstantaneousAutoFocus);
    }

    #[test]
    fn parse_ois_on() {
        let command_data = [0x00, 0x06, 0x00, 0x00, 0x01];
        let command = parse_lens_command(&command_data);
        assert_eq!(
            command,
            LensCommand::OpticalImageStabalization(Operation::Assign, true)
        );
    }
}
