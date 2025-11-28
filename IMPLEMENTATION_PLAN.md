# Compass Implementation Plan

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
- [x] TransportMode enum
- [x] Corsair process management
- [x] Mode switching
- [ ] Connection retry logic
- [ ] New identity implementation
- [ ] Status reporting

## Phase 2: Servo Integration

### 2.1 Network Layer
- [ ] Integrate Rigging transport
- [ ] Request interception for blocking
- [ ] Privacy header injection
- [ ] HTTPS upgrade logic

### 2.2 Rendering Hooks
- [ ] Canvas fingerprint protection
- [ ] WebGL fingerprint protection
- [ ] Font enumeration limiting
- [ ] Screen resolution spoofing

### 2.3 Browser Shell
- [ ] Basic window creation
- [ ] Navigation controls
- [ ] URL bar
- [ ] Tab management

## Phase 3: User Interface

### 3.1 Toolbar
- [ ] Navigation buttons
- [ ] URL bar with security indicator
- [ ] Tor toggle button
- [ ] Menu button

### 3.2 Settings Page
- [ ] Privacy settings UI
- [ ] Tor settings UI
- [ ] Network settings UI
- [ ] About page

### 3.3 Privacy Indicators
- [ ] Connection security indicator
- [ ] Tor status indicator
- [ ] Blocked tracker count

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
- [ ] New identity button
- [ ] Circuit display
- [ ] Exit node selection (future)

### 5.2 Onion Services
- [ ] .onion URL handling
- [ ] Onion-Location header support
- [ ] Onion icons

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
- [ ] Tor bootstrap progress

### 6.3 Accessibility
- [ ] Keyboard navigation
- [ ] Screen reader support
- [ ] High contrast mode

## Milestones

### v0.1.0 - Core Library
- Configuration system
- Privacy manager
- Transport manager
- Basic Corsair integration

### v0.2.0 - Basic Browser
- Servo integration
- Navigation
- Privacy blocking active

### v0.3.0 - Privacy Features
- Fingerprinting resistance
- Full tracker blocking
- Cookie isolation

### v0.4.0 - Tor Integration
- Full Corsair integration
- New identity
- Tor status UI

### v0.5.0 - User Interface
- Complete toolbar
- Settings page
- Privacy indicators

### v1.0.0 - Stable Release
- All core features
- Performance optimization
- Documentation complete

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| rigging | git | Transport layer |
| serde | 1.x | Serialization |
| toml | 0.8.x | Config parsing |
| tokio | 1.x | Async runtime |
| log | 0.4.x | Logging |
| env_logger | 0.11.x | Log output |
| url | 2.x | URL parsing |
| thiserror | 1.x | Error handling |
| clap | 4.x | CLI parsing |
| dirs | 5.x | Config directories |

## Technical Debt

1. **Blocklist Format**: Need standardized format
2. **Test Coverage**: Integration tests needed
3. **Servo Updates**: Track upstream changes

## Open Questions

1. How to handle mixed content (HTTP on HTTPS)?
2. Should we implement bookmark sync?
3. What extensions (if any) to support?

## Testing Strategy

### Unit Tests
- Config parsing
- Privacy rule matching
- URL blocking

### Integration Tests
- Corsair communication
- Transport mode switching
- Privacy header injection

### Manual Tests
- Fingerprinting test sites
- Tor connectivity verification
- UI responsiveness

## Contributing

See AGENTS.md for AI assistant guidelines and coding standards.

## Upstream Tracking

Compass builds on Servo. Key areas to track:

1. **servo/servo** main branch
2. Network stack changes
3. Security fixes
4. WebRender updates

Periodically rebase marctjones/servo fork on upstream.
