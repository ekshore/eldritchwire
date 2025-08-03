use crate::{FixedPointDecimal, Operation, error::EldritchError};
use eldritchwire_macros::CommandGroup;

use super::CommandData;

#[derive(Clone, Debug, PartialEq, CommandGroup)]
pub enum LensCommand {
    #[command(parameter(0x00), data_type(128))]
    Focus(Operation, FixedPointDecimal),

    #[command(parameter(0x01))]
    InstantaneousAutoFocus,

    #[command(parameter(0x02), data_type(128))]
    ApatureFStop(Operation, FixedPointDecimal),

    #[command(parameter(0x03), data_type(128))]
    ApatureNormalized(Operation, FixedPointDecimal),

    #[command(parameter(0x04), data_type(2))]
    ApatureOrdinal(Operation, i16),

    #[command(parameter(0x05))]
    InstantaneousAutoApature,

    #[command(parameter(0x06), data_type(0))]
    OpticalImageStabalization(Operation, bool),

    #[command(parameter(0x07), data_type(2))]
    AbsoluteZoomMM(Operation, i16),

    #[command(parameter(0x08), data_type(128))]
    AbsoluteZoomNormalized(Operation, FixedPointDecimal),

    #[command(parameter(0x09), data_type(128))]
    AbsoluteZoomContinuous(Operation, FixedPointDecimal),
}

type LensResult = Result<LensCommand, EldritchError>;

pub fn parse_lens_command(command_data: CommandData) -> LensResult {
    type Command = LensCommand;

    match command_data.parameter() {
        0x00 => parse_focus_command(command_data),
        0x01 => Ok(Command::InstantaneousAutoFocus),
        0x02 => parse_apature_fstop_command(command_data),
        0x03 => parse_apature_normalized_command(command_data),
        0x04 => parse_apature_ordinal(command_data),
        0x05 => Ok(Command::InstantaneousAutoApature),
        0x06 => parse_ois_command(command_data),
        0x07 => parse_absolute_zoom_mm_command(command_data),
        0x08 => parse_absolute_zoom_normalized_command(command_data),
        0x09 => parse_absolute_zoom_continuous_command(command_data),
        _ => Err(EldritchError::InvalidCommandData),
    }
}

fn parse_focus_command(cmd_data: CommandData) -> LensResult {
    if let Ok(data) = cmd_data.data_buff().try_into() {
        let data = FixedPointDecimal::from_data(data);
        if !(0.0..=1.0).contains(&data.get_real_val()) {
            Err(EldritchError::DataOutOfBounds)
        } else {
            Ok(LensCommand::Focus(
                if *cmd_data.operation() == 0 {
                    Operation::Assign
                } else {
                    Operation::Increment
                },
                data,
            ))
        }
    } else {
        Err(EldritchError::InvalidDataType {
            command: String::from("Focus Command"),
            expected: String::from("FixedPointDecimal"),
        })
    }
}

fn parse_apature_fstop_command(cmd_data: CommandData) -> LensResult {
    if let Ok(data) = cmd_data.data_buff().try_into() {
        let data = FixedPointDecimal::from_data(data);
        if !(-1.0..=16.0).contains(&data.get_real_val()) {
            Err(EldritchError::DataOutOfBounds)
        } else {
            Ok(LensCommand::ApatureFStop(
                if *cmd_data.operation() == 0 {
                    Operation::Assign
                } else {
                    Operation::Increment
                },
                data,
            ))
        }
    } else {
        Err(EldritchError::InvalidCommandData)
    }
}

fn parse_apature_normalized_command(cmd_data: CommandData) -> LensResult {
    if let Ok(data) = cmd_data.data_buff().try_into() {
        let data = FixedPointDecimal::from_data(data);
        if !(0.0..=1.0).contains(&data.get_real_val()) {
            Err(EldritchError::DataOutOfBounds)
        } else {
            Ok(LensCommand::ApatureNormalized(
                if *cmd_data.operation() == 0 {
                    Operation::Assign
                } else {
                    Operation::Increment
                },
                data,
            ))
        }
    } else {
        Err(EldritchError::InvalidCommandData)
    }
}

