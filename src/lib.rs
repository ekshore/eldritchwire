mod commands;
mod error;
use commands::Command;
use error::EldritchError;

#[derive(Clone, Debug, PartialEq)]
pub struct FixedPointDecimal {
    raw_val: i16,
}

impl FixedPointDecimal {
    pub fn get_real_val(self) -> f32 {
        println!("raw_val: {}", self.raw_val);
        f32::from(self.raw_val) / 2_f32.powi(11)
    }

    pub fn get_rounded_val(self) -> f32 {
        (self.get_real_val() * 100.0).round() / 100.0
    }

    pub fn from_data(data: &[u8; 2]) -> Self {
        assert!(data.len() == 2);
        Self {
            raw_val: u16::from_le_bytes(*data) as i16,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    Assign,
    Increment,
    Toggle,
}

struct PacketData {
    data: Vec<u8>,
    cursor: u8,
}

#[derive(Clone, Debug, PartialEq)]
struct CommandHeader {
    device_id: u8,
    command_length: u8,
    command_id: u8,
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

        if let Some(reserved) = self.data.get((self.cursor + 3) as usize) {
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
        }
    }

    fn has_data(&self) -> bool {
        usize::from(self.cursor) < self.data.len() || self.cursor < 255
    }

    fn get_slice(&mut self, slice_len: u8) -> Result<&[u8], EldritchError> {
        let new_cur = self.cursor + slice_len;
        if usize::from(new_cur) > self.data.len() {
            todo!();
        }
        !todo!();
    }
}

pub fn parse_packet(data: Vec<u8>) -> Result<Vec<Command>, EldritchError> {
    let mut packet = PacketData::new(data)?;

    while packet.has_data() {
        let header = packet.parse_header()?;

        let command_data = packet.get_slice(header.command_length)?;
        let command = commands::parse_command(header.command_id, command_data);

        let padding = packet.get_slice(header.command_length % 4)?;
        verify_padding(padding, header.command_length)?;
    }

    Ok(vec![])
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

    if padding_errors.len() > 0 {
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
    use super::*;

    #[test]
    fn new() {
        let packet_data = vec![
            0x00, 0x05, 0x00, 0x00, // Header
            0x00, 0x80, 0x01, 0x9a, 0xfd, // Command
            0x00, 0x00, 0x00, // Padding
        ];

        if let Ok(packet) = PacketData::new(packet_data) {
            assert!(true);
            assert_eq!(packet.cursor, 0);
            assert_eq!(packet.data.len(), 12);
        } else {
            assert!(false);
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
            assert!(false);
        }
    }

    #[test]
    fn parse_header_bad_reserved_byte() {
        let packet_data = vec![
            0x00, 0x05, 0x00, 0xFF, // Header
            0x00, 0x80, 0x01, 0x9a, 0xfd, // Command
            0x00, 0x00, 0x00, // Padding
        ];

        let mut packet =
            PacketData::new(packet_data).expect("Failed to build PacketData with known good data");

        match packet.parse_header() {
            Ok(_) => assert!(false),
            Err(error) => assert_eq!(error, EldritchError::InvalidHeader),
        }
    }
}

#[cfg(test)]
mod lib_test {
    use super::*;

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
