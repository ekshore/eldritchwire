# eldritchwire

> Decode the arcane Blackmagic camera control protocol over SDI.

`eldritchwire` is a Rust crate for parsing, building, and interpreting packets from Blackmagic Design’s camera control protocol over SDI. It gives you strongly typed access to protocol commands, reliable packet parsing, and a foundation for building video and broadcast automation tools.

---

## ✨ Features

- 🧙‍♂️ Strongly typed command structures for the full protocol
- 🧵 Zero-copy packet parsing from raw SDI buffers
- ⚙️ Helpers for building and sending command packets
- 🧪 Designed for easy integration and testing in Rust applications

---

## 🔧 Example

```rust
use eldritchwire::parse_packet;

let raw_data: &[u8] = /* your SDI data buffer here */;
let commands = parse_packet(raw_data)?;

for cmd in commands {
    println!("{:?}", cmd);
}
```

---
## Official Docs

Blackmagic Design's official protocol [docs](https://documents.blackmagicdesign.com/DeveloperManuals/BlackmagicCameraControl.pdf)
