use crate::{FixedPointDecimal, Operation, error::EldritchError};

use super::CommandData;

#[derive(Clone, Debug, PartialEq)]
pub enum LensCommand {
    Focus(Operation, FixedPointDecimal),
    InstantaneousAutoFocus,
    ApatureFStop(Operation, FixedPointDecimal),
    ApatureNormalized(Operation, FixedPointDecimal),
    OpticalImageStabalization(Operation, bool),
    NoOp,
}

pub fn parse_lens_command(command_data: CommandData) -> Result<LensCommand, EldritchError> {
    type Command = LensCommand;

    match command_data.parameter {
        0x00 => parse_focus_command(command_data),
        0x01 => Ok(Command::InstantaneousAutoFocus),
        0x02 => parse_apature_fstop_command(command_data),
        0x03 => parse_apature_normalized_command(command_data),
        0x06 => parse_ois_command(command_data),
        _ => Ok(Command::NoOp),
    }
}

fn parse_focus_command(cmd_data: CommandData) -> Result<LensCommand, EldritchError> {
    if let Ok(data) = cmd_data.data_buff.try_into() {
        let data = FixedPointDecimal::from_data(data);
        Ok(LensCommand::Focus(
            if *cmd_data.operation == 0 {
                Operation::Assign
            } else {
                Operation::Increment
            },
            data,
        ))
    } else {
        Err(EldritchError::InvalidCommandData)
    }
}

fn parse_apature_fstop_command(cmd_data: CommandData) -> Result<LensCommand, EldritchError> {
    if let Ok(data) = cmd_data.data_buff.try_into() {
        Ok(LensCommand::ApatureFStop(
            if *cmd_data.operation == 0 {
                Operation::Assign
            } else {
                Operation::Increment
            },
            FixedPointDecimal::from_data(data),
        ))
    } else {
        Err(EldritchError::InvalidCommandData)
    }
}

fn parse_apature_normalized_command(cmd_data: CommandData) -> Result<LensCommand, EldritchError> {
    if let Ok(data) = cmd_data.data_buff.try_into() {
        Ok(LensCommand::ApatureNormalized(
                if *cmd_data.operation == 0 {
                    Operation::Assign
                } else {
                    Operation::Increment
                },
                FixedPointDecimal::from_data(data),
        ))
    } else {
        Err(EldritchError::InvalidCommandData)
    }
}

fn parse_ois_command(cmd_data: CommandData) -> Result<LensCommand, EldritchError> {
    Ok(LensCommand::OpticalImageStabalization(
        if *cmd_data.operation == 0 {
            Operation::Assign
        } else {
            return Err(EldritchError::InvalidCommandData);
        },
        cmd_data.data_buff[0] != 0,
    ))
}

#[cfg(test)]
mod lens_commands {
    use crate::commands::parse_command_data;
    use super::*;

    #[test]
    fn parse_focus_command() {
        let command_packet_data: [u8; 6] = [0x00, 0x00, 0x80, 0x01, 0x9a, 0xfd];
        let command_data = parse_command_data(&command_packet_data).expect("Known good packet data");
        let command = super::parse_lens_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::Focus(
                Operation::Increment,
                FixedPointDecimal {
                    raw_val: 0xfd9au16 as i16
                }
            ))
        );
    }

    #[test]
    fn parse_auto_focus_command() {
        let command_packet_data = [0x00, 0x01, 0x00, 0x00];
        let command_data = parse_command_data(&command_packet_data).expect("Known good packet data");
        let command = parse_lens_command(command_data);
        assert_eq!(command, Ok(LensCommand::InstantaneousAutoFocus));
    }

    #[test]
    fn parse_ois_command_on() {
        let command_data = [0x00, 0x06, 0x00, 0x00, 0x01];
        let command_data = parse_command_data(&command_data).expect("Known good packet data");
        let command = super::parse_lens_command(command_data);
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
        let command_data = parse_command_data(&command_data).expect("Known good packet data");
        let command = super::parse_lens_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::OpticalImageStabalization(
                    Operation::Assign,
                    false
            ))
        );
    }
}