fn parse_apature_ordinal(cmd_data: CommandData) -> LensResult {
    if *cmd_data.data_type() == 2 {
        if let Ok(data) = (*cmd_data.data_buff()).try_into() {
            let data = i16::from_le_bytes(data);
            if data < 0 {
                Err(EldritchError::DataOutOfBounds)
            } else {
                Ok(LensCommand::ApatureOrdinal(
                    if *cmd_data.operation() == 0 {
                        Operation::Assign
                    } else {
                        Operation::Increment
                    },
                    data,
                ))
            }
        } else {
            Err(EldritchError::InvalidCommandData)
        }
    } else {
        Err(EldritchError::InvalidCommandData)
    }
}

fn parse_ois_command(cmd_data: CommandData) -> LensResult {
    Ok(LensCommand::OpticalImageStabalization(
        if *cmd_data.operation() == 0 {
            Operation::Assign
        } else {
            return Err(EldritchError::InvalidCommandData);
        },
        cmd_data.data_buff()[0] != 0,
    ))
}

fn parse_absolute_zoom_mm_command(cmd_data: CommandData) -> LensResult {
    if *cmd_data.data_type() == 2 {
        if let Ok(data) = cmd_data.data_buff().try_into() {
            let data = i16::from_le_bytes(data);
            if data < 0 {
                Err(EldritchError::DataOutOfBounds)
            } else {
                Ok(LensCommand::AbsoluteZoomMM(
                    if *cmd_data.operation() == 0 {
                        Operation::Assign
                    } else {
                        Operation::Increment
                    },
                    data,
                ))
            }
        } else {
            Err(EldritchError::InvalidCommandData)
        }
    } else {
        Err(EldritchError::InvalidCommandData)
    }
}

fn parse_absolute_zoom_normalized_command(cmd_data: CommandData) -> LensResult {
    if *cmd_data.data_type() == 128 {
        if let Ok(data) = cmd_data.data_buff().try_into() {
            let data = FixedPointDecimal::from_data(data);
            if !(0.0..=1.0).contains(&data.get_real_val()) {
                Err(EldritchError::DataOutOfBounds)
            } else {
                Ok(LensCommand::AbsoluteZoomNormalized(
                    if *cmd_data.operation() == 0 {
                        Operation::Assign
                    } else {
                        Operation::Increment
                    },
                    data,
                ))
            }
        } else {
            Err(EldritchError::InvalidCommandData)
        }
    } else {
        Err(EldritchError::InvalidCommandData)
    }
}

fn parse_absolute_zoom_continuous_command(cmd_data: CommandData) -> LensResult {
    if *cmd_data.data_type() == 128 {
        if let Ok(data) = cmd_data.data_buff().try_into() {
            let data = FixedPointDecimal::from_data(data);
            if !(-1.0..=1.0).contains(&data.get_real_val()) {
                Err(EldritchError::DataOutOfBounds)
            } else {
                Ok(LensCommand::AbsoluteZoomContinuous(Operation::Assign, data))
            }
        } else {
            Err(EldritchError::InvalidCommandData)
        }
    } else {
        Err(EldritchError::InvalidCommandData)
    }
}

#[cfg(test)]
mod lens_commands_tests {
    use super::*;

