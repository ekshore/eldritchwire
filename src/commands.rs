pub mod lens_commands;

use lens_commands::LensCommand;

use crate::error::EldritchError;

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Lens(LensCommand),
}

#[derive(Debug, PartialEq)]
pub struct CommandData<'a> {
    category: &'a u8,
    parameter: &'a u8,
    data_type: &'a u8,
    operation: &'a u8,
    data_buff: &'a [u8],
}

pub fn parse_command(cmd_buffer: &[u8]) -> Result<Command, EldritchError> {
    if let Ok(cmd_data) = parse_command_data(cmd_buffer) {
        let command = match cmd_data.category {
            0x00 => Command::Lens(lens_commands::parse_lens_command(cmd_data)?),
            _ => todo!("Command category has either not been implemented or is invalid"),
        };
        Ok(command)
    } else {
        Err(EldritchError::InvalidCommandData)
    }
}

fn parse_command_data(cmd_buff: &[u8]) -> Result<CommandData, EldritchError> {
    if cmd_buff.len() < 3 {
        return Err(EldritchError::InvalidCommandData);
    }

    Ok(CommandData {
        category: &cmd_buff[0],
        parameter: &cmd_buff[1],
        data_type: &cmd_buff[2],
        operation: &cmd_buff[3],
        data_buff: &cmd_buff[4..],
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{FixedPointDecimal, Operation, commands::LensCommand};

    #[test]
    fn parse_lens_focus_command() {
        let command_data: [u8; 6] = [0x00, 0x00, 0x80, 0x01, 0x9a, 0xfd];
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

    #[test]
    fn parse_lens_ois_command() {
        let cmd_data = [0x00, 0x06, 0x00, 0x00, 0x001];
        let cmd = parse_command(&cmd_data);
        assert_eq!(
            cmd,
            Ok(Command::Lens(LensCommand::OpticalImageStabalization(
                Operation::Assign,
                true
            )))
        );
    }

    #[test]
    fn parse_command_data_success() {
        let cmd_data = [0x00, 0x06, 0x00, 0x00, 0x001];
        let cmd_data = super::parse_command_data(&cmd_data);
        assert_eq!(
            cmd_data,
            Ok(CommandData {
                category: &0x00,
                parameter: &0x06,
                data_type: &0x00,
                operation: &0x00,
                data_buff: &[0x01],
            })
        );
    }

    #[test]
    fn parse_command_data_success_longer_data() {
        let command_data: [u8; 6] = [0x00, 0x00, 0x80, 0x01, 0x9a, 0xfd];
        let command_data = super::parse_command_data(&command_data);
        assert_eq!(
            command_data,
            Ok(CommandData {
                category: &0x00,
                parameter: &0x00,
                data_type: &0x80,
                operation: &0x01,
                data_buff: &[0x9a, 0xfd],
            })
        );
    }
}
