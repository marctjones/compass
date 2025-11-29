# Compass Implementation Plan

## Overview

Compass is a **privacy-focused web browser** with integrated Tor support. It is designed as an alternative to Tor Browser, built on the Servo rendering engine.

### Relationship with Rigging

Compass uses the **Rigging** library to embed Servo. Rigging provides:

1. A **stable embedding API** (`rigging::embed`) that insulates Compass from Servo internals
2. **Transport-aware networking** with support for TCP and Tor transports
3. **Multi-transport URL format** (`http::tcp://`, `http::tor://`)

```text
+-------------------------------------------------------------+
|                         Compass                              |
|  - Privacy engine (tracker blocking, fingerprint resistance) |
|  - Tor integration via Corsair daemon                        |
|  - Custom privacy-focused UI (not servoshell's egui)         |
+-----------------------------+-------------------------------+
                              | Uses stable API
                              v
+-------------------------------------------------------------+
|                    Rigging Library                           |
|  +-------------------------------------------------------+  |
|  |  embed/ - Stable Embedding API                        |  |
|  |  - BrowserBuilder, BrowserConfig, BrowserEvent        |  |
|  +-------------------------------------------------------+  |
|  +-------------------------------------------------------+  |
|  |  Transport Layer                                      |  |
|  |  - TransportUrl parsing (http::tcp://, http::tor://)  |  |
|  |  - TCP connector (standard internet)                  |  |
|  |  - Tor connector (via Corsair IPC)                    |  |
|  +-------------------------------------------------------+  |
+-----------------------------+-------------------------------+
                              | Internal implementation
                              v
+-------------------------------------------------------------+
|              Servo Engine (marctjones/servo fork)            |
|              with Rigging transport patches applied          |
+-------------------------------------------------------------+
```

### Sister Project: Harbor

Compass shares the Rigging embedding API with **Harbor** (local desktop app framework):

| Feature | Compass | Harbor |
|---------|---------|--------|
| **Purpose** | Privacy web browser | Desktop app framework |
| **Networking** | TCP + Tor (internet access) | UDS only (no internet) |
| **Transport URLs** | `http::tcp://`, `http::tor://` | `http::unix://` |
| **UI** | Privacy-focused (Tor toggle, tracker count) | Minimal/none (app-defined) |
| **External Links** | Route through Tor or TCP | Open in OS browser |

Both projects use the same Rigging API, demonstrating its flexibility for different use cases.

## Servo Integration Architecture

### What Parts of Servo Compass Uses

Compass uses Servo as its rendering engine through the Rigging library:

| Component | Purpose | How Used |
|-----------|---------|----------|
| **WebRender** | GPU-accelerated 2D rendering | Renders web content to OpenGL surfaces |
| **Stylo** | CSS engine (from Firefox) | Parses and applies CSS styles |
| **Layout** | Page layout engine | Computes element positions and sizes |
| **Script** | JavaScript engine (SpiderMonkey) | Executes page scripts |
| **net** | Networking (patched by Rigging) | HTTP over TCP and Tor transports |

### What Compass Does NOT Use

- **servoshell** - Servo's default shell (egui-based toolbar, tabs, URL bar)
- **minibrowser** - Servo's built-in browser UI

Compass **reimplements its own UI** because:
1. Privacy-focused toolbar (Tor toggle, tracker count, circuit info)
2. Custom navigation handling (route through Tor or block)
3. Privacy indicators (blocked trackers, connection security)
4. Settings UI for privacy/Tor configuration

### Transport URL Format

Rigging extends URL syntax to encode transport information:

```
scheme::transport//authority/path

Examples:
- http::tcp//example.com/        -> Standard TCP connection
- http::tor//example.com/        -> Route through Tor network
- http://example.onion/          -> Auto-detect Tor for .onion domains
- https::tor//check.torproject.org/ -> HTTPS over Tor
```

**Compass-specific behavior:**
- Default transport is TCP (standard internet browsing)
- When Tor mode is enabled, all requests use `http::tor://` transport
- `.onion` URLs automatically use Tor transport
- User can toggle between TCP and Tor modes via UI

### Third-Party Libraries Compass Uses Directly

| Library | Version | Purpose |
|---------|---------|---------|
| **winit** | 0.30+ | Window creation and event handling |
| **surfman** | 0.9+ | GPU surface management |
| **glow** | 0.16 | OpenGL wrapper |
| **euclid** | 0.22 | Geometric types |
| **raw-window-handle** | 0.6 | Window handle abstraction |
| **tokio** | 1.x | Async runtime |
| **hyper** | 1.x | HTTP client |

