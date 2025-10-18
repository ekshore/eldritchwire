use std::fs;
use std::io;
use std::path::Path;

use crate::traits::I2cTransport;

/// A mock I2C transport for testing that simulates device responses
/// based on 16-bit (little-endian) register addresses.
///
/// - `0x3000` → returns `[0x01]`
/// - `0x3001` → returns `[len as u8]` for the next buffer
/// - `0x3100` → returns the next buffer of data (consumes it)
pub struct MockI2c {
    /// Buffers of test data loaded from file.
    data_buffers: Vec<Vec<u8>>,
    /// Index of the next buffer to return.
    current_index: usize,
}

impl MockI2c {
    /// Creates a new mock I²C transport from a test data file.
    ///
    /// Each non-empty line in the file represents a buffer of hex bytes:
    ///
    /// ```text
    /// AA BB CC DD
    /// 11 22 33 44 55
    /// DE AD BE EF
    /// ```
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let data_buffers = contents
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|byte_str| u8::from_str_radix(byte_str, 16).ok())
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<_>>();

        Ok(Self {
            data_buffers,
            current_index: 0,
        })
    }

    fn next_buffer(&mut self) -> Option<Vec<u8>> {
        if self.current_index < self.data_buffers.len() {
            let buf = self.data_buffers[self.current_index].clone();
            self.current_index += 1;
            Some(buf)
        } else {
            None
        }
    }

    fn has_next_buffer(&self) -> bool {
        self.current_index < self.data_buffers.len()
    }

    fn read_from_register(&mut self, reg_le: &[u8; 2], buffer: &mut [u8]) -> io::Result<()> {
        let reg = u16::from_le_bytes(*reg_le);
        println!("Mock reading from register {reg:02x}");

        match reg {
            0x3000 => {
                buffer.fill(0);
                buffer[0] = if self.has_next_buffer() {
                    0x00 // arm flag is cleared
                } else {
                    0x01
                };
                Ok(())
            }
            0x3001 => {
                buffer.fill(0);
                if let Some(next) = self.data_buffers.get(self.current_index) {
                    buffer[0] = next.len() as u8;
                } else {
                    buffer[0] = 0;
                }
                Ok(())
            }
            0x3100 => {
                buffer.fill(0);
                if let Some(next) = self.next_buffer() {
                    for (b, &val) in buffer.iter_mut().zip(next.iter()) {
                        *b = val;
                    }
                }
                Ok(())
            }
            _ => {
                buffer.fill(0);
                Ok(())
            }
        }
    }
}

impl I2cTransport for MockI2c {
    type Error = io::Error;

    fn write(&mut self, _addr: &[u8; 2], _bytes: &[u8]) -> Result<(), Self::Error> {
        // This mock ignores writes for now
        Ok(())
    }

    fn read(&mut self, addr: &[u8; 2], buffer: &mut [u8]) -> Result<(), Self::Error> {
        // For direct read calls, we’ll just clear the buffer
        self.read_from_register(addr, buffer)
    }
}

#[cfg(test)]
mod test {
    use super::MockI2c;
    use crate::traits::I2cTransport;
    use std::path::PathBuf;

    #[test]
    fn test_mock_i2c_from_file() {
        // Path to your test data file (adjust if needed)
        let path = PathBuf::from("src/transport_adapters/test_data.txt");

        let mut mock = MockI2c::from_file(&path).expect("Failed to load test data file");

        let mut buf = [0u8; 64];

        // The test data file should have 6 buffers (lines)
        let expected_buffers = vec![
            vec![0x04, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00],
            vec![
                0xFF, 0x05, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
            ],
            vec![
                0x04, 0x08, 0x00, 0x00, 0x01, 0x05, 0x03, 0x00, 0x10, 0x27, 0x00, 0x00,
            ],
            vec![
                0x04, 0x06, 0x00, 0x00, 0x04, 0x02, 0x80, 0x01, 0x33, 0x01, 0x00, 0x00,
            ],
            vec![
                0xFF, 0x09, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x18, 0x01, 0x03, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ],
            vec![
                0x04, 0x0C, 0x00, 0x00, 0x08, 0x01, 0x80, 0x01, 0x00, 0x00, 0x9A, 0xFD, 0x9A, 0xFD,
                0x00, 0x00,
            ],
        ];

        // For each buffer, simulate 0x3000 (ready), 0x3001 (length), and 0x3100 (data)
        for expected in expected_buffers.iter() {
            // 0x3000 — ready flag
            mock.read(&[0x00, 0x30], &mut buf)
                .expect("read 0x3000 failed");
            assert_eq!(buf[0], 0x00, "expected ready flag 0x01");

            // 0x3001 — next buffer length
            mock.read(&[0x01, 0x30], &mut buf)
                .expect("read 0x3001 failed");
            assert_eq!(buf[0], expected.len() as u8, "expected next buffer length");

            // 0x3100 — next buffer contents
            mock.read(&[0x00, 0x31], &mut buf)
                .expect("read 0x3100 failed");

            let returned = &buf[..expected.len()];
            assert_eq!(
                returned,
                expected,
                "buffer mismatch for sequence {}",
                mock.current_index - 1
            );
        }

        // After all data consumed, 0x3001 should report 0 length
        mock.read(&[0x01, 0x30], &mut buf)
            .expect("read 0x3001 failed after exhaustion");
        assert_eq!(buf[0], 0, "expected 0 length after buffers exhausted");
    }
}
