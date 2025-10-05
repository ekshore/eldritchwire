# 🛡️ eldritch_shield

**Eldritch Shield** is a Rust library for interfacing with the **Blackmagic Design 3G-SDI Shield for Arduino** over I²C.  
It provides a safe, high-level API for controlling and querying the shield’s peripheral functions, while remaining agnostic of the underlying I²C transport implementation.

Designed to integrate cleanly with [`eldritchwire`](https://crates.io/crates/eldritchwire) or any existing I²C client.

---

## ✨ Features

- 🧩 **Transport-agnostic:** works with any I²C client — `linux-embedded-hal` or `rppal`, custom hardware clients, or mocks for testing.  
- 🧠 **Strongly typed API:** interact with camera control and SDI features using well-defined Rust types.  
- ⚙️ **Blackmagic-specific:** implements the protocol for the Blackmagic 3G-SDI Shield for Arduino.  
- 🧪 **Test-friendly:** easy to mock the transport layer for integration testing.  

---

## 🚀 Example

```rust
use eldritch_shield::{EldritchShield, traits::I2cTransport};
use linux_embedded_hal::I2cdev;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create your own I²C client or use one from embedded-hal
    let i2c = I2cdev::new("/dev/i2c-1")?;
    
    // The Blackmagic Shield’s I²C address (adjust as needed)
    let mut shield = EldritchShield::new(i2c, 0x3B);

    // Initialize the device
    shield.init()?;

    // Read status
    let status = shield.read_status()?;
    println!("Shield status: 0x{status:02X}");

    // Set a configuration mode
    shield.set_mode(0x01)?;

    Ok(())
}
````

---

## 🧱 Architecture

```
eldritch_shield/
├── src/
│   ├── lib.rs          # Crate entry point
│   ├── traits.rs       # I²C transport abstraction
│   ├── shield.rs       # High-level Blackmagic-specific logic
│   ├── registers.rs    # Register constants
│   └── errors.rs       # Unified error type
```

* **`I2cTransport` trait**: abstracts read/write operations, allowing pluggable backends.
* **`EldritchShield` struct**: encapsulates all shield functionality.
* **`PeripheralError`**: unifies I²C transport and device-specific errors.

---

## 🔌 Transport Abstraction

Eldritch Shield doesn’t implement its own I²C driver.
Instead, it defines a small trait you can implement for any backend:

```rust
pub trait I2cTransport {
    type Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error>;
    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error>;

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
```

Implement this for your own I²C layer, and you’re ready to go.

---

## 🧩 Integration with `eldritchwire`

`eldritchwire` handles **Blackmagic SDI camera control protocol** parsing and serialization.
`eldritch_shield` handles **the physical I²C link** to the 3G-SDI Shield.

Together, they form a full control pipeline:

```
Camera Control  ⇄  eldritchwire  ⇄  eldritch_shield  ⇄  I²C Transport  ⇄  SDI Shield  ⇄  Camera
```

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
eldritch_shield = "0.1"
```

---

## 🛠️ Development

### Build

```bash
cargo build
```

### Test

```bash
cargo test
```

### Lint & Format

```bash
cargo fmt
cargo clippy
```

---

## 🧰 Future Plans

* [ ] Implement support for non-default I2C addressing for rppal transport
* [ ] Add async support via `embedded-hal-async`
* [ ] Provide helper methods for camera control commands
* [ ] Integrate better with `eldritchwire` message types
* [ ] Support multi-shield configurations

---

## 📖 References

* [Blackmagic Design 3G-SDI Shield Developer Manual (PDF)](https://documents.blackmagicdesign.com/DeveloperManuals/BlackmagicCameraControl.pdf)
* [Blackmagic Camera Control Protocol](https://documents.blackmagicdesign.com/DeveloperManuals/BlackmagicCameraControl.pdf)
* [Eldritchwire crate (Camera Control Protocol)](https://crates.io/crates/eldritchwire)

---

**Eldritch Shield** — *a conduit between Rust and the arcane depths of Blackmagic hardware.*

