# AI Agent Development Guide for Compass

This document provides instructions for AI coding assistants (Claude Code, Gemini, Cursor, etc.) working on the Compass privacy-focused browser.

**IMPORTANT**: Read this ENTIRE document before writing any code. Pay special attention to the "Common Mistakes to Avoid" section.

## Before Starting Any Work

**ALWAYS read `IMPLEMENTATION_PLAN.md` first** to understand:
- Current project status (what's complete, what's in progress)
- What phases are blocked and why
- The specific next tasks to work on
- Detailed step-by-step implementation plans

The implementation plan has checkboxes showing exactly where we left off.

## Project Overview

**Compass** is a privacy-focused web browser that:
- Uses **Rigging** for Servo embedding (WebView API)
- Adds **browser chrome** on top (toolbar, tabs, bookmarks - what Rigging stripped out)
- Integrates **Corsair** for Tor connectivity
- Implements **privacy protections** (fingerprint resistance, tracker blocking)

### How Compass Relates to Rigging and Harbor

```
┌─────────────────────────────────────────────────────────────────┐
│                    APPLICATIONS                                  │
├─────────────────────────────┬───────────────────────────────────┤
│         HARBOR              │           COMPASS                  │
│  - No browser chrome        │  - Full browser chrome ◄── THIS   │
│  - Backend management       │  - Tor integration                 │
│  - UdsConnector only        │  - TorConnector + TcpConnector     │
│  - Electron alternative     │  - Privacy browser                 │
└─────────────────────────────┴───────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    RIGGING                                       │
│            (forked from servoshell core)                         │
│  - WebView API                                                   │
│  - Window management (winit/surfman)                             │
│  - Pluggable Connector trait                                     │
│  - NO browser chrome (Compass adds this)                         │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    SERVO (upstream)                              │
│  - WebRender, Stylo, Layout, Script, etc.                       │
└─────────────────────────────────────────────────────────────────┘
```

**Key insight**: Rigging is a fork of servoshell with browser chrome stripped out. Compass adds its own browser chrome back on top.

## What Compass IS vs IS NOT

