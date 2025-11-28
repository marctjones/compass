# AI Agent Development Guide for Compass

This document provides instructions for AI coding assistants (Claude Code, Gemini, Cursor, etc.) working on the Compass privacy-focused browser.

## Project Overview

**Compass** is a privacy-focused web browser built on the Servo engine with integrated Tor support. It combines:
- Servo browser engine for rendering
- Corsair daemon for Tor connectivity
- Rigging library for transport abstraction
- Built-in privacy protections

## Repository Structure

```
compass/
├── Cargo.toml           # Package manifest
├── src/
│   ├── lib.rs           # Library exports and architecture docs
│   ├── config.rs        # Browser configuration
│   ├── privacy.rs       # Privacy protection features
│   └── transport.rs     # Transport management (Corsair integration)
├── README.md
├── DESIGN.md
└── IMPLEMENTATION_PLAN.md
```

## Related Repositories

| Repo | Purpose | Relationship |
|------|---------|--------------|
| [servo](https://github.com/marctjones/servo) | Browser engine | Fork, upstream tracking |
| [corsair](https://github.com/marctjones/corsair) | Tor daemon | Runtime dependency |
| [rigging](https://github.com/marctjones/rigging) | Transport layer | Library dependency |
| [harbor](https://github.com/marctjones/harbor) | Local apps | Sibling project |

## Coding Standards

### Rust Guidelines
- **Edition**: Rust 2021
- **Async Runtime**: Tokio
- **Error Handling**: `thiserror` for library, `anyhow` for CLI
- **Configuration**: `serde` with TOML
- **Logging**: `log` crate with `env_logger`

### Code Style
```rust
// Good: Privacy-first defaults
impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            fingerprint_resistance: true,  // On by default
            block_third_party_cookies: true,
            https_only: true,
            do_not_track: true,
        }
    }
}

// Good: Clear transport mode handling
pub enum TransportMode {
    Direct,  // Standard TCP
    Tor,     // Via Corsair daemon
}

// Good: Feature gating
#[cfg(feature = "tor")]
pub fn enable_tor(&mut self) -> Result<(), TransportError> {
    self.ensure_corsair_running()?;
    self.mode = TransportMode::Tor;
    Ok(())
}
```

### Privacy Patterns
```rust
// Good: Fingerprint spoofing
pub struct FingerprintResistance {
    pub spoof_canvas: bool,
    pub spoof_webgl: bool,
    pub normalize_user_agent: bool,
    pub screen_resolution: Option<(u32, u32)>,
}

// Good: URL blocking
pub fn should_block_url(&self, url: &str) -> bool {
    if !self.config.block_trackers {
        return false;
    }
    self.blocked_domains.iter().any(|d| url.contains(d))
}
```

## Key Concepts

### Privacy by Default

Compass enables privacy protections by default:
- Fingerprint resistance (canvas, WebGL, audio spoofing)
- Third-party cookie blocking
- Tracker blocking
- HTTPS-only mode
- Do Not Track / Global Privacy Control headers

### Tor Integration

Compass uses Corsair daemon (not direct Arti) due to Arti/Stylo conflicts:

```
┌─────────────────┐     IPC      ┌─────────────────┐
│     Compass     │◄────────────►│     Corsair     │
│  (with Servo)   │  Unix Socket │   (with Arti)   │
└─────────────────┘              └─────────────────┘
```

### Transport Modes

1. **Direct**: Standard TCP connections
2. **Tor**: All traffic routed through Tor via Corsair

Users can switch modes at runtime via UI toggle.

## Development Tasks

### Adding a Privacy Feature

1. Add configuration option to `PrivacyConfig` in `config.rs`
2. Implement protection logic in `privacy.rs`
3. Wire into browser at appropriate hook point
4. Add UI control (in Servo shell)
5. Write tests
6. Update documentation

### Integrating with Servo

Compass extends Servo's functionality. Key integration points:
- Network stack (transport selection)
- Cookie storage (isolation)
- Rendering (fingerprint spoofing)
- User agent (normalization)

### Modifying Transport Behavior

1. Update `TransportManager` in `transport.rs`
2. Coordinate with Rigging library if needed
3. Test with both Direct and Tor modes
4. Handle mode switching gracefully

## Common Commands

```bash
# Build
cargo build --release

# Build without Tor
cargo build --release --no-default-features

# Run tests
cargo test

# Run with Tor enabled
compass --tor

# Run with custom config
compass --config ~/.config/compass/config.toml
```

## Configuration

### Default Config Location
```
~/.config/compass/config.toml
```

### Configuration File
```toml
[privacy]
fingerprint_resistance = true
block_third_party_cookies = true
clear_cookies_on_exit = false
clear_history_on_exit = false
do_not_track = true
block_trackers = true
https_only = true

[tor]
enabled = false
corsair_socket = "/tmp/corsair.sock"
auto_start_corsair = true
new_identity_per_tab = false

[window]
width = 1280
height = 800
maximized = false
show_toolbar = true

[network]
default_transport = "tcp"
timeout = 30
dns_over_https = true
doh_server = "https://cloudflare-dns.com/dns-query"
```

## Privacy Features Reference

### Fingerprint Resistance

| Feature | Effect |
|---------|--------|
| Canvas spoofing | Returns noise for canvas reads |
| WebGL spoofing | Normalizes WebGL fingerprint |
| Audio spoofing | Adds noise to AudioContext |
| User agent | Normalized to common browser |
| Timezone | Reports UTC |
| Language | Reports en-US |
| Screen size | Reports common resolution |

### Tracking Protection

- Blocklist of known tracker domains
- Third-party cookie blocking
- Referrer trimming
- DNT and GPC headers

## Important Notes

1. **Arti/Stylo Conflict**: Never try to embed Arti in Servo directly
2. **Privacy Defaults**: Always default to more privacy
3. **User Choice**: Allow disabling for compatibility
4. **Performance**: Privacy features should not significantly impact speed
5. **Testing**: Test both with and without Tor

## Error Handling

```rust
#[derive(Debug, Error)]
pub enum CompassError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Transport error: {0}")]
    Transport(#[from] TransportError),

    #[error("Privacy error: {0}")]
    Privacy(String),

    #[error("Servo error: {0}")]
    Servo(String),
}
```

## Integration Testing

### With Corsair
```bash
# Start Corsair daemon
corsair --socket /tmp/corsair.sock &

# Run Compass with Tor
compass --tor https://check.torproject.org
```

### Privacy Testing
```bash
# Test fingerprinting resistance
compass https://browserleaks.com/canvas

# Test tracker blocking
compass https://example.com  # Monitor blocked requests
```

## Security Considerations

1. **Process Isolation**: Corsair runs as separate process
2. **Minimal Permissions**: Browser runs with user privileges
3. **No Logging Sensitive Data**: URLs, cookies not logged
4. **Secure Defaults**: All privacy features on by default

## Related Projects

- [Servo](https://github.com/servo/servo) - Browser engine
- [Corsair](https://github.com/marctjones/corsair) - Tor daemon
- [Rigging](https://github.com/marctjones/rigging) - Transport library
- [Tor Browser](https://www.torproject.org/) - Reference implementation
