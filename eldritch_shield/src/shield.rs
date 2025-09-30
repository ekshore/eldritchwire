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
        let mut buff: [u8; registers::IDENTITY.length] = [0; registers::IDENTITY.length];
        self.i2c.read(&registers::IDENTITY.address, &mut buff)?;
        Ok(String::from_utf8(buff.to_vec()).map_err(|_| ShieldError::InvalidResponse)?)
    }

    pub fn get_hardware_version(&mut self) -> Result<(u8, u8), ShieldError<E>> {
        let mut buff: [u8; registers::HARDWARE_VERSION.length] =
            [0; registers::HARDWARE_VERSION.length];
        self.i2c.read(&registers::IDENTITY.address, &mut buff)?;
        Ok((buff[0], buff[1]))
    }

    pub fn get_firmware_version(&mut self) -> Result<(u8, u8), ShieldError<E>> {
        let mut buff: [u8; registers::FIRMWARE_VERSION.length] =
            [0; registers::FIRMWARE_VERSION.length];
        self.i2c
            .read(&registers::FIRMWARE_VERSION.address, &mut buff)?;
        Ok((buff[0], buff[1]))
    }

    pub fn get_system_control_register(&mut self) -> Result<u8, ShieldError<E>> {
        let mut buff: [u8; registers::CONTROL.length] = [0; registers::CONTROL.length];
        self.i2c.read(&registers::CONTROL.address, &mut buff)?;
        Ok(buff[0])
    }

    pub fn set_system_control_register(&mut self, byte: u8) -> Result<(), ShieldError<E>> {
        self.i2c.write(&registers::CONTROL.address, &[byte])?;
        Ok(())
    }

    pub fn set_system_control_override(&mut self, enabled: bool) -> Result<(), ShieldError<E>> {
        let mut buff: [u8; registers::CONTROL.length] = [0; registers::CONTROL.length];
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
        let mut buff: [u8; registers::CONTROL.length] = [0; registers::CONTROL.length];
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
        let mut buff: [u8; registers::CONTROL.length] = [0; registers::CONTROL.length];
        self.i2c.read(&registers::CONTROL.address, &mut buff)?;
        buff = [buff[0] | 0b0000_0100];
        self.i2c.write(&registers::CONTROL.address, &buff)?;
        Ok(())
    }

    pub fn set_system_output_override(&mut self, enabled: bool) -> Result<(), ShieldError<E>> {
        let mut buff: [u8; registers::CONTROL.length] = [0; registers::CONTROL.length];
        self.i2c.read(&registers::CONTROL.address, &mut buff)?;
        buff = if enabled {
            [buff[0] | 0b0000_1000]
        } else {
            [buff[0] & 0b0000_0111]
        };
        Ok(())
    }
    }
}