| Compass IS | Compass IS NOT |
|------------|----------------|
| A privacy browser | A general-purpose browser |
| Using Rigging for embedding | Embedding Servo directly |
| Adding browser chrome to Rigging | Modifying Rigging's core |
| Using TcpConnector + TorConnector | Using UdsConnector (that's Harbor) |
| A full browser with UI | A chromeless app framework |

## Repository Structure

```
compass/
├── Cargo.toml           # Package manifest
├── src/
│   ├── lib.rs           # Library exports
│   ├── app.rs           # CompassApp - main browser
│   ├── chrome/          # Browser UI (what Rigging doesn't have)
│   │   ├── mod.rs
│   │   ├── toolbar.rs   # URL bar, back/forward, etc.
│   │   ├── tabs.rs      # Tab management
│   │   └── bookmarks.rs # Bookmark management
│   ├── config.rs        # Browser configuration
│   ├── privacy.rs       # Privacy protection features
│   └── tor.rs           # Tor integration (via Corsair)
├── README.md
├── DESIGN.md
└── IMPLEMENTATION_PLAN.md
```

## Related Projects

| Project | Purpose | Relationship |
|---------|---------|--------------|
| [Rigging](https://github.com/marctjones/rigging) | Servo embedding | Library dependency |
| [Corsair](https://github.com/marctjones/corsair) | Tor daemon | Runtime dependency |
| [Harbor](https://github.com/marctjones/harbor) | Local apps | Sibling (also uses Rigging) |
| [Servo](https://github.com/servo/servo) | Browser engine | Via Rigging |

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

## Using Rigging's API

Compass uses Rigging's WebView API with TcpConnector (normal browsing) or TorConnector (Tor):

```rust
use rigging::{WebView, WebViewConfig, TcpConnector, TorConnector};

// Create a WebView for normal browsing
let webview = WebView::new(
    WebViewConfig {
        initial_url: "https://example.com".into(),
        width: 1280,
        height: 800,
        device_pixel_ratio: 1.0,
    },
    TcpConnector,  // or TorConnector for Tor mode
    &window,
)?;

// Handle events
for event in webview.tick() {
    match event {
        WebViewEvent::NavigationRequest { url, .. } => {
            // Update URL bar
        }
        WebViewEvent::TitleChanged(title) => {
            // Update window title / tab title
        }
        // ...
    }
}
```

## Common Mistakes to Avoid

**READ THIS SECTION CAREFULLY.** These are mistakes AI assistants keep making:

### 1. DO NOT Embed Servo Directly

**WRONG:**
```rust
use servo::Servo;
```

**RIGHT:**
```rust
use rigging::{WebView, WebViewConfig, TcpConnector};
```

**WHY:** Compass uses Rigging for Servo embedding. Rigging provides the stable API.

### 2. DO NOT Modify Rigging for Compass-Specific Features

**WRONG:**
- Adding Tor-specific code to Rigging
- Adding browser chrome to Rigging

**RIGHT:**
- Implement TorConnector that uses Corsair
- Add browser chrome in Compass's `chrome/` module

**WHY:** Rigging is shared with Harbor. Compass-specific code stays in Compass.

### 3. DO NOT Use UdsConnector

**WRONG:**
```rust
use rigging::UdsConnector;
let webview = WebView::new(config, UdsConnector, &window)?;
```

**RIGHT:**
```rust
use rigging::{TcpConnector, TorConnector};
// Normal browsing
let webview = WebView::new(config, TcpConnector, &window)?;
// Tor browsing
let webview = WebView::new(config, TorConnector::new(corsair_socket)?, &window)?;
```

**WHY:** UdsConnector is for Harbor (local apps). Compass is a web browser.

### 4. DO NOT Embed Arti Directly

**WRONG:**
```rust
use arti_client::TorClient;
```

**RIGHT:**
```rust
// Use Corsair daemon via TorConnector
use rigging::TorConnector;
let connector = TorConnector::new("/tmp/corsair.sock")?;
```

**WHY:** Arti conflicts with Stylo (both use different SpiderMonkey versions). Corsair runs Arti in a separate process.

### 5. DO NOT Forget Privacy Defaults

**WRONG:**
```rust
impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            fingerprint_resistance: false,  // Off by default
            // ...
        }
    }
}
```

**RIGHT:**
```rust
impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            fingerprint_resistance: true,   // On by default
            block_third_party_cookies: true,
            https_only: true,
            // ...
        }
    }
}
```

**WHY:** This is a privacy browser. Privacy features should be ON by default.

### 6. DO NOT Log Sensitive Data

**WRONG:**
```rust
log::info!("Navigating to: {}", url);
log::debug!("Cookie: {}", cookie_value);
```

**RIGHT:**
```rust
log::info!("Navigation started");
// Never log URLs, cookies, or form data
```

**WHY:** Privacy browser. Logs could leak user activity.

## Development Workflow: TDD and Commits

### Test-Driven Development

**Write tests BEFORE or ALONGSIDE implementation code.**

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_privacy_defaults_are_strict() {
        let config = PrivacyConfig::default();
        assert!(config.fingerprint_resistance);
        assert!(config.block_third_party_cookies);
        assert!(config.https_only);
    }
}
```

### Commit Frequently

**Commit after every successful test run.**

```bash
cargo test && git add -A && git commit -m "feat: add fingerprint resistance"
```

## Related Projects

- [Rigging](https://github.com/marctjones/rigging) - Servo embedding (forked from servoshell)
- [Corsair](https://github.com/marctjones/corsair) - Tor daemon
- [Harbor](https://github.com/marctjones/harbor) - Local app framework (sibling project)
- [Servo](https://github.com/servo/servo) - Browser engine (via Rigging)
- [Tor Browser](https://www.torproject.org/) - Reference for privacy features
