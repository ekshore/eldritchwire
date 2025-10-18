pub mod audio_commands;
pub mod color_correction_commands;
pub mod configuration_commands;
pub mod display_commands;
pub mod lens_commands;
pub mod media_commands;
pub mod output_commands;
pub mod ptz_control_commands;
pub mod reference_commands;
pub mod tally_commands;
pub mod video_commands;

use crate::error::EldritchError;

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Lens(lens_commands::LensCommand),
    Video(video_commands::VideoCommand),
    Audio(audio_commands::AudioCommand),
    Output(output_commands::OutputCommand),
    Display(display_commands::DisplayCommand),
    Tally(tally_commands::TallyCommand),
    Reference(reference_commands::ReferenceCommand),
    Configuration(configuration_commands::ConfigurationCommand),
    ColorCorrection(color_correction_commands::ColorCorrectionCommand),
    Media(media_commands::MediaCommand),
    PtzControl(ptz_control_commands::PtzControlCommand),
}

#[derive(Debug, PartialEq)]
pub struct CommandData<'a> {
    bytes: &'a [u8],
}

impl<'a> CommandData<'a> {
    pub fn new(bytes: &'a [u8]) -> Result<Self, EldritchError> {
        if bytes.len() < 3 {
            return Err(EldritchError::InvalidCommandData {
                message: "Package to short".into(),
                data: bytes.to_vec(),
            });
        }
        Ok(Self { bytes })
    }

    pub fn raw(&self) -> &[u8] {
        self.bytes
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
            0x05 => Command::Tally(tally_commands::parse_command(cmd_data)?),
            0x06 => Command::Reference(reference_commands::parse_command(cmd_data)?),
            0x07 => Command::Configuration(configuration_commands::parse_command(cmd_data)?),
            0x08 => Command::ColorCorrection(color_correction_commands::parse_command(cmd_data)?),
            0x0a => Command::Media(media_commands::parse_command(cmd_data)?),
            0x0b => Command::PtzControl(ptz_control_commands::parse_command(cmd_data)?),
            _ => todo!("Command category has either not been implemented or is invalid"),
        };
        Ok(command)
    } else {
        Err(EldritchError::InvalidCommandData {
            message: "No matching command group".into(),
            data: cmd_buffer.to_vec(),
        })
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
            Ok(Command::Lens(
                lens_commands::LensCommand::OpticalImageStabalization {
                    operation: Operation::Assign,
                    data: true
                }
            ))
        );
    }

    #[test]
    fn parse_set_exposure_command() {
        let cmd_data = [0x01, 0x05, 0x03, 0x00, 0x10, 0x27, 0x00, 0x00];
        let cmd = parse_command(&cmd_data);
        assert_eq!(
            cmd,
            Ok(Command::Video(video_commands::VideoCommand::ExposureUS {
                operation: Operation::Assign,
                data: 10000
            }))
        );
    }

    #[test]
    fn parse_add_15_percent_zebra() {
        let cmd_data = [0x04, 0x02, 0x80, 0x01, 0x33, 0x01];
        let cmd = parse_command(&cmd_data);
        assert_eq!(
            cmd,
            Ok(Command::Display(
                display_commands::DisplayCommand::ZebraLevel {
                    operation: Operation::Increment,
                    data: FixedPointDecimal::from_data(&[0x33, 0x01])
                }
            ))
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
    fn parse_subtract_gamma() {
        let cmd_data = [
            0x08, 0x01, 0x80, 0x01, 0x00, 0x00, 0x9a, 0xfd, 0x9a, 0xfd, 0x00, 0x00,
        ];
        let cmd = parse_command(&cmd_data);
        assert_eq!(
            cmd,
            Ok(Command::ColorCorrection(
                color_correction_commands::ColorCorrectionCommand::GammaAdjust {
                    operation: Operation::Increment,
                    data: color_correction_commands::RedGreenBlueLuma {
                        red: FixedPointDecimal { raw_val: 0x00 },
                        green: FixedPointDecimal {
                            raw_val: 0xfd9au16 as i16
                        },
                        blue: FixedPointDecimal {
                            raw_val: 0xfd9au16 as i16
                        },
                        luma: FixedPointDecimal { raw_val: 0x00 },
                    }
                }
            ))
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

    mod debug_examples {
        use super::lens_commands;
        use super::Command;
        use crate::Operation;

        #[test]
        fn packet_command() {
            let cmd_data: [u8; 6] = [0x00, 0x02, 0x80, 0x00, 0x8D, 0x1C];
            if let Ok(cmd_data) = super::parse_command(&cmd_data) {
                assert_eq!(
                    cmd_data,
                    super::Command::Lens(lens_commands::LensCommand::ApatureFStop {
                        operation: Operation::Assign,
                        data: crate::FixedPointDecimal {
                            raw_val: 0x1c8d_u16 as i16
                        }
                    })
                );
            } else {
                panic!();
            }
            assert!(true);
        }

        #[test]
        fn bug_nd_filter_stop_command() {
            let cmd_data: [u8; 6] = [0x01, 0x10, 0x80, 0x00, 0x00, 0x00];
            if let Ok(cmd) = super::parse_command(&cmd_data) {
                assert_eq!(
                    cmd,
                    Command::Video(super::video_commands::VideoCommand::NDFilterStop)
                );
            } else {
                panic!();
            }
        }
    }
}
