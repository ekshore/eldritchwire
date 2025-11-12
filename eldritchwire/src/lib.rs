pub mod commands;
mod error;
use commands::Command;
use error::EldritchError;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub struct AddressedCommand {
    pub device_id: u8,
    pub command: Command,
}

#[derive(Clone, PartialEq, PartialOrd)]
pub struct FixedPointDecimal {
    raw_val: i16,
}

impl FixedPointDecimal {
    pub fn get_real_val(&self) -> f32 {
        f32::from(self.raw_val) / 2_f32.powi(11)
    }

    pub fn get_rounded_val(&self) -> f32 {
        (self.get_real_val() * 100.0).round() / 100.0
    }

    pub fn from_data(data: &[u8; 2]) -> Self {
        assert!(data.len() == 2);
        Self {
            raw_val: u16::from_le_bytes(*data) as i16,
        }
    }
}

impl Debug for FixedPointDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FixedPointDecimal {{ raw_val: {}, real_val: {} }}",
            self.raw_val,
            self.get_real_val()
        )
    }
}

impl PartialEq<f32> for FixedPointDecimal {
    fn eq(&self, other: &f32) -> bool {
        &self.get_real_val() == other
    }
}

impl PartialOrd<f32> for FixedPointDecimal {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        let real_val = self.get_real_val();
        if &real_val == other {
            Some(std::cmp::Ordering::Equal)
        } else if &real_val < other {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Greater)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    Assign,
    Increment,
    Toggle,
}

#[derive(Clone, Debug, PartialEq)]
struct CommandHeader {
    device_id: u8,
    command_length: u8,
    command_id: u8,
}

#[derive(Debug, PartialEq)]
struct PacketData {
    data: Vec<u8>,
    cursor: u8,
}

impl PacketData {
    pub fn new(packet_data: Vec<u8>) -> Result<Self, EldritchError> {
        if packet_data.len() > 255 {
            Err(EldritchError::PacketToLarge)
        } else {
            Ok(Self {
                data: packet_data,
                cursor: 0,
            })
        }
    }

    pub fn parse_header(&mut self) -> Result<CommandHeader, EldritchError> {
        let device_id = self
            .data
            .get(self.cursor as usize)
            .ok_or(EldritchError::InvalidHeader)?;
        let command_length = self
            .data
            .get((self.cursor + 1) as usize)
            .ok_or(EldritchError::InvalidHeader)?;
        let command_id = self
            .data
            .get((self.cursor + 2) as usize)
            .ok_or(EldritchError::InvalidHeader)?;

        let header = if let Some(reserved) = self.data.get((self.cursor + 3) as usize) {
            if *reserved == 0 {
                Ok(CommandHeader {
                    device_id: *device_id,
                    command_length: *command_length,
                    command_id: *command_id,
                })
            } else {
                Err(EldritchError::InvalidHeader)
            }
        } else {
            Err(EldritchError::InvalidHeader)
        };
        self.cursor += 4;
        header
    }

    fn has_data(&self) -> bool {
        usize::from(self.cursor) < self.data.len()
    }

    fn get_slice(&mut self, slice_len: u8) -> Result<&[u8], EldritchError> {
        let new_cur = self.cursor + slice_len;
        if usize::from(new_cur) > self.data.len() {
            return Err(EldritchError::EndOfPacket);
        }
        let slice_data = &self.data[usize::from(self.cursor)..usize::from(new_cur)];
        self.cursor = new_cur;
        Ok(slice_data)
    }
}

pub fn parse_frame_packet(data: Vec<u8>) -> Result<Vec<AddressedCommand>, EldritchError> {
    let mut packet = PacketData::new(data)?;
    let mut commands: Vec<AddressedCommand> = Vec::new();

    while packet.has_data() {
        let header = packet.parse_header()?;
        // println!("Command Header: {header:?}");
        match packet.get_slice(header.command_length) {
            Ok(command_data) => {
                // println!("Command Data: {command_data:?}");
                commands.push(AddressedCommand {
                    device_id: header.device_id,
                    command: commands::parse_command(command_data)?,
                });
            }
            Err(EldritchError::EndOfPacket) => break,
            Err(err) => Err(err)?,
        };

        match packet.get_slice(calculate_padding_length(header.command_length)) {
            Ok(padding) => verify_padding(padding, header.command_length)?,
            Err(EldritchError::EndOfPacket) => break,
            Err(err) => Err(err)?,
        };
    }

    Ok(commands)
}

