use crate::{FixedPointDecimal, Operation};

#[derive(Clone, Debug, PartialEq)]
pub enum LensCommand {
    Focus(Operation, FixedPointDecimal),
    InstantaneousAutoFocus,
    ApatureFStop(FixedPointDecimal),
    ApatureNormalized(FixedPointDecimal),
    NoOp,
}

pub fn parse_lens_command(command_data: &[u8]) -> LensCommand {
    type Command = LensCommand;
    let param_val = command_data.get(0).expect("Should have param_val byte");
    println!("parse_lens_command param_val: {:?}", param_val);

    match param_val {
        0x00 => {
            // TODO: I really don't like this implimentation and this needs to be fixed.
            let (data, _rest) = command_data[3..5].split_at(size_of::<u16>());
            Command::Focus(
                if command_data[2] == b'0' {
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
        _ => Command::NoOp,
    }
}

#[cfg(test)]
mod lens_commands {
    use super::*;

    #[test]
    fn parse_focus_command() {
        let command_packet_data: [u8; 7] = [0x00, 0x80, 0x01, 0x9a, 0xfd, 0x00, 0x00];
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
        let command_packet_data = [0x01, 0x00, 0x00];
        let command = parse_lens_command(&command_packet_data);
        assert_eq!(command, LensCommand::InstantaneousAutoFocus);
    }
}