## Phase 1: Core Library (Current)

### 1.1 Configuration System
- [x] CompassConfig struct
- [x] PrivacyConfig with defaults
- [x] TorConfig for Corsair
- [x] WindowConfig
- [x] NetworkConfig
- [x] TOML parsing
- [ ] Config file loading from standard paths
- [ ] Environment variable support

### 1.2 Privacy Engine
- [x] PrivacyManager struct
- [x] Basic tracker domain blocklist
- [x] URL blocking logic
- [x] Cookie blocking logic
- [x] Privacy headers (DNT, GPC)
- [x] FingerprintResistance settings
- [ ] Full blocklist loading
- [ ] Blocklist updates

### 1.3 Transport Manager
- [x] TransportMode enum (TCP, Tor)
- [x] Corsair process management
- [x] Mode switching
- [ ] Connection retry logic
- [ ] New identity implementation
- [ ] Status reporting

## Phase 2: Servo Integration via Rigging - BLOCKED ON RIGGING

> **⚠️ BLOCKED**: This phase cannot proceed until Rigging completes its servoshell fork.
> See `/home/marc/rigging/IMPLEMENTATION_PLAN.md` for detailed Rigging tasks.
>
> **What Compass is waiting for from Rigging:**
> 1. Rigging must fork servoshell's core embedding code (~2,500 lines)
> 2. Rigging must strip browser chrome (toolbar, tabs, bookmarks, etc.)
> 3. Rigging must add the pluggable `Connector` trait
> 4. Rigging must implement `TcpConnector` for standard browsing
> 5. Rigging must implement `TorConnector` for Tor integration
> 6. Rigging must expose the `WebView` public API
>
> **Once Rigging is ready**, Compass can:
> - Use `rigging::WebView` with `TcpConnector` (normal browsing)
> - Switch to `TorConnector` when Tor mode is enabled
> - Add its own browser chrome (privacy toolbar, Tor toggle, etc.)
> - Implement tracker blocking via request interception
>
> **Note**: Compass adds its OWN browser chrome on top of Rigging. Rigging provides
> headless embedding, Compass provides the privacy-focused UI.

### 2.1 Rigging Embedding API
- [ ] Integrate `rigging::embed::BrowserBuilder`
- [ ] Configure `BrowserConfig` with Compass settings
- [ ] Handle `BrowserEvent` for navigation, load progress, errors
- [ ] Implement event callback for privacy interception

### 2.2 Network Layer (Rigging Transport)
- [ ] Configure default transport (TCP or Tor based on mode)
- [ ] Request interception for tracker blocking
- [ ] Privacy header injection via `BrowserConfig::user_agent`
- [ ] HTTPS upgrade logic
- [ ] `.onion` URL detection and Tor routing

### 2.3 Tor Integration (Corsair IPC)
- [ ] Start Corsair daemon on Tor mode enable
- [ ] Binary IPC protocol for Tor connections
- [ ] Circuit management (new identity)
- [ ] Bootstrap progress monitoring

### 2.4 Rendering Hooks
- [ ] Canvas fingerprint protection
- [ ] WebGL fingerprint protection
- [ ] Font enumeration limiting
- [ ] Screen resolution spoofing

### 2.5 Window Integration
- [ ] winit window creation with privacy-focused chrome
- [ ] surfman GPU surface setup
- [ ] WebRender rendering context
- [ ] Event loop integration
- [ ] Keyboard/mouse event forwarding
- [ ] Resize handling

## Phase 3: User Interface (Custom, Not servoshell)

### 3.1 Privacy-Focused Toolbar
- [ ] Navigation buttons (back, forward, reload)
- [ ] URL bar with security indicator
- [ ] **Tor toggle button** (switch TCP/Tor mode)
- [ ] **Blocked tracker count** badge
- [ ] Menu button

### 3.2 Settings Page
- [ ] Privacy settings UI
- [ ] Tor settings UI (bridges, bootstrap)
- [ ] Network settings UI
- [ ] About page

### 3.3 Privacy Indicators
- [ ] Connection security indicator (HTTPS, HTTP, .onion)
- [ ] **Tor circuit indicator** (entry -> relay -> exit)
- [ ] Blocked tracker count with expandable list
- [ ] Fingerprinting protection status

## Phase 4: Advanced Privacy

### 4.1 Enhanced Fingerprinting Resistance
- [ ] Canvas noise injection
- [ ] WebGL parameter normalization
- [ ] AudioContext protection
- [ ] Battery API blocking
- [ ] Sensor API blocking

