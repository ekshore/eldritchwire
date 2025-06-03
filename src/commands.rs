pub mod lens_commands;

use lens_commands::LensCommand;

use crate::error::EldritchError;

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Lens(LensCommand),
}

pub fn parse_command(cmd_buffer: &[u8]) -> Result<Command, EldritchError> {
    if let Some(cmd_category) = cmd_buffer.get(0) {
        let command = match cmd_category {
            0x00 => Command::Lens(lens_commands::parse_lens_command(cmd_buffer)),
            _ => todo!("Command category has either not been implemented or is invalid"),
        };
        Ok(command)
    } else {
        Err(EldritchError::InvalidCommandData)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{FixedPointDecimal, Operation, commands::LensCommand};

    #[test]
    fn parse_lens_focus_command() {
        let command_data: [u8; 8] = [0x00, 0x00, 0x80, 0x01, 0x9a, 0xfd, 0x00, 0x00];
        let command = parse_command(&command_data);
        assert_eq!(
            command,
            Ok(Command::Lens(LensCommand::Focus(
                Operation::Increment,
                FixedPointDecimal {
                    raw_val: 0xFD9Au16 as i16
                }
            )))
        )
    }
}
