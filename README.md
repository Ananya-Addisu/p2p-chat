# Secure P2P LAN Chat Application

[![Rust](https://img.shields.io/badge/Rust-1.65%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A secure, encrypted peer-to-peer chat application for local area networks (LAN) with automatic peer discovery.

## Features

### 🔒 Security
- **End-to-End Encryption** using X25519 key exchange
- **Perfect Forward Secrecy** - Unique session keys per conversation
- **Authenticated Encryption** with ChaCha20-Poly1305
- **Secure Key Derivation** using HKDF-SHA256

### 🌐 Networking
- Automatic peer discovery via UDP broadcasts
- Reliable message delivery over TCP
- Multi-peer communication support
- Async I/O using Tokio runtime

### 💻 Interface
- Simple terminal-based UI
- Colored status messages
- Real-time message display
- Connection status monitoring

## Prerequisites

- Rust 1.65+ (install via [rustup](https://rustup.rs/))
- Multiple machines on the same LAN
- Open firewall ports: 54545 (UDP) and 54546 (TCP)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/Ananya-Addisu/p2p-chat.git
cd p2p-chat
```

2. Build the application:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --release
```

## Usage

### Starting the Chat
```bash
# On all participating machines
./target/release/p2p-chat
```

### Sending Messages
1. Type your message and press Enter
2. Messages will appear on all peers' terminals
3. Messages are automatically encrypted before sending

### Commands
- `/exit` - Shut down the application
- `/peers` - List connected peers (TODO)
- `/help` - Show help menu (TODO)

## Technical Details

### 🔑 Cryptography
1. **Key Exchange**: X25519 elliptic curve Diffie-Hellman
2. **Encryption**: ChaCha20-Poly1305 AEAD
3. **Key Rotation**: New session keys per connection
4. **Randomness**: System-secure random number generation

### 📡 Network Protocol
1. **Discovery Phase** (UDP):
   - Broadcast `P2P_CHAT_DISCOVERY` every 5 seconds
   - Respond to discovery requests with public key

2. **Handshake Phase** (TCP):
   ```plaintext
   Client A                          Client B
     | ----- Public Key (32B) -------> |
     | <---- Public Key (32B) -------- |
     | ---- Encrypted Session Key ---> |
   ```

3. **Message Format**:
   ```rust
   struct Message {
       nonce: [u8; 12],
       ciphertext: Vec<u8>,
   }
   ```

## Security Considerations

1. **Trust First Use** (TOFU) - Trust peers on first connection
2. **No Certificate Authority** - Manual verification not implemented
3. **Message Authentication** - Built into ChaCha20-Poly1305
4. **Replay Protection** - Unique nonces for each message

> **Warning**  
> This is a educational project. Not audited for production use.

## Limitations

- LAN-only communication (no NAT traversal)
- No message persistence/history
- Basic terminal interface
- No mobile support
- Limited error recovery

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Support

For issues/questions:
- [Open GitHub Issue](https://github.com/Ananya-Addisu/p2p-chat/)
- Email: sidraq@codeday.org