### 4.2 Cookie Management
- [ ] First-party isolation
- [ ] Container tabs concept
- [ ] Cookie viewer/editor
- [ ] Clear on exit

### 4.3 History/Storage
- [ ] Private browsing mode
- [ ] History clearing
- [ ] Cache management
- [ ] Storage quotas

## Phase 5: Tor Features

### 5.1 Circuit Management
- [ ] New identity button (request via Corsair IPC)
- [ ] Circuit display (entry, relay, exit nodes)
- [ ] Exit node selection (future)

### 5.2 Onion Services
- [ ] .onion URL handling (auto Tor transport)
- [ ] Onion-Location header support
- [ ] Onion icons in URL bar

### 5.3 Bridge Support
- [ ] Bridge configuration UI
- [ ] Built-in bridge list
- [ ] obfs4 support

## Phase 6: Polish

### 6.1 Performance
- [ ] Startup optimization
- [ ] Memory profiling
- [ ] Network performance
- [ ] Rendering performance

### 6.2 Error Handling
- [ ] User-friendly error messages
- [ ] Connection error pages
- [ ] Tor bootstrap progress UI

### 6.3 Accessibility
- [ ] Keyboard navigation
- [ ] Screen reader support
- [ ] High contrast mode

## Milestones

### v0.1.0 - Core Library [CURRENT]
- [x] Configuration system
- [x] Privacy manager
- [x] Transport manager
- [x] Basic Corsair integration

### v0.2.0 - Rigging Integration
- [ ] Rigging embedding API integration (`BrowserBuilder`, `BrowserConfig`)
- [ ] Window rendering with WebRender
- [ ] TCP + Tor networking via Rigging transports
- [ ] Basic navigation

### v0.3.0 - Privacy Features
- [ ] Fingerprinting resistance
- [ ] Full tracker blocking
- [ ] Cookie isolation
- [ ] Privacy indicators in UI

### v0.4.0 - Tor Integration
- [ ] Full Corsair integration
- [ ] New identity via IPC
- [ ] Tor status UI
- [ ] Circuit display

### v0.5.0 - User Interface
- [ ] Complete privacy-focused toolbar
- [ ] Settings page
- [ ] Blocked tracker list

### v1.0.0 - Stable Release
- [ ] All core features
- [ ] Performance optimization
- [ ] Documentation complete

## Dependencies

### Current Dependencies (Phase 1)

| Crate | Version | Purpose |
|-------|---------|---------|
| serde | 1.x | Serialization |
| toml | 0.8.x | Config parsing |
| tokio | 1.x | Async runtime |
| log | 0.4.x | Logging |
| env_logger | 0.11.x | Log output |
| url | 2.x | URL parsing |
| thiserror | 1.x | Error handling |
| clap | 4.x | CLI parsing |
| dirs | 5.x | Config directories |
| rigging | path | Servo embedding API |

### Future Dependencies (Phase 2 - Servo Integration)

| Crate | Version | Purpose |
|-------|---------|---------|
| winit | 0.30+ | Window creation, events |
| surfman | 0.9+ | GPU surface management |
| glow | 0.16 | OpenGL wrapper |
| euclid | 0.22 | Geometric types |
| raw-window-handle | 0.6 | Window handle abstraction |
| hyper | 1.x | HTTP/1.1 client |
| bincode | 2.x | Corsair IPC serialization |

## Technical Debt

1. **Blocklist Format**: Need standardized format
2. **Test Coverage**: Integration tests needed
3. **Servo Updates**: Track upstream changes via Rigging

## Open Questions

1. How to handle mixed content (HTTP on HTTPS)?
2. Should we implement bookmark sync?
3. What extensions (if any) to support?
4. How to handle WebRTC (leak risk)?

## Testing Strategy

### Unit Tests
- Config parsing
- Privacy rule matching
- URL blocking
- Transport URL parsing

### Integration Tests
- Corsair communication
- Transport mode switching
- Privacy header injection
- Rigging embedding API

### Manual Tests
- Fingerprinting test sites (amiunique.org, browserleaks.com)
- Tor connectivity verification (check.torproject.org)
- UI responsiveness
- Memory usage under load

## Contributing

See AGENTS.md for AI assistant guidelines and coding standards.

## Upstream Tracking

Compass builds on Servo via Rigging. Key areas to track:

1. **Rigging** - Embedding API changes, transport updates
2. **servo/servo** main branch (via Rigging patches)
3. Network stack changes
4. Security fixes
5. WebRender updates

Rigging handles the complexity of tracking Servo changes. Compass should:
- Update Rigging dependency when new versions are released
- Test privacy features after Rigging updates
- Report any embedding API issues to Rigging project
