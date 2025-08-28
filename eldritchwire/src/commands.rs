pub mod lens_commands;
pub mod video_commands;
pub mod audio_commands;
pub mod output_commands;
pub mod display_commands;

use crate::error::EldritchError;

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Lens(lens_commands::LensCommand),
    Video(video_commands::VideoCommand),
    Audio(audio_commands::AudioCommand),
    Output(output_commands::OutputCommand),
    Display(display_commands::DisplayCommand),
}

#[derive(Debug, PartialEq)]
pub struct CommandData<'a> {
    bytes: &'a [u8],
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
            0x00 => Command::Lens(lens_commands::parse_command(cmd_data)?),
            0x01 => Command::Video(video_commands::parse_command(cmd_data)?),
            0x02 => Command::Audio(audio_commands::parse_command(cmd_data)?),
            0x03 => Command::Output(output_commands::parse_command(cmd_data)?),
            0x04 => Command::Display(display_commands::parse_command(cmd_data)?),
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
    use crate::{FixedPointDecimal, Operation};

    #[test]
    fn parse_lens_focus_command() {
        let command_data: [u8; 6] = [0x00, 0x00, 0x80, 0x01, 0x33, 0x01];
        let command = parse_command(&command_data);
        assert_eq!(
            command,
            Ok(Command::Lens(lens_commands::LensCommand::Focus {
                operation: Operation::Increment,
                data: FixedPointDecimal {
                    raw_val: 0x0133u16 as i16
                }
            }))
        );
    }

    #[test]
    fn parse_lens_ois_command() {
        let cmd_data = [0x00, 0x06, 0x00, 0x00, 0x001];
        let cmd = parse_command(&cmd_data);
        assert_eq!(
            cmd,
            Ok(Command::Lens(lens_commands::LensCommand::OpticalImageStabalization {
                operation: Operation::Assign,
                data: true
            }))
        );
    }

    #[test]
    fn parse_video_mode_command() {
        let cmd_data = [0x01, 0x00, 0x01, 0x00, 0x18, 0x01, 0x03, 0x00, 0x00];
        let cmd = parse_command(&cmd_data);
        assert_eq!(
            cmd,
            Ok(Command::Video(video_commands::VideoCommand::VideoMode {
                operation: Operation::Assign,
                data: video_commands::VideoModeData {
                    frame_rate: 24,
                    m_rate: 1,
                    dimensions: 3,
                    interlaced: 0,
                    color_space: 0,
                }
            },))
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
            panic!();
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
            panic!();
        }
    }
}
