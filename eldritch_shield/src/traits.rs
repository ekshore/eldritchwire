pub trait I2cTransport {
    type Error;

    fn write(&mut self, addr: [u8;2], bytes: &[u8]) -> Result<(), Self::Error>;
    fn read(&mut self, addr: [u8;2], buffer: &mut [u8]) -> Result<(), Self::Error>;

    fn write_read(
        &mut self,
        addr: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.write(addr, bytes)?;
        self.read(addr, buffer)
    }
}

