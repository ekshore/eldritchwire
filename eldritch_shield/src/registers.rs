#[allow(dead_code)]
pub struct Register {
    pub address: [u8; 2],
    pub length: usize,
}

pub const IDENTITY: &'static Register = &Register {
    address: [0x00, 0x00],
    length: 4,
};

pub const HARDWARE_VERSION: &'static Register = &Register {
    address: [0x04, 0x00],
    length: 2,
};

pub const FIRMWARE_VERSION: &'static Register = &Register {
    address: [0x06, 0x00],
    length: 2,
};

pub const CONTROL: &'static Register = &Register {
    address: [0x00, 0x10],
    length: 1,
};

pub const OUTPUT_CONTROL_ARM: &'static Register = &Register {
    address: [0x00, 0x20],
    length: 1,
};

pub const OUTPUT_CONTROL_LENGTH: &'static Register = &Register {
    address: [0x01, 0x20],
    length: 1,
};

pub const OUTPUT_CONTROL_DATA: &'static Register = &Register {
    address: [0x00, 0x21],
    length: 0xfe,
};

pub const INCOMING_CONTROL_ARM: &'static Register = &Register {
    address: [0x00, 0x30],
    length: 1,
};

pub const INCOMING_CONTROL_LENGTH: &'static Register = &Register {
    address: [0x01, 0x30],
    length: 1,
};

pub const INCOMING_CONTROL_DATA: &'static Register = &Register {
    address: [0x00, 0x31],
    length: 0xfe,
};

pub const OUTPUT_TALLY_ARM: &'static Register = &Register {
    address: [0x00, 0x40],
    length: 1,
};

pub const OUTPUT_TALLY_LENGTH: &'static Register = &Register {
    address: [0x01, 0x40],
    length: 1,
};

pub const OUTPUT_TALLY_DATA: &'static Register = &Register {
    address: [0x00, 0x41],
    length: 0xfe,
};

pub const INCOMING_TALLY_ARM: &'static Register = &Register {
    address: [0x00, 0x50],
    length: 1,
};

pub const INCOMING_TALLY_LENGTH: &'static Register = &Register {
    address: [0x01, 0x50],
    length: 1,
};

pub const INCOMING_TALLY_DATA: &'static Register = &Register {
    address: [0x00, 0x51],
    length: 0xfe,
};
