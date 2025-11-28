/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Compass - Privacy-focused Web Browser
//!
//! Compass is a privacy-focused web browser built on the Servo engine with
//! integrated Tor support via Corsair daemon.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                     Compass Browser                          │
//! ├─────────────────────────────────────────────────────────────┤
//! │                                                              │
//! │  ┌──────────────────┐         ┌──────────────────────────┐  │
//! │  │    Browser UI    │         │      Privacy Engine      │  │
//! │  │  (Servo-based)   │         │  - Tor via Corsair       │  │
//! │  │                  │         │  - Transport via Rigging │  │
//! │  │  - Tab management│         │  - Fingerprint resist    │  │
//! │  │  - Navigation    │         │  - Cookie isolation      │  │
//! │  │  - Bookmarks     │         │                          │  │
//! │  └──────────────────┘         └──────────────────────────┘  │
//! │                                                              │
//! │  ┌──────────────────────────────────────────────────────┐   │
//! │  │                   Rigging Transport                   │   │
//! │  │    TCP | Unix Socket | Named Pipe | Tor (Corsair)    │   │
//! │  └──────────────────────────────────────────────────────┘   │
//! │                                                              │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Features
//!
//! - **Tor Integration**: Routes traffic through Tor via Corsair daemon
//! - **Transport Flexibility**: Supports multiple transports via Rigging
//! - **Privacy by Default**: Fingerprint resistance, cookie isolation
//! - **Servo Engine**: Modern, memory-safe browser engine

pub mod config;
pub mod privacy;
pub mod transport;

pub use config::CompassConfig;
