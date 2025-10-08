pub trait I2cTransport {
    type Error;

    fn write(&mut self, addr: &[u8; 2], bytes: &[u8]) -> Result<(), Self::Error>;
    fn read(&mut self, addr: &[u8; 2], buffer: &mut [u8]) -> Result<(), Self::Error>;
}
