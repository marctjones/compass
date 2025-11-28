# Upstream Servo Tracking Strategy

## Overview

This document describes how Compass and related projects should track the upstream Servo project, what features are missing from upstream, and the strategy for contributing back vs. maintaining locally.

## Repository Relationships

```
┌─────────────────────────────────────────────────────────────────┐
│                    servo/servo (upstream)                        │
│                    - Base browser engine                        │
│                    - TCP/HTTPS only                             │
│                    - No transport abstraction                   │
└──────────────────────────┬──────────────────────────────────────┘
                           │ fork
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│                  marctjones/servo (fork)                        │
│                  - Track upstream main branch                   │
│                  - Add transport abstraction patches            │
│                  - Privacy feature patches                      │
└──────────────────────────┬──────────────────────────────────────┘
                           │ depends on
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Compass (browser)                          │
│                      - Uses forked Servo                        │
│                      - Privacy features                         │
│                      - Tor integration                          │
└─────────────────────────────────────────────────────────────────┘
```

## Features Missing from Upstream Servo

### 1. Transport Layer Abstraction

**Status in Upstream**: Not present - Servo only supports TCP/HTTPS
**Status in Our Fork**: Fully implemented

| Feature | Upstream | Our Fork | Priority |
|---------|----------|----------|----------|
| Transport URL parsing | ❌ | ✅ | High |
| Unix Domain Sockets | ❌ | ✅ | High |
| Named Pipes (Windows) | ❌ | 🚧 | Medium |
| Transport composition | ❌ | ✅ | Medium |
| Tor transport | ❌ | ✅ | High |
| QUIC transport | ❌ | 🚧 | Low |

**Recommendation**: Maintain locally. The transport URL syntax and multi-transport architecture is specific to our use case. Upstream Servo has no stated interest in this.

### 2. Unix Domain Socket Support

**Status in Upstream**: Not present
**Status in Our Fork**: Complete

Components:
- `ServoUnixConnector` - Maps hostnames to socket paths
- `SocketMapping` - Path validation and security
- `hyperlocal` integration - HTTP over Unix sockets
- `unix_client` in HttpState

**Recommendation**: Consider proposing upstream as an optional feature. UDS is useful for:
- Container environments
- Microservice communication
- Local development servers

Could be upstreamed as `--features unix-sockets`

### 3. Privacy Features

**Status in Upstream**: Minimal
**Status in Our Fork**: To be implemented

| Feature | Upstream | Our Fork | Upstream Interest |
|---------|----------|----------|-------------------|
| Fingerprint resistance | ❌ | 🚧 | Unknown |
| Tracker blocking | ❌ | 🚧 | Unknown |
| Cookie isolation | Partial | 🚧 | Possible |
| HTTPS-only mode | ❌ | 🚧 | Likely yes |

**Recommendation**:
- HTTPS-only mode: Propose upstream (widely useful)
- Fingerprint resistance: Maintain locally (complex, opinionated)
- Tracker blocking: Maintain locally (requires blocklist maintenance)

### 4. Tor Integration

**Status in Upstream**: Not present and not wanted
**Status in Our Fork**: Via Corsair daemon

**Recommendation**: Maintain locally. Tor is outside Servo's scope.

### 5. Embedded Application Support (servoapp/libservo)

**Status in Upstream**: Basic embedding API exists
**Status in Our Fork**: Enhanced for Harbor use case

**Recommendation**: Watch upstream `libservo` development. May be able to contribute improvements.

## Tracking Strategy by Project

### marctjones/servo (Fork)

**Tracking Approach**: Rebase on upstream periodically

```bash
# Setup remotes
git remote add upstream https://github.com/servo/servo.git
git remote add origin https://github.com/marctjones/servo.git

# Tracking workflow
git fetch upstream
git checkout main
git rebase upstream/main

# Resolve conflicts in:
# - components/net/Cargo.toml (our added dependencies)
# - components/net/http_loader.rs (our transport dispatch)
# - components/net/lib.rs (our module exports)
# - components/shared/net/ (our transport types)

git push origin main --force-with-lease
```

**Frequency**: Monthly or when upstream has significant changes

**Conflict-Prone Files**:
- `components/net/Cargo.toml` - Dependencies
- `components/net/http_loader.rs` - Request dispatch
- `components/net/lib.rs` - Module structure
- `components/shared/net/lib.rs` - Shared types

### Compass (Browser)

**Tracking Approach**: Depend on marctjones/servo fork

```toml
# Cargo.toml
[dependencies]
servo = { git = "https://github.com/marctjones/servo", branch = "main" }
```