    #[test]
    fn parse_command_data_assign() {
        let command_data = [0x00, 0x00, 0x80, 0x00, 0x33, 0x01];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::Focus(
                Operation::Assign,
                FixedPointDecimal {
                    raw_val: 0x0133u16 as i16,
                }
            ))
        );
    }

    #[test]
    fn parse_focus_command_increment() {
        let command_packet_data: [u8; 6] = [0x00, 0x00, 0x80, 0x01, 0x33, 0x01];
        let command_data = CommandData::new(&command_packet_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::Focus(
                Operation::Increment,
                FixedPointDecimal {
                    raw_val: 0x0133u16 as i16
                }
            ))
        );
    }

    #[test]
    fn parse_focus_command_below_bounds() {
        // Value 0.0001 (ffff) is below the bounds of 0.0
        let command_data = [0x00, 0x00, 0x80, 0x00, 0xff, 0xff];
        let command_data = CommandData::new(&command_data).expect("Data has correct length");
        let command = super::parse_command(command_data);
        assert_eq!(command, Err(EldritchError::DataOutOfBounds));
    }

    #[test]
    fn parse_focus_command_above_bounds() {
        // Value 1.1 (08cc) is greater the bound of 1.0
        let command_data = [0x00, 0x00, 0x80, 0x00, 0xcc, 0x08];
        let command_data = CommandData::new(&command_data).expect("Data has correct length");
        let command = super::parse_command(command_data);
        assert_eq!(command, Err(EldritchError::DataOutOfBounds));
    }

    #[test]
    fn parse_auto_focus_command() {
        let command_packet_data = [0x00, 0x01, 0x00, 0x00];
        let command_data = CommandData::new(&command_packet_data).expect("Known good packet data");
        let command = parse_command(command_data);
        assert_eq!(command, Ok(LensCommand::InstantaneousAutoFocus));
    }

    #[test]
    fn parse_apature_fstop_command() {
        let command_data = [0x00, 0x02, 0x80, 0x00, 0x9a, 0xfd];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::ApatureFStop(
                Operation::Assign,
                FixedPointDecimal {
                    raw_val: 0xfd9au16 as i16
                }
            ))
        );
    }

    #[test]
    fn parse_apature_fstop_command_below_bounds() {
        // Value -1.1 (f734) is below the lower bound of -1
        let command_data = [0x00, 0x02, 0x80, 0x00, 0x34, 0xf7];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(command, Err(EldritchError::DataOutOfBounds));
    }

    #[test]
    fn parse_apature_fstop_command_at_upper_bounds() {
        // Value 15.9995 (7fff) is right at the limit of the upper bound
        let command_data = [0x00, 0x02, 0x80, 0x00, 0xff, 0x7f];
        let fp = FixedPointDecimal::from_data(&[0x00, 0x80]);
        println!("{:?}", fp);
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::ApatureFStop(
                Operation::Assign,
                FixedPointDecimal {
                    raw_val: 0x7fffu16 as i16
                }
            ))
        );
    }

    #[test]
    fn parse_apature_normalized_assign() {
        let command_data = [0x00, 0x03, 0x80, 0x00, 0x00, 0x04];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::ApatureNormalized(
                Operation::Assign,
                FixedPointDecimal {
                    raw_val: 0x0400u16 as i16
                }
            ))
        );
    }

    #[test]
    fn parse_apature_normalized_command_below_bounds() {
        // Value -0.1 is below the bound of 0.0
        let command_data = [0x00, 0x03, 0x80, 0x00, 0xff, 0xff];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(command, Err(EldritchError::DataOutOfBounds));
    }

    #[test]
    fn parse_apature_normalized_command_above_bounds() {
        // Value 1.1 is above the bound of 1.0
        let command_data = [0x00, 0x03, 0x80, 0x00, 0xcc, 0x08];
        let command_data = CommandData::new(&command_data).expect("Known good command data");
        let command = super::parse_command(command_data);
        assert_eq!(command, Err(EldritchError::DataOutOfBounds));
    }

    #[test]
    fn parse_apature_ordinal_assign() {
        let command_data = [0x00, 0x04, 0x02, 0x00, 0x10, 0x27];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::ApatureOrdinal(Operation::Assign, 10_000_i16))
        );
    }

    #[test]
    fn parse_apature_ordinal_command_bad_data_type() {
        let command_data = [0x00, 0x04, 0x01, 0x00, 0x10, 0x27];
        let command_data = CommandData::new(&command_data).expect("Should parse");
        let command = super::parse_command(command_data);
        assert_eq!(command, Err(EldritchError::InvalidCommandData));
    }

    #[test]
    fn parse_apature_ordinal_command_assign_below_bounds() {
        // Value -1 is below the bound of 0.0
        let command_data = [0x00, 0x04, 0x02, 0x00, 0xff, 0xff];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(command, Err(EldritchError::DataOutOfBounds));
    }

    #[test]
    fn parse_auto_apature_command() {
        let command_packet_data = [0x00, 0x05, 0x00, 0x00];
        let command_data = CommandData::new(&command_packet_data).expect("Known good packet data");
        let command = parse_command(command_data);
        assert_eq!(command, Ok(LensCommand::InstantaneousAutoApature));
    }

    #[test]
    fn parse_ois_command_on() {
        let command_data = [0x00, 0x06, 0x00, 0x00, 0x01];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::OpticalImageStabalization(
                Operation::Assign,
                true
            ))
        );
    }

    #[test]
    fn parse_ois_command_off() {
        let command_data = [0x00, 0x06, 0x00, 0x00, 0x00];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::OpticalImageStabalization(
                Operation::Assign,
                false
            ))
        );
    }

    #[test]
    fn parse_absolute_zoom_mm_command_assign() {
        let command_data = [0x00, 0x07, 0x02, 0x00, 0x10, 0x00];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::AbsoluteZoomMM(Operation::Assign, 16))
        );
    }

    #[test]
    fn parse_absolute_zoom_mm_command_increment() {
        let command_data = [0x00, 0x07, 0x02, 0x01, 0x10, 0x00];
        let command_data = CommandData::new(&command_data).expect("Should parse");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::AbsoluteZoomMM(Operation::Increment, 16))
        );
    }

    #[test]
    fn parse_absolute_zoom_mm_command_wrong_data_type() {
        let command_data = [0x00, 0x07, 0x01, 0x00, 0x10, 0x00];
        let command_data = CommandData::new(&command_data).expect("Should parse");
        let command = super::parse_command(command_data);
        assert_eq!(command, Err(EldritchError::InvalidCommandData));
    }

    #[test]
    fn parse_absolute_zoom_mm_command_value_below_bounds() {
        let command_data = [0x00, 0x07, 0x02, 0x00, 0xfe, 0xff];
        let command_data = CommandData::new(&command_data).expect("Should parse");
        let command = super::parse_command(command_data);
        assert_eq!(command, Err(EldritchError::DataOutOfBounds));
    }

    #[test]
    fn parse_absolute_zoom_normalized_command_assign() {
        let command_data = [0x00, 0x08, 0x80, 0x00, 0xff, 0x00];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::AbsoluteZoomNormalized(
                Operation::Assign,
                FixedPointDecimal {
                    raw_val: 0x00ffu16 as i16
                }
            ))
        );
    }

    #[test]
    fn parse_absolute_zoom_normalized_command_increment() {
        let command_data = [0x00, 0x08, 0x80, 0x01, 0xff, 0x00];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::AbsoluteZoomNormalized(
                Operation::Increment,
                FixedPointDecimal {
                    raw_val: 0x00ffu16 as i16
                }
            ))
        );
    }

    #[test]
    fn parse_absolute_zoom_normalized_command_below_bounds() {
        // Value -0.1 is below the bound of 0.0
        let command_data = [0x00, 0x08, 0x80, 0x00, 0xff, 0xff];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(command, Err(EldritchError::DataOutOfBounds));
    }

    #[test]
    fn parse_absolute_zoom_normalized_command_above_bounds() {
        // Value 1.1 is above the bound of 1.0
        let command_data = [0x00, 0x08, 0x80, 0x00, 0xcc, 0x08];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(command, Err(EldritchError::DataOutOfBounds));
    }

    #[test]
    fn parse_absolute_zoom_normalized_command_wrong_data_type() {
        let command_data = [0x00, 0x08, 0x01, 0x00, 0xcc, 0x08];
        let command_data = CommandData::new(&command_data).expect("Known good packet");
        let command = super::parse_command(command_data);
        assert_eq!(command, Err(EldritchError::InvalidCommandData));
    }

    #[test]
    fn parse_absolute_zoom_continuous_command() {
        let command_data = [0x00, 0x09, 0x80, 0x00, 0xff, 0x00];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::AbsoluteZoomContinuous(Operation::Assign, FixedPointDecimal {
                raw_val: 0x00ffu16 as i16
            }))
        );
    }

    #[test]
    fn parse_absolute_zoom_continuous_command_below_bounds() {
        // Value -1.1 is below the bound of -1.0
        let command_data = [0x00, 0x09, 0x80, 0x00, 0x34, 0xf7];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(command, Err(EldritchError::DataOutOfBounds));
    }

    #[test]
    fn parse_absolute_zoom_continuous_command_above_bounds() {
        // Value 1.1 is above the bound of 1.0
        let command_data = [0x00, 0x09, 0x80, 0x00, 0xcc, 0x08];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(command, Err(EldritchError::DataOutOfBounds));
    }

    #[test]
    fn parse_absolute_zoom_continuous_command_wrong_data_type() {
        let command_data = [0x00, 0x09, 0x01, 0x00, 0xcc, 0x08];
        let command_data = CommandData::new(&command_data).expect("Known good packet");
        let command = super::parse_command(command_data);
        assert_eq!(command, Err(EldritchError::InvalidCommandData));
    }
}
