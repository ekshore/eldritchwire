use crate::{error::EldritchError, FixedPointDecimal, Operation};
use eldritchwire_macros::CommandGroup;

use super::CommandData;

#[derive(Clone, Debug, PartialEq, CommandGroup)]
pub enum LensCommand {
    #[command(parameter(0x00), data_type(128), bounds(lower(0.0), upper(1.0)))]
    Focus {
        operation: Operation,
        data: FixedPointDecimal,
    },

    #[command(parameter(0x01))]
    InstantaneousAutoFocus,

    #[command(parameter(0x02), data_type(128), bounds(lower(-1.0), upper(16.0)))]
    ApatureFStop {
        operation: Operation,
        data: FixedPointDecimal,
    },

    #[command(parameter(0x03), data_type(128), bounds(lower(0.0), upper(1.0)))]
    ApatureNormalized {
        operation: Operation,
        data: FixedPointDecimal,
    },

    #[command(parameter(0x04), data_type(2), bounds(lower(0)))]
    ApatureOrdinal { operation: Operation, data: i16 },

    #[command(parameter(0x05))]
    InstantaneousAutoApature,

    #[command(parameter(0x06), data_type(0))]
    OpticalImageStabalization { operation: Operation, data: bool },

    #[command(parameter(0x07), data_type(2), bounds(lower(0)))]
    AbsoluteZoomMM { operation: Operation, data: i16 },

    #[command(parameter(0x08), data_type(128), bounds(lower(0.0), upper(1.0)))]
    AbsoluteZoomNormalized {
        operation: Operation,
        data: FixedPointDecimal,
    },