**When Upstream Changes**:
1. Update fork first
2. Rebuild Compass
3. Fix any API breakage
4. Update privacy hooks if rendering changed

### Rigging (Transport Library)

**Tracking Approach**: Independent - no Servo dependency

Rigging is a standalone library. It duplicates some types from servoipc's transport layer but:
- Can be used without Servo
- Simpler API for external users
- Harbor uses Rigging, not raw Servo

**Sync Strategy**: When servoipc transport types change, evaluate if Rigging needs updates.

### Corsair (Tor Daemon)

**Tracking Approach**: Independent - no Servo dependency

Corsair only needs to:
- Track Arti releases
- Maintain IPC protocol compatibility

### Harbor (Local App Framework)

**Tracking Approach**: Uses Rigging, not Servo directly

Harbor only interacts with Servo through servoapp/libservo embedding.

## What to Implement vs. Wait

### Implement Ourselves

| Feature | Reason |
|---------|--------|
| Transport URL syntax | Core to our architecture, not upstream priority |
| Unix socket connector | Core to Harbor use case |
| Tor integration | Not in Servo's scope |
| Fingerprint resistance | Too opinionated for upstream |
| Composed transports | Unique to our use case |

### Propose to Upstream

| Feature | Rationale |
|---------|-----------|
| HTTPS-only mode | Widely useful, simple implementation |
| Unix sockets (optional) | Container-friendly, opt-in feature |
| Better embedding API | Benefits all libservo users |

### Wait for Upstream

| Feature | Status |
|---------|--------|
| WebGPU improvements | Active development |
| WebXR improvements | Active development |
| Performance improvements | Continuous |
| Security fixes | Critical - merge immediately |

## Merge Strategy for Security Fixes

When upstream Servo publishes security fixes:

1. **Immediate**: Fetch and review the fix
2. **Same Day**: Apply to marctjones/servo fork
3. **Test**: Ensure our transport modifications don't break
4. **Release**: Update Compass with fixed version

```bash
# Security fix workflow
git fetch upstream
git cherry-pick <security-commit>
git push origin main
```

## Version Pinning

### Current Pins

```toml
# Compass Cargo.toml
[dependencies]
servo = { git = "https://github.com/marctjones/servo", rev = "abc123" }
rigging = { git = "https://github.com/marctjones/rigging", tag = "v0.1.0" }
```

### When to Update Pins

- **Security fix**: Immediately
- **Upstream feature needed**: As needed
- **Regular maintenance**: Monthly

## Contributing Back to Upstream

### Good Candidates for Upstream PRs

1. **Bug fixes** we discover
2. **Documentation** improvements
3. **HTTPS-only mode** implementation
4. **Unix socket support** as optional feature
5. **Test improvements**

### PR Process

1. Implement in our fork first
2. Test thoroughly
3. Extract minimal change for PR
4. Follow Servo's contribution guidelines
5. Be patient - Servo has limited maintainers

### Keep Local

- Transport URL syntax (too specialized)
- Tor integration (out of scope)
- Privacy fingerprinting (too opinionated)
- Harbor-specific servoapp changes

## Monitoring Upstream

### Watch These Areas

1. **components/net/**: Network stack changes
2. **components/script/**: Privacy-relevant APIs
3. **ports/libservo/**: Embedding API changes
4. **components/canvas/**: Fingerprinting surface
5. **components/webgl/**: Fingerprinting surface

### Servo Communication Channels

- GitHub Issues: https://github.com/servo/servo/issues
- Zulip Chat: https://servo.zulipchat.com/
- Blog: https://servo.org/blog/

## Conflict Resolution Patterns

### Transport Dispatch Conflicts

When upstream changes `http_loader.rs` fetch logic:

```rust
// Our pattern: Check transport first, then call upstream logic
fn http_fetch(...) {
    // === OUR CODE START ===
    if let Some(transport) = get_transport_from_url(&url) {
        return handle_transport_fetch(transport, ...);
    }
    // === OUR CODE END ===

    // Original upstream code continues...
}
```

### Dependency Conflicts

When upstream updates hyper/tokio versions:

1. Check if hyperlocal supports new version
2. Update our connectors for API changes
3. Test Unix socket functionality
4. Update Rigging if needed

## Summary

| Project | Tracks | Strategy |
|---------|--------|----------|
| marctjones/servo | servo/servo | Periodic rebase |
| Compass | marctjones/servo | Pin to commit |
| Rigging | None (independent) | Standalone |
| Corsair | Arti releases | Track Arti |
| Harbor | Rigging + servoapp | Pin versions |
