use crate::traits::I2cTransport;
use rppal::i2c::I2c;

impl I2cTransport for I2c {
    type Error = crate::errors::ShieldError<rppal::i2c::Error>;

    fn write(&mut self, addr: &[u8; 2], bytes: &[u8]) -> Result<(), Self::Error> {
        let mut buff: Vec<u8> = Vec::from(addr);
        buff.extend_from_slice(bytes);
        self.write(buff.as_slice())
            .map_err(|err| Self::Error::Transport(err))?;
        Ok(())
    }

    fn read(&mut self, addr: &[u8; 2], buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.write(addr)
            .map_err(|err| Self::Error::Transport(err))?;
        self.read(buffer)
            .map_err(|err| Self::Error::Transport(err))?;
        Ok(())
    }
}
