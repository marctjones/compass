# Compass Design Document

## Overview

Compass is a privacy-focused web browser built on the Servo engine with integrated Tor support and comprehensive privacy protections.

## Vision

Create a modern, privacy-respecting browser that:
- Provides Tor integration without sacrificing usability
- Implements strong fingerprinting resistance
- Offers a clean, simple user interface
- Maintains compatibility with modern web standards

## Goals

1. **Privacy First**: Strong defaults that protect user privacy
2. **Tor Integration**: Seamless Tor connectivity via Corsair
3. **Modern Engine**: Leverage Servo's performance and safety
4. **User Control**: Allow users to customize privacy/usability trade-offs
5. **Simplicity**: Clean interface, minimal complexity

## Non-Goals

1. Not a Chromium/Firefox replacement for general use
2. Not aiming for 100% website compatibility
3. Not implementing every Tor Browser feature initially

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Compass Browser                          │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │                    Browser Shell                      │   │
│  │  - Window management                                  │   │
│  │  - Tab management                                     │   │
│  │  - Navigation UI                                      │   │
│  │  - Settings UI                                        │   │
│  └──────────────────────────────────────────────────────┘   │
│                              │                               │
│  ┌───────────────────┐  ┌────┴────────────────────────────┐ │
│  │  Privacy Engine   │  │        Servo Engine             │ │
│  │                   │  │                                 │ │
│  │ - Fingerprint     │  │  - HTML/CSS rendering           │ │
│  │   resistance      │  │  - JavaScript execution         │ │
│  │ - Tracker         │  │  - DOM management               │ │
│  │   blocking        │  │  - Network requests             │ │
│  │ - Cookie          │  │                                 │ │
│  │   isolation       │  └─────────────────────────────────┘ │
│  └───────────────────┘                                      │
│                              │                               │
│  ┌───────────────────────────┴──────────────────────────┐   │
│  │              Transport Layer (Rigging)                │   │
│  │                                                       │   │
│  │    ┌─────────┐     ┌─────────┐     ┌─────────────┐   │   │
│  │    │   TCP   │     │   Tor   │     │   Future    │   │   │
│  │    │ Direct  │     │(Corsair)│     │  Transports │   │   │
│  │    └─────────┘     └─────────┘     └─────────────┘   │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Component Design

### Browser Shell

The main application that hosts Servo and provides UI:

```rust
pub struct CompassBrowser {
    config: CompassConfig,
    privacy: PrivacyManager,
    transport: TransportManager,
    // Servo integration points
}
```

### Privacy Engine

Manages all privacy-related features:

```rust
pub struct PrivacyManager {
    config: PrivacyConfig,
    blocked_domains: HashSet<String>,
}

impl PrivacyManager {
    pub fn should_block_url(&self, url: &str) -> bool;
    pub fn should_block_cookie(&self, domain: &str, third_party: bool) -> bool;
    pub fn get_privacy_headers(&self) -> Vec<(String, String)>;
    pub fn get_fingerprint_resistance(&self) -> FingerprintResistance;
}
```

### Transport Manager

Handles network transport selection:

```rust
pub struct TransportManager {
    mode: TransportMode,
    tor_config: TorConfig,
    corsair_process: Option<Child>,
}

impl TransportManager {
    pub fn set_mode(&mut self, mode: TransportMode) -> Result<()>;
    pub fn ensure_corsair_running(&mut self) -> Result<()>;
    pub fn new_identity(&self) -> Result<()>;
}
```

## Privacy Features

### Fingerprint Resistance

Compass implements multi-layer fingerprint protection:

| Layer | Technique | Effect |
|-------|-----------|--------|
| Canvas | Noise injection | Randomizes canvas fingerprint |
| WebGL | Parameter normalization | Common WebGL values |
| Audio | Noise addition | Prevents AudioContext fingerprinting |
| Fonts | Limit enumeration | Reduce font fingerprint surface |
| Screen | Fixed resolution | Report common 1920x1080 |
| Timezone | UTC only | Prevent timezone fingerprinting |
| Language | en-US | Standardized language |
| User Agent | Normalized | Common Firefox UA |

### Tracking Protection