    #[command(parameter(0x09), data_type(128), bounds(lower(-1.0), upper(1.0)))]
    AbsoluteZoomContinuous {
        operation: Operation,
        data: FixedPointDecimal,
    },
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
            Ok(LensCommand::Focus {
                operation: Operation::Assign,
                data: FixedPointDecimal {
                    raw_val: 0x0133u16 as i16,
                }
            })
        );
    }

    #[test]
    fn parse_focus_command_increment() {
        let command_packet_data: [u8; 6] = [0x00, 0x00, 0x80, 0x01, 0x33, 0x01];
        let command_data = CommandData::new(&command_packet_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::Focus {
                operation: Operation::Increment,
                data: FixedPointDecimal {
                    raw_val: 0x0133u16 as i16
                }
            })
        );
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
            Ok(LensCommand::ApatureFStop {
                operation: Operation::Assign,
                data: FixedPointDecimal {
                    raw_val: 0xfd9au16 as i16
                }
            })
        );
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
            Ok(LensCommand::ApatureFStop {
                operation: Operation::Assign,
                data: FixedPointDecimal {
                    raw_val: 0x7fffu16 as i16
                }
            })
        );
    }

    #[test]
    fn parse_apature_normalized_assign() {
        let command_data = [0x00, 0x03, 0x80, 0x00, 0x00, 0x04];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::ApatureNormalized {
                operation: Operation::Assign,
                data: FixedPointDecimal {
                    raw_val: 0x0400u16 as i16
                }
            })
        );
    }

    #[test]
    fn parse_apature_ordinal_assign() {
        let command_data = [0x00, 0x04, 0x02, 0x00, 0x10, 0x27];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::ApatureOrdinal {
                operation: Operation::Assign,
                data: 10_000_i16
            })
        );
    }

    #[test]
    fn parse_apature_ordinal_command_bad_data_type() {
        let command_data = [0x00, 0x04, 0x01, 0x00, 0x10, 0x27];
        let command_data = CommandData::new(&command_data).expect("Should parse");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Err(EldritchError::InvalidCommandData {
                message: String::from("Invalid Data type for command"),
                data: vec![0x00, 0x04, 0x01, 0x00, 0x10, 0x27],
            })
        );
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
            Ok(LensCommand::OpticalImageStabalization {
                operation: Operation::Assign,
                data: true
            })
        );
    }

    #[test]
    fn parse_ois_command_off() {
        let command_data = [0x00, 0x06, 0x00, 0x00, 0x00];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::OpticalImageStabalization {
                operation: Operation::Assign,
                data: false
            })
        );
    }

    #[test]
    fn parse_absolute_zoom_mm_command_assign() {
        let command_data = [0x00, 0x07, 0x02, 0x00, 0x10, 0x00];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::AbsoluteZoomMM {
                operation: Operation::Assign,
                data: 16
            })
        );
    }

    #[test]
    fn parse_absolute_zoom_mm_command_increment() {
        let command_data = [0x00, 0x07, 0x02, 0x01, 0x10, 0x00];
        let command_data = CommandData::new(&command_data).expect("Should parse");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::AbsoluteZoomMM {
                operation: Operation::Increment,
                data: 16
            })
        );
    }

    #[test]
    fn parse_absolute_zoom_mm_command_wrong_data_type() {
        let command_data = [0x00, 0x07, 0x01, 0x00, 0x10, 0x00];
        let command_data = CommandData::new(&command_data).expect("Should parse");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Err(EldritchError::InvalidCommandData {
                message: String::from("Invalid Data type for command"),
                data: vec![0x00, 0x07, 0x01, 0x00, 0x10, 0x00],
            })
        );
    }

    #[test]
    fn parse_absolute_zoom_normalized_command_assign() {
        let command_data = [0x00, 0x08, 0x80, 0x00, 0xff, 0x00];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::AbsoluteZoomNormalized {
                operation: Operation::Assign,
                data: FixedPointDecimal {
                    raw_val: 0x00ffu16 as i16
                }
            })
        );
    }

    #[test]
    fn parse_absolute_zoom_normalized_command_increment() {
        let command_data = [0x00, 0x08, 0x80, 0x01, 0xff, 0x00];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::AbsoluteZoomNormalized {
                operation: Operation::Increment,
                data: FixedPointDecimal {
                    raw_val: 0x00ffu16 as i16
                }
            })
        );
    }

    #[test]
    fn parse_absolute_zoom_normalized_command_wrong_data_type() {
        let command_data = [0x00, 0x08, 0x01, 0x00, 0xcc, 0x08];
        let command_data = CommandData::new(&command_data).expect("Known good packet");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Err(EldritchError::InvalidCommandData {
                message: String::from("Invalid Data type for command"),
                data: vec![0x00, 0x08, 0x01, 0x00, 0xcc, 0x08]
            })
        );
    }

    #[test]
    fn parse_absolute_zoom_continuous_command() {
        let command_data = [0x00, 0x09, 0x80, 0x00, 0xff, 0x00];
        let command_data = CommandData::new(&command_data).expect("Known good packet data");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Ok(LensCommand::AbsoluteZoomContinuous {
                operation: Operation::Assign,
                data: FixedPointDecimal {
                    raw_val: 0x00ffu16 as i16
                }
            })
        );
    }

    #[test]
    fn parse_absolute_zoom_continuous_command_wrong_data_type() {
        let command_data = [0x00, 0x09, 0x01, 0x00, 0xcc, 0x08];
        let command_data = CommandData::new(&command_data).expect("Known good packet");
        let command = super::parse_command(command_data);
        assert_eq!(
            command,
            Err(EldritchError::InvalidCommandData {
                message: String::from("Invalid Data type for command"),
                data: vec![0x00, 0x09, 0x01, 0x00, 0xcc, 0x08]
            })
        );
    }

    #[allow(unexpected_cfgs)]
    #[cfg(feature = "bounds-checked")]
    mod check_bounds {
        use crate::{commands::CommandData, error::EldritchError};

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
        fn parse_apature_fstop_command_below_bounds() {
            // Value -1.1 (f734) is below the lower bound of -1
            let command_data = [0x00, 0x02, 0x80, 0x00, 0x34, 0xf7];
            let command_data = CommandData::new(&command_data).expect("Known good packet data");
            let command = super::parse_command(command_data);
            assert_eq!(command, Err(EldritchError::DataOutOfBounds));
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
        fn parse_apature_ordinal_command_assign_below_bounds() {
            // Value -1 is below the bound of 0.0
            let command_data = [0x00, 0x04, 0x02, 0x00, 0xff, 0xff];
            let command_data = CommandData::new(&command_data).expect("Known good packet data");
            let command = super::parse_command(command_data);
            assert_eq!(command, Err(EldritchError::DataOutOfBounds));
        }

        #[test]
        fn parse_absolute_zoom_mm_command_value_below_bounds() {
            let command_data = [0x00, 0x07, 0x02, 0x00, 0xfe, 0xff];
            let command_data = CommandData::new(&command_data).expect("Should parse");
            let command = super::parse_command(command_data);
            assert_eq!(command, Err(EldritchError::DataOutOfBounds));
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
    }
}
