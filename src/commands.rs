pub mod lens_commands;

use lens_commands::LensCommand;

use crate::error::EldritchError;

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Lens(LensCommand),
}

#[derive(Debug, PartialEq)]
pub struct CommandData<'a> {
    bytes: &'a [u8],
    // category: &'a u8,
    // parameter: &'a u8,
    // data_type: &'a u8,
    // operation: &'a u8,
    // data_buff: &'a [u8],
}

impl<'a> CommandData<'a> {
    pub fn new(bytes: &'a [u8]) -> Result<Self, EldritchError> {
        if bytes.len() < 3 {
            return Err(EldritchError::InvalidCommandData);
        }
        Ok(Self { bytes })
    }
}

impl CommandData<'_> {
    pub fn category(&self) -> &u8 {
        &self.bytes[0]
    }

    #[inline(always)]
    pub fn parameter(&self) -> &u8 {
        &self.bytes[1]
    }

    #[inline(always)]
    pub fn data_type(&self) -> &u8 {
        &self.bytes[2]
    }

    #[inline(always)]
    pub fn operation(&self) -> &u8 {
        &self.bytes[3]
    }

    #[inline(always)]
    pub fn data_buff(&self) -> &[u8] {
        &self.bytes[4..]
    }
}

pub fn parse_command(cmd_buffer: &[u8]) -> Result<Command, EldritchError> {
    if let Ok(cmd_data) = CommandData::new(cmd_buffer) {
        let command = match cmd_data.category() {
            0x00 => Command::Lens(lens_commands::parse_lens_command(cmd_data)?),
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
        if let Ok(cmd_data) = CommandData::new(&cmd_data) {
            assert_eq!(
                cmd_data,
                CommandData {
                    bytes: &[0x00, 0x06, 0x00, 0x00, 0x001]
                }
            );
            assert_eq!(&0x00, cmd_data.category());
        } else {
            assert!(false)
        }
    }

    #[test]
    fn parse_command_data_success_longer_data() {
        let cmd_data: [u8; 6] = [0x00, 0x00, 0x80, 0x01, 0x9a, 0xfd];
        if let Ok(cmd_data) = CommandData::new(&cmd_data) {
        assert_eq!(
            cmd_data,
            CommandData {
                bytes: &[0x00, 0x00, 0x80, 0x01, 0x9a, 0xfd],
            }
        );
        } else {
            assert!(false)
        }
    }
}
