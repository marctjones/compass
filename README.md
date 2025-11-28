# Compass 🧭

Privacy-focused web browser built on the Servo engine with integrated Tor support.

## Overview

Compass is a privacy-first web browser that combines:
- **Servo Engine**: Modern, memory-safe browser engine written in Rust
- **Tor Integration**: Optional traffic routing through Tor via Corsair daemon
- **Privacy by Default**: Fingerprint resistance, tracker blocking, cookie isolation
- **Transport Flexibility**: Multiple transport backends via Rigging library

## Features

### Privacy Protection
- **Fingerprint Resistance**: Spoofs canvas, WebGL, audio context, and other fingerprinting vectors
- **Tracker Blocking**: Blocks known tracking domains
- **Third-Party Cookie Blocking**: Prevents cross-site tracking
- **HTTPS-Only Mode**: Automatically upgrades insecure connections
- **Do Not Track / GPC**: Sends privacy preference signals

### Tor Integration
- **One-Click Tor**: Enable/disable Tor routing instantly
- **Corsair Daemon**: Isolated Tor process with binary IPC protocol
- **New Identity**: Request fresh Tor circuits on demand
- **Per-Tab Isolation**: Optional new identity per tab

### Network
- **Transport Abstraction**: Uses Rigging for flexible transport backends
- **DNS over HTTPS**: Encrypted DNS queries
- **Connection Pooling**: Efficient resource usage

## Quick Start

### Installation

```bash
# Clone and build
git clone https://github.com/marctjones/compass
cd compass
cargo build --release

# Install Corsair for Tor support (optional)
cargo install --git https://github.com/marctjones/corsair
```

### Configuration

Create `~/.config/compass/config.toml`:

```toml
[privacy]
fingerprint_resistance = true
block_third_party_cookies = true
https_only = true

[tor]
enabled = false  # Enable for Tor by default
corsair_socket = "/tmp/corsair.sock"
auto_start_corsair = true

[window]
width = 1280
height = 800

[network]
dns_over_https = true
doh_server = "https://cloudflare-dns.com/dns-query"
```

### Running

```bash
# Standard browsing
compass

# With Tor enabled
compass --tor

# Open specific URL
compass https://example.com
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Compass Browser                          │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────────┐         ┌──────────────────────────┐  │
│  │    Browser UI    │         │      Privacy Engine      │  │
│  │  (Servo-based)   │         │  - Tor via Corsair       │  │
│  │                  │         │  - Transport via Rigging │  │
│  │  - Tab management│         │  - Fingerprint resist    │  │
│  │  - Navigation    │         │  - Cookie isolation      │  │
│  │  - Bookmarks     │         │                          │  │
│  └──────────────────┘         └──────────────────────────┘  │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │                   Rigging Transport                   │   │
│  │    TCP | Unix Socket | Named Pipe | Tor (Corsair)    │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Related Projects

- [Servo](https://github.com/servo/servo) - The browser engine
- [Corsair](https://github.com/marctjones/corsair) - Tor daemon with binary IPC
- [Rigging](https://github.com/marctjones/rigging) - Transport layer library
- [Harbor](https://github.com/marctjones/harbor) - Local app framework

## Building from Source

### Prerequisites

- Rust 1.75+
- Servo dependencies (see [Servo build instructions](https://servo.org/building/))

### Build

```bash
# Debug build
cargo build

# Release build
cargo build --release

# With Tor support
cargo build --release --features tor
```

## Privacy Settings Reference

| Setting | Default | Description |
|---------|---------|-------------|
| `fingerprint_resistance` | `true` | Spoof browser fingerprint |
| `block_third_party_cookies` | `true` | Block cross-site cookies |
| `clear_cookies_on_exit` | `false` | Delete cookies when closing |
| `clear_history_on_exit` | `false` | Delete history when closing |
| `do_not_track` | `true` | Send DNT header |
| `block_trackers` | `true` | Block known tracking domains |
| `https_only` | `true` | Force HTTPS connections |

## Tor Settings Reference

| Setting | Default | Description |
|---------|---------|-------------|
| `enabled` | `false` | Enable Tor by default |
| `corsair_socket` | `/tmp/corsair.sock` | Corsair daemon socket |
| `auto_start_corsair` | `true` | Start Corsair automatically |
| `new_identity_per_tab` | `false` | New circuit per tab |

## License

Mozilla Public License 2.0 (MPL-2.0)

## Security

For security issues, please see [SECURITY.md](SECURITY.md).
