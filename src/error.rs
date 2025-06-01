use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum EldritchError {
    EndOfPacket,
    InvalidHeader,
    PacketToLarge,
    PaddingViolation(String),
}

impl fmt::Display for EldritchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EldritchError::EndOfPacket => write!(f, "Attempting to retrieve more data at the end of packet"),
            EldritchError::PacketToLarge => write!(f, "Blanking packet is larger then 255 bytes"),
            EldritchError::InvalidHeader => write!(f, "Command Header is invlid"),
            EldritchError::PaddingViolation(msg) => write!(f, "{}", msg),
        }
    }
}

//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{{ file: {}, line: {} }}", file!(), line!())
//     }
// }