fn calculate_padding_length(command_length: u8) -> u8 {
    if command_length % 4 == 0 {
        0
    } else {
        4 - (command_length % 4)
    }
}

fn verify_padding(padding: &[u8], command_length: u8) -> Result<(), EldritchError> {
    if (padding.len() + usize::from(command_length)) % 4 > 0 {
        return Err(EldritchError::PaddingViolation(String::from(
            "Padding length is incorrect",
        )));
    }

    let mut padding_errors: Vec<Result<(), EldritchError>> = padding
        .iter()
        .enumerate()
        .map(|(idx, byte)| {
            if *byte != 0x00 {
                Err(EldritchError::PaddingViolation(format!(
                    "Padding byte at index: {idx} is not 0x00",
                )))
            } else {
                Ok(())
            }
        })
        .filter(|check| check.is_err())
        .collect();

    if !padding_errors.is_empty() {
        padding_errors.remove(0)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod fixed_point_test {
    use super::*;

    #[test]
    fn fpd_fifteen_percent() {
        let fifteen_percent = FixedPointDecimal { raw_val: 0x0133 };
        let float_value = fifteen_percent.get_rounded_val();
        assert_eq!(float_value, 0.15);
    }

    #[test]
    fn fpd_minus_point_three() {
        let minus_point_three = FixedPointDecimal {
            raw_val: 0xfd9au16 as i16,
        };
        let float_value = minus_point_three.get_rounded_val();
        assert_eq!(float_value, -0.3_f32);
    }
}

#[cfg(test)]
mod packet_data_test {
    use crate::commands::{
        color_correction_commands::{ColorCorrectionCommand, RedGreenBlueLuma},
        display_commands::DisplayCommand,
        lens_commands::LensCommand,
        video_commands::{VideoCommand, VideoModeData},
    };

    use super::*;

    #[test]
    fn new() {
        let packet_data = vec![
            0x00, 0x05, 0x00, 0x00, // Header
            0x00, 0x80, 0x01, 0x9a, 0xfd, // Command
            0x00, 0x00, 0x00, // Padding
        ];

        if let Ok(packet) = PacketData::new(packet_data) {
            assert_eq!(packet.cursor, 0);
            assert_eq!(packet.data.len(), 12);
        } else {
            panic!();
        }
    }

    #[test]
    fn parse_header() {
        let packet_data = vec![
            0x00, 0x05, 0x00, 0x00, // Header
            0x00, 0x80, 0x01, 0x9a, 0xfd, // Command
            0x00, 0x00, 0x00, // Padding
        ];

        let mut packet =
            PacketData::new(packet_data).expect("Failed to build PacketData with known good data");

        if let Ok(header) = packet.parse_header() {
            assert_eq!(
                header,
                CommandHeader {
                    device_id: 0,
                    command_length: 5,
                    command_id: 0
                }
            );
        } else {
            panic!();
        }
    }

    #[test]
    fn parse_header_bad_reserved_byte() {
        let packet_data = vec![
            0x00, 0x05, 0x00, 0xFF, // Header
            0x00, 0x80, 0x01, 0x9a, 0xfd, // Command
            0x00, 0x00, 0x00, // Padding
        ];

        let mut packet = PacketData::new(packet_data).expect("Verified packet data");

        if let Err(error) = packet.parse_header() {
            assert_eq!(error, EldritchError::InvalidHeader);
            assert_eq!(4, packet.cursor);
        } else {
            panic!();
        }
    }

    #[test]
    fn has_data_true() {
        let packet_data = vec![
            0x00, 0x05, 0x00, 0xFF, // Header
            0x00, 0x80, 0x01, 0x9a, 0xfd, // Command
            0x00, 0x00, 0x00, // Padding
        ];

        let mut packet = PacketData::new(packet_data).expect("Verified packet data");
        packet.cursor = 10;
        println!("Packet: {:?}", packet);
        assert!(packet.has_data());
    }

    #[test]
    fn has_data_false() {
        let packet_data = vec![
            0x00, 0x05, 0x00, 0xFF, // Header
            0x00, 0x80, 0x01, 0x9a, 0xfd, // Command
            0x00, 0x00, 0x00, // Padding
        ];

        let mut packet = PacketData::new(packet_data).expect("Verified packet data");
        packet.cursor = 12;
        println!("Packet: {:?}", packet);
        assert!(!packet.has_data());
    }

    #[test]
    fn get_slice() {
        let packet_data = vec![
            0x00, 0x05, 0x00, 0xFF, // Header
            0x00, 0x80, 0x01, 0x9a, 0xfd, // Command
            0x00, 0x00, 0x00, // Padding
        ];
        let mut packet = PacketData::new(packet_data).expect("Verified packet data");
        packet.cursor = 4;

        if let Ok(cmd_data) = packet.get_slice(5) {
            assert_eq!([0x00, 0x80, 0x01, 0x9a, 0xfd,] as [u8; 5], cmd_data);
            assert_eq!(9, packet.cursor);
        } else {
            panic!();
        }
        if let Ok(padding) = packet.get_slice(calculate_padding_length(5)) {
            assert_eq!([0x00, 0x00, 0x00] as [u8; 3], padding);
            assert_eq!(12, packet.cursor);
        } else {
            panic!();
        }
    }

    #[test]
    fn get_slice_error() {
        let packet_data = vec![
            0x00, 0x05, 0x00, 0xFF, // Header
            0x00, 0x80, 0x01, 0x9a, 0xfd, // Command
            0x00, 0x00, 0x00, // Padding
        ];
        let mut packet = PacketData::new(packet_data).expect("Verified packet data");
        packet.cursor = 8;

        if let Err(error) = packet.get_slice(5) {
            assert_eq!(EldritchError::EndOfPacket, error);
        } else {
            panic!();
        }
    }

    #[test]
    fn parse_large_packet() {
        let frame_packet = vec![
            0x04, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0xff, 0x05, 0x00, 0x00, 0x00, 0x06,
            0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x04, 0x08, 0x00, 0x00, 0x01, 0x05, 0x03, 0x00,
            0x10, 0x27, 0x00, 0x00, 0x04, 0x06, 0x00, 0x00, 0x04, 0x02, 0x80, 0x01, 0x33, 0x01,
            0x00, 0x00, 0xff, 0x09, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x18, 0x01, 0x03, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x04, 0x0c, 0x00, 0x00, 0x08, 0x01, 0x80, 0x01, 0x00, 0x00,
            0x9a, 0xfd, 0x9a, 0xfd, 0x00, 0x00,
        ];
        let commands = parse_frame_packet(frame_packet).expect("Test Frame should parse");
        let mut commands = commands.iter();

        assert_eq!(
            commands.next(),
            Some(AddressedCommand {
                device_id: 4,
                command: Command::Lens(LensCommand::InstantaneousAutoFocus),
            })
            .as_ref()
        );

        assert_eq!(
            commands.next(),
            Some(AddressedCommand {
                device_id: 255,
                command: Command::Lens(LensCommand::OpticalImageStabalization {
                    operation: Operation::Assign,
                    data: true
                })
            })
            .as_ref()
        );

        assert_eq!(
            commands.next(),
            Some(AddressedCommand {
                device_id: 4,
                command: Command::Video(VideoCommand::ExposureUS {
                    operation: Operation::Assign,
                    data: 10000
                })
            })
            .as_ref()
        );

        assert_eq!(
            commands.next(),
            Some(AddressedCommand {
                device_id: 4,
                command: Command::Display(DisplayCommand::ZebraLevel {
                    operation: Operation::Increment,
                    data: FixedPointDecimal {
                        raw_val: 0x0133u16 as i16
                    }
                })
            })
            .as_ref()
        );

        assert_eq!(
            commands.next(),
            Some(AddressedCommand {
                device_id: 255,
                command: Command::Video(VideoCommand::VideoMode {
                    operation: Operation::Assign,
                    data: VideoModeData {
                        frame_rate: 24,
                        m_rate: 1,
                        dimensions: 3,
                        interlaced: 0,
                        color_space: 0,
                    }
                })
            })
            .as_ref()
        );

        assert_eq!(
            commands.next(),
            Some(AddressedCommand {
                device_id: 4,
                command: Command::ColorCorrection(ColorCorrectionCommand::GammaAdjust {
                    operation: Operation::Increment,
                    data: RedGreenBlueLuma {
                        red: FixedPointDecimal { raw_val: 0x00 },
                        green: FixedPointDecimal {
                            raw_val: 0xfd9au16 as i16
                        },
                        blue: FixedPointDecimal {
                            raw_val: 0xfd9au16 as i16
                        },
                        luma: FixedPointDecimal { raw_val: 0x00 },
                    }
                })
            })
            .as_ref()
        );
    }

    #[test]
    fn long_real_packet_one() {
        let cmd_data: [u8; 243] = [
            0x01, 0x06, 0x00, 0x00, 0x00, 0x02, 0x80, 0x00, 0x8D, 0x1C, 0x00, 0x00, 0x01, 0x06,
            0x00, 0x00, 0x01, 0x10, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x05, 0x00, 0x00,
            0x01, 0x0D, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x05, 0x00, 0x00, 0x01, 0x01,
            0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x08, 0x00, 0x00, 0x01, 0x02, 0x02, 0x00,
            0x04, 0x10, 0x00, 0x00, 0x01, 0x08, 0x00, 0x00, 0x01, 0x05, 0x03, 0x00, 0x1B, 0x41,
            0x00, 0x00, 0x01, 0x05, 0x00, 0x00, 0x01, 0x08, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00,
            0x01, 0x05, 0x00, 0x00, 0x04, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x0C,
            0x00, 0x00, 0x08, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45, 0x00,
            0x01, 0x0C, 0x00, 0x00, 0x08, 0x01, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x01, 0x0C, 0x00, 0x00, 0x08, 0x02, 0x80, 0x00, 0x00, 0x08, 0x00, 0x08,
            0x00, 0x08, 0xCE, 0x07, 0x01, 0x0C, 0x00, 0x00, 0x08, 0x03, 0x80, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x08, 0x00, 0x00, 0x08, 0x04, 0x80, 0x00,
            0x00, 0x04, 0x00, 0x08, 0x01, 0x06, 0x00, 0x00, 0x08, 0x05, 0x80, 0x00, 0x00, 0x08,
            0x00, 0x00, 0x01, 0x08, 0x00, 0x00, 0x08, 0x06, 0x80, 0x00, 0x00, 0x00, 0x00, 0x08,
            0x02, 0x06, 0x00, 0x00, 0x00, 0x02, 0x80, 0x00, 0x9B, 0x12, 0x00, 0x00, 0x02, 0x06,
            0x00, 0x00, 0x01, 0x10, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x05, 0x00, 0x00,
            0x01, 0x0D, 0x01, 0x00, 0x08, 0x00, 0x00, 0x00, 0x02, 0x05, 0x00, 0x00, 0x01, 0x01,
            0x01, 0x00, 0x08, 0x00, 0x00,
        ];
        let result = parse_frame_packet(cmd_data.to_vec());
        let _commands = result.unwrap();
    }

    #[test]
    fn long_real_packet_two() {
        let cmd_data: [u8; 251] = [
            0x03, 0x0C, 0x00, 0x00, 0x08, 0x02, 0x80, 0x00, 0x00, 0x08, 0x00, 0x08, 0x00, 0x08,
            0x00, 0x08, 0x03, 0x0C, 0x00, 0x00, 0x08, 0x03, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x03, 0x08, 0x00, 0x00, 0x08, 0x04, 0x80, 0x00, 0x00, 0x04,
            0x00, 0x08, 0x03, 0x06, 0x00, 0x00, 0x08, 0x05, 0x80, 0x00, 0x00, 0x08, 0x00, 0x00,
            0x03, 0x08, 0x00, 0x00, 0x08, 0x06, 0x80, 0x00, 0x00, 0x00, 0x00, 0x08, 0x04, 0x06,
            0x00, 0x00, 0x00, 0x02, 0x80, 0x00, 0xF2, 0x15, 0x00, 0x00, 0x04, 0x06, 0x00, 0x00,
            0x01, 0x10, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x05, 0x00, 0x00, 0x01, 0x0D,
            0x01, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x04, 0x05, 0x00, 0x00, 0x01, 0x01, 0x01, 0x00,
            0x02, 0x00, 0x00, 0x00, 0x04, 0x08, 0x00, 0x00, 0x01, 0x02, 0x02, 0x00, 0x04, 0x10,
            0x00, 0x00, 0x04, 0x08, 0x00, 0x00, 0x01, 0x05, 0x03, 0x00, 0x1B, 0x41, 0x00, 0x00,
            0x04, 0x05, 0x00, 0x00, 0x01, 0x08, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x04, 0x05,
            0x00, 0x00, 0x04, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x0C, 0x00, 0x00,
            0x08, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x0C,
            0x00, 0x00, 0x08, 0x01, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x04, 0x0C, 0x00, 0x00, 0x08, 0x02, 0x80, 0x00, 0x00, 0x08, 0x00, 0x08, 0x00, 0x08,
            0x00, 0x08, 0x04, 0x0C, 0x00, 0x00, 0x08, 0x03, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x04, 0x08, 0x00, 0x00, 0x08, 0x04, 0x80, 0x00, 0x00, 0x04,
            0x00, 0x08, 0x04, 0x06, 0x00, 0x00, 0x08, 0x05, 0x80, 0x00, 0x00, 0x08, 0x00,
        ];
        let result = parse_frame_packet(cmd_data.to_vec());
        let _commands = result.unwrap();
    }

    #[test]
    fn long_real_packet_three() {
        let cmd_data: [u8; 247] = [
            0x10, 0x05, 0x00, 0x00, 0x04, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x0C,
            0x00, 0x00, 0x08, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x10, 0x0C, 0x00, 0x00, 0x08, 0x01, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x10, 0x0C, 0x00, 0x00, 0x08, 0x02, 0x80, 0x00, 0x00, 0x08, 0x00, 0x08,
            0x00, 0x08, 0x00, 0x08, 0x10, 0x0C, 0x00, 0x00, 0x08, 0x03, 0x80, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x08, 0x00, 0x00, 0x08, 0x04, 0x80, 0x00,
            0x00, 0x04, 0x00, 0x08, 0x10, 0x06, 0x00, 0x00, 0x08, 0x05, 0x80, 0x00, 0x00, 0x08,
            0x00, 0x00, 0x10, 0x08, 0x00, 0x00, 0x08, 0x06, 0x80, 0x00, 0x00, 0x00, 0x00, 0x08,
            0x11, 0x06, 0x00, 0x00, 0x01, 0x10, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x05,
            0x00, 0x00, 0x01, 0x0D, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x05, 0x00, 0x00,
            0x01, 0x01, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x11, 0x08, 0x00, 0x00, 0x01, 0x05,
            0x03, 0x00, 0x20, 0x4E, 0x00, 0x00, 0x11, 0x05, 0x00, 0x00, 0x01, 0x08, 0x01, 0x00,
            0x01, 0x00, 0x00, 0x00, 0x11, 0x05, 0x00, 0x00, 0x04, 0x04, 0x01, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x11, 0x0C, 0x00, 0x00, 0x08, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x11, 0x0C, 0x00, 0x00, 0x08, 0x01, 0x80, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x0C, 0x00, 0x00, 0x08, 0x02, 0x80, 0x00,
            0x00, 0x08, 0x00, 0x08, 0x00, 0x08, 0x00, 0x08, 0x11, 0x0C, 0x00, 0x00, 0x08, 0x03,
            0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let result = parse_frame_packet(cmd_data.to_vec());
        let _commands = result.unwrap();
    }

    #[test]
    fn short_real_packet_one() {
        let cmd_data: [u8; 23] = [
            0x02, 0x06, 0x00, 0x00, 0x00, 0x02, 0x80, 0x00, 0x64, 0x20, 0x00, 0x00, 0x02, 0x06,
            0x00, 0x00, 0x00, 0x02, 0x80, 0x00, 0x07, 0x21, 0x00,
        ];
        let result = parse_frame_packet(cmd_data.to_vec());
        let _commands = result.unwrap();
    }
}

#[cfg(test)]
mod lib_test {
    use super::*;
    use crate::commands::{lens_commands::LensCommand, Command};

    #[test]
    fn parse_packet_single_command() {
        let packet_data = vec![
            0x00, 0x06, 0x00, 0x00, // Header
            0x00, 0x00, 0x80, 0x01, 0x33, 0x01, // Command
            0x00, 0x00, // Padding
        ];

        if let Ok(commands) = parse_frame_packet(packet_data) {
            println!("parse_packet_single_command() commands: {:?}", commands);
            assert_eq!(1, commands.len());
            assert_eq!(
                AddressedCommand {
                    device_id: 0,
                    command: Command::Lens(LensCommand::Focus {
                        operation: Operation::Increment,
                        data: FixedPointDecimal {
                            raw_val: 0x0133u16 as i16
                        }
                    })
                },
                *commands.first().expect("Length asserted to be one")
            );
        } else {
            panic!();
        }
    }

    #[test]
    fn parse_packet_two_commands() {
        let packet_data = vec![
            0x00, 0x06, 0x00, 0x00, // Header
            0x00, 0x00, 0x80, 0x01, 0x33, 0x01, // Command
            0x00, 0x00, // Padding
            0x00, 0x06, 0x00, 0x00, // Header
            0x00, 0x00, 0x80, 0x01, 0x33, 0x01, // Command
            0x00, 0x00, // Padding
        ];

        if let Ok(commands) = parse_frame_packet(packet_data) {
            println!("parse_packet_single_command() commands: {:?}", commands);
            assert_eq!(2, commands.len());
            assert_eq!(
                AddressedCommand {
                    device_id: 0,
                    command: Command::Lens(LensCommand::Focus {
                        operation: Operation::Increment,
                        data: FixedPointDecimal {
                            raw_val: 0x0133u16 as i16
                        }
                    })
                },
                *commands
                    .first()
                    .expect("Length asserted to be more then one")
            );
            assert_eq!(
                AddressedCommand {
                    device_id: 0,
                    command: Command::Lens(LensCommand::Focus {
                        operation: Operation::Increment,
                        data: FixedPointDecimal {
                            raw_val: 0x0133u16 as i16
                        }
                    })
                },
                *commands.get(1).expect("Length asserted to be two")
            );
        } else {
            panic!();
        }
    }

    #[test]
    fn calculate_padding_length_no_padding() {
        assert_eq!(0_u8, calculate_padding_length(8));
        assert_eq!(0_u8, calculate_padding_length(4));
    }

    #[test]
    fn calculate_padding_length_one() {
        assert_eq!(1_u8, calculate_padding_length(3));
        assert_eq!(1_u8, calculate_padding_length(7));
    }

    #[test]
    fn calculate_padding_length_two() {
        assert_eq!(2_u8, calculate_padding_length(2));
        assert_eq!(2_u8, calculate_padding_length(6));
    }

    #[test]
    fn calculate_padding_length_three() {
        assert_eq!(3_u8, calculate_padding_length(1));
        assert_eq!(3_u8, calculate_padding_length(5));
    }

    #[test]
    fn verify_padding_success() {
        let padding: [u8; 3] = [0x00; 3];
        let command_length = 5;

        assert_eq!(Ok(()), verify_padding(&padding, command_length));
    }

    #[test]
    fn verify_padding_too_short() {
        let padding: [u8; 2] = [0x00; 2];
        let command_length = 5;

        assert_eq!(
            Err(EldritchError::PaddingViolation(String::from(
                "Padding length is incorrect"
            ))),
            verify_padding(&padding, command_length)
        );
    }

    #[test]
    fn verify_padding_nonzero_byte_idx_0() {
        let padding: [u8; 3] = [0xff, 0x00, 0x00];
        let command_length = 5;

        assert_eq!(
            Err(EldritchError::PaddingViolation(String::from(
                "Padding byte at index: 0 is not 0x00"
            ))),
            verify_padding(&padding, command_length),
        )
    }

    #[test]
    fn verify_padding_nonzero_byte_idx_2() {
        let padding: [u8; 3] = [0x00, 0x00, 0xff];
        let command_length = 5;

        assert_eq!(
            Err(EldritchError::PaddingViolation(String::from(
                "Padding byte at index: 2 is not 0x00"
            ))),
            verify_padding(&padding, command_length),
        )
    }
}
