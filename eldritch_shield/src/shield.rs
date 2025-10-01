use crate::errors::ShieldError;
use crate::registers;
use crate::traits::I2cTransport;

pub struct EldritchShield<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C, E> EldritchShield<I2C>
where
    I2C: I2cTransport<Error = E>,
{
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }

    pub fn with_i2c(i2c: I2C) -> Self {
        Self { i2c, address: 0x6e }
    }

    pub fn get_identity(&mut self) -> Result<String, ShieldError<E>> {
        let mut buff = [0u8; registers::IDENTITY.length];
        self.i2c.read(&registers::IDENTITY.address, &mut buff)?;
        Ok(String::from_utf8(buff.to_vec()).map_err(|_| ShieldError::InvalidResponse)?)
    }

    pub fn get_hardware_version(&mut self) -> Result<(u8, u8), ShieldError<E>> {
        let mut buff = [0u8; registers::HARDWARE_VERSION.length];
        self.i2c.read(&registers::IDENTITY.address, &mut buff)?;
        Ok((buff[0], buff[1]))
    }

    pub fn get_firmware_version(&mut self) -> Result<(u8, u8), ShieldError<E>> {
        let mut buff = [0u8; registers::FIRMWARE_VERSION.length];
        self.i2c
            .read(&registers::FIRMWARE_VERSION.address, &mut buff)?;
        Ok((buff[0], buff[1]))
    }

    pub fn get_system_control_register(&mut self) -> Result<u8, ShieldError<E>> {
        let mut buff = [0u8; registers::CONTROL.length];
        self.i2c.read(&registers::CONTROL.address, &mut buff)?;
        Ok(buff[0])
    }

    pub fn set_system_control_register(&mut self, byte: u8) -> Result<(), ShieldError<E>> {
        self.i2c.write(&registers::CONTROL.address, &[byte])?;
        Ok(())
    }

    pub fn set_system_control_override(&mut self, enabled: bool) -> Result<(), ShieldError<E>> {
        let mut buff = [0u8; registers::CONTROL.length];
        self.i2c.read(&registers::CONTROL.address, &mut buff)?;
        buff = if enabled {
            [buff[0] | 0b0000_0001]
        } else {
            [buff[0] & 0b0000_1110]
        };
        self.i2c.write(&registers::CONTROL.address, &buff)?;
        Ok(())
    }

    pub fn set_system_tally_override(&mut self, enabled: bool) -> Result<(), ShieldError<E>> {
        let mut buff = [0u8; registers::CONTROL.length];
        self.i2c.read(&registers::CONTROL.address, &mut buff)?;
        buff = if enabled {
            [buff[0] | 0b0000_0010]
        } else {
            [buff[0] & 0b0000_1101]
        };
        self.i2c.write(&registers::CONTROL.address, &buff)?;
        Ok(())
    }

    pub fn system_reset_tally(&mut self) -> Result<(), ShieldError<E>> {
        let mut buff = [0u8; registers::CONTROL.length];
        self.i2c.read(&registers::CONTROL.address, &mut buff)?;
        buff = [buff[0] | 0b0000_0100];
        self.i2c.write(&registers::CONTROL.address, &buff)?;
        Ok(())
    }

    pub fn set_system_output_override(&mut self, enabled: bool) -> Result<(), ShieldError<E>> {
        let mut buff = [0u8; registers::CONTROL.length];
        self.i2c.read(&registers::CONTROL.address, &mut buff)?;
        buff = if enabled {
            [buff[0] | 0b0000_1000]
        } else {
            [buff[0] & 0b0000_0111]
        };
        Ok(())
    }

    pub fn output_control_arm(&mut self) -> Result<(), ShieldError<E>> {
        self.i2c
            .write(&registers::OUTPUT_CONTROL_ARM.address, &[0x01])?;
        Ok(())
    }

    pub fn is_output_control_armed(&mut self) -> Result<bool, ShieldError<E>> {
        let mut buff = [0; registers::OUTPUT_CONTROL_ARM.length];
        self.i2c
            .read(&registers::OUTPUT_CONTROL_ARM.address, &mut buff)?;
        Ok(buff[0] > 0)
    }

    pub fn set_output_control_length(&mut self, length: u8) -> Result<(), ShieldError<E>> {
        self.i2c
            .write(&registers::OUTPUT_CONTROL_LENGTH.address, &[length])?;
        Ok(())
    }

    pub fn set_output_control_data(&mut self, data: &[u8]) -> Result<(), ShieldError<E>> {
        self.i2c
            .write(&registers::OUTPUT_CONTROL_DATA.address, &data)?;
        Ok(())
    }

    pub fn incoming_control_arm(&mut self) -> Result<(), ShieldError<E>> {
        self.i2c
            .write(&registers::INCOMING_CONTROL_ARM.address, &[0x01])?;
        Ok(())
    }

    pub fn is_incoming_control_armed(&mut self) -> Result<bool, ShieldError<E>> {
        let mut buff = [0u8; registers::INCOMING_CONTROL_ARM.length];
        self.i2c
            .read(&registers::INCOMING_CONTROL_ARM.address, &mut buff)?;
        Ok(buff[0] > 0)
    }

    pub fn get_incoming_control_length(&mut self) -> Result<u8, ShieldError<E>> {
        let mut buff = [0u8; registers::INCOMING_CONTROL_LENGTH.length];
        self.i2c
            .read(&registers::INCOMING_CONTROL_LENGTH.address, &mut buff)?;
        Ok(buff[0])
    }

    pub fn get_incoming_control_data(&mut self) -> Result<Box<[u8]>, ShieldError<E>> {
        let len = self.get_incoming_control_length()?;
        let mut buff = create_buffer(len).map_err(|err| ShieldError::MemoryAllocationError(err))?;
        self.i2c
            .read(&registers::INCOMING_CONTROL_DATA.address, buff.as_mut())?;
        Ok(buff)
    }

    pub fn output_tally_arm(&mut self) -> Result<(), ShieldError<E>> {
        self.i2c
            .write(&registers::OUTPUT_TALLY_ARM.address, &[0x01])?;
        Ok(())
    }

    pub fn output_tally_armed(&mut self) -> Result<bool, ShieldError<E>> {
        let mut buff = [0u8; registers::OUTPUT_TALLY_ARM.length];
        self.i2c
            .read(&registers::OUTPUT_TALLY_ARM.address, &mut buff)?;
        Ok(buff[0] > 0)
    }

    pub fn set_output_tally_length(&mut self, length: u8) -> Result<(), ShieldError<E>> {
        self.i2c
            .write(&registers::OUTPUT_TALLY_LENGTH.address, &[length])?;
        Ok(())
    }

    pub fn set_output_tally_data(&mut self, data: &[u8]) -> Result<(), ShieldError<E>> {
        self.i2c
            .write(&registers::OUTPUT_TALLY_DATA.address, &data)?;
        Ok(())
    }

    pub fn incoming_tally_arm(&mut self) -> Result<(), ShieldError<E>> {
        self.i2c
            .write(&registers::INCOMING_TALLY_ARM.address, &[0x01])?;
        Ok(())
    }

    pub fn incoming_tally_armed(&mut self) -> Result<bool, ShieldError<E>> {
        let mut buff = [0u8; registers::INCOMING_TALLY_ARM.length];
        self.i2c
            .read(&registers::INCOMING_TALLY_ARM.address, &mut buff)?;
        Ok(buff[0] > 0)
    }

    pub fn get_incoming_tally_length(&mut self) -> Result<u8, ShieldError<E>> {
        let mut buff = [0u8; registers::INCOMING_TALLY_LENGTH.length];
        self.i2c
            .read(&registers::INCOMING_TALLY_LENGTH.address, &mut buff)?;
        Ok(buff[0])
    }

    pub fn get_incoming_tally_data(&mut self) -> Result<Box<[u8]>, ShieldError<E>> {
        let len = self.get_incoming_tally_length()?;
        let mut buff = create_buffer(len).map_err(|err| ShieldError::MemoryAllocationError(err))?;
        self.i2c
            .read(&registers::INCOMING_TALLY_DATA.address, &mut buff)?;
        Ok(buff)
    }
}

use std::alloc;
use std::slice;
fn create_buffer(size: u8) -> Result<Box<[u8]>, alloc::LayoutError> {
    let layout = alloc::Layout::array::<u8>(size.into())?;
    unsafe {
        let prt = alloc::alloc_zeroed(layout);
        if prt.is_null() {
            alloc::handle_alloc_error(layout);
        }
        let slice = slice::from_raw_parts_mut(prt, size.into());
        Ok(Box::from_raw(slice))
    }
}