1. **Domain Blocklist**: Known tracker domains blocked
2. **Cookie Isolation**: Third-party cookies blocked by default
3. **Referrer Policy**: Strict referrer trimming
4. **Headers**: DNT and GPC sent automatically

### HTTPS-Only Mode

- All HTTP requests upgraded to HTTPS
- User warning for sites without HTTPS
- Exception list for compatibility

## Tor Integration

### Why Corsair (Not Direct Arti)

Arti and Servo's Stylo have conflicting trait implementations causing compiler recursion overflow. Corsair solves this by process isolation.

### Connection Flow

```
User navigates to URL
         │
         ▼
┌─────────────────────┐
│  Transport Manager  │
│  (check mode)       │
└─────────┬───────────┘
          │
    ┌─────┴─────┐
    │           │
    ▼           ▼
┌───────┐   ┌───────────┐
│Direct │   │    Tor    │
│(TCP)  │   │ (Corsair) │
└───┬───┘   └─────┬─────┘
    │             │
    │             ▼
    │     ┌───────────────┐
    │     │ Send request  │
    │     │ to Corsair    │
    │     └───────┬───────┘
    │             │
    │             ▼
    │     ┌───────────────┐
    │     │ Corsair opens │
    │     │ Tor circuit   │
    │     └───────┬───────┘
    │             │
    └──────┬──────┘
           │
           ▼
    ┌─────────────┐
    │   Website   │
    └─────────────┘
```

### New Identity

When user requests new identity:
1. Send NewIdentity request to Corsair
2. Corsair clears Tor circuits
3. Clear browser state (cookies, etc.)
4. Notify user of completion

## Configuration System

### Config Sources (Priority Order)

1. Command-line arguments
2. Config file (~/.config/compass/config.toml)
3. Environment variables (COMPASS_*)
4. Compiled defaults

### Config Structure

```rust
pub struct CompassConfig {
    pub privacy: PrivacyConfig,
    pub tor: TorConfig,
    pub window: WindowConfig,
    pub network: NetworkConfig,
}
```

## User Interface

### Toolbar

```
┌────────────────────────────────────────────────────────────┐
│ ◀ ▶ ⟳  │ 🔒 https://example.com                    │ ☰ 🧅 │
└────────────────────────────────────────────────────────────┘
     │                    │                              │  │
     │                    │                              │  │
     │                    │                              │  └─ Tor toggle
     │                    │                              └──── Menu
     │                    └─────────────────────────────────── URL bar
     └──────────────────────────────────────────────────────── Nav buttons
```

### Privacy Indicator

- 🟢 Green: Strong privacy (Tor + all protections)
- 🟡 Yellow: Some protections disabled
- 🔴 Red: Tor off, protections disabled

## Security Model

### Threat Model

Compass protects against:
- Website tracking
- Browser fingerprinting
- Network surveillance (with Tor)
- Third-party cookie tracking

Compass does NOT protect against:
- Malware on user's device
- Physical device access
- Targeted attacks by well-resourced adversaries

### Process Isolation

```
┌─────────────┐   ┌─────────────┐   ┌─────────────┐
│   Compass   │   │   Corsair   │   │   Backend   │
│  (Browser)  │   │   (Tor)     │   │  (Harbor)   │
│             │   │             │   │             │
│ User: marc  │   │ User: marc  │   │ User: marc  │
│ No net caps │   │ Tor only    │   │ UDS only    │
└─────────────┘   └─────────────┘   └─────────────┘
```

## Future Extensions

1. **Per-Site Settings**: Override privacy settings per domain
2. **Private Tabs**: Temporary sessions with extra isolation
3. **Bookmark Sync**: Encrypted bookmark synchronization
4. **Extension Support**: Limited, privacy-vetted extensions
5. **Onion Services**: Browse .onion sites
6. **Bridge Support**: Tor bridge configuration

## Comparison with Tor Browser

| Feature | Compass | Tor Browser |
|---------|---------|-------------|
| Engine | Servo | Firefox |
| Tor | Corsair (Arti) | tor daemon |
| Fingerprinting | Basic | Comprehensive |
| Extensions | None yet | NoScript, etc. |
| Maturity | New | Established |
| Memory | Lower | Higher |
