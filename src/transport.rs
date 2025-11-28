/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Transport layer integration for Compass browser
//!
//! This module integrates with Rigging library for transport abstraction
//! and Corsair for Tor connections.

use crate::config::{NetworkConfig, TorConfig};
use std::process::{Child, Command, Stdio};
use std::path::Path;
use std::time::{Duration, Instant};
use thiserror::Error;
use log::{debug, info, warn, error};

/// Transport errors
#[derive(Debug, Error)]
pub enum TransportError {
    #[error("Corsair daemon not running")]
    CorsairNotRunning,

    #[error("Failed to start Corsair: {0}")]
    CorsairStartFailed(String),

    #[error("Corsair socket not ready after {0} seconds")]
    CorsairTimeout(u64),

    #[error("Transport error: {0}")]
    Transport(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Transport mode for the browser
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportMode {
    /// Direct TCP connections
    Direct,
    /// Route through Tor via Corsair
    Tor,
}

impl Default for TransportMode {
    fn default() -> Self {
        Self::Direct
    }
}

/// Manages browser transport layer
pub struct TransportManager {
    mode: TransportMode,
    tor_config: TorConfig,
    network_config: NetworkConfig,
    corsair_process: Option<Child>,
}

impl TransportManager {
    /// Create a new transport manager
    pub fn new(tor_config: TorConfig, network_config: NetworkConfig) -> Self {
        let mode = if tor_config.enabled {
            TransportMode::Tor
        } else {
            TransportMode::Direct
        };

        Self {
            mode,
            tor_config,
            network_config,
            corsair_process: None,
        }
    }

    /// Get current transport mode
    pub fn mode(&self) -> TransportMode {
        self.mode
    }

    /// Set transport mode
    pub fn set_mode(&mut self, mode: TransportMode) -> Result<(), TransportError> {
        if mode == TransportMode::Tor {
            self.ensure_corsair_running()?;
        }
        self.mode = mode;
        Ok(())
    }

    /// Enable Tor mode
    pub fn enable_tor(&mut self) -> Result<(), TransportError> {
        self.set_mode(TransportMode::Tor)
    }

    /// Disable Tor mode
    pub fn disable_tor(&mut self) {
        self.mode = TransportMode::Direct;
    }

    /// Check if Corsair is running
    pub fn is_corsair_running(&self) -> bool {
        let socket_path = Path::new(&self.tor_config.corsair_socket);
        if !socket_path.exists() {
            return false;
        }

        // Try to connect to verify it's actually running
        #[cfg(unix)]
        {
            use std::os::unix::net::UnixStream;
            UnixStream::connect(socket_path).is_ok()
        }

        #[cfg(not(unix))]
        {
            socket_path.exists()
        }
    }

    /// Ensure Corsair daemon is running
    pub fn ensure_corsair_running(&mut self) -> Result<(), TransportError> {
        if self.is_corsair_running() {
            debug!("Corsair already running");
            return Ok(());
        }

        if !self.tor_config.auto_start_corsair {
            return Err(TransportError::CorsairNotRunning);
        }

        info!("Starting Corsair daemon...");
        self.start_corsair()
    }

    /// Start Corsair daemon
    fn start_corsair(&mut self) -> Result<(), TransportError> {
        // Clean up existing socket
        let socket_path = Path::new(&self.tor_config.corsair_socket);
        if socket_path.exists() {
            let _ = std::fs::remove_file(socket_path);
        }

        // Start Corsair process
        let child = Command::new("corsair")
            .arg("--socket")
            .arg(&self.tor_config.corsair_socket)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| TransportError::CorsairStartFailed(e.to_string()))?;

        self.corsair_process = Some(child);

        // Wait for socket to be ready
        self.wait_for_corsair()?;

        info!("Corsair daemon started");
        Ok(())
    }

    /// Wait for Corsair socket to be ready
    fn wait_for_corsair(&self) -> Result<(), TransportError> {
        let socket_path = Path::new(&self.tor_config.corsair_socket);
        let start = Instant::now();
        let timeout = Duration::from_secs(60); // Tor can take a while to bootstrap

        while start.elapsed() < timeout {
            if socket_path.exists() {
                #[cfg(unix)]
                {
                    use std::os::unix::net::UnixStream;
                    if UnixStream::connect(socket_path).is_ok() {
                        return Ok(());
                    }
                }

                #[cfg(not(unix))]
                {
                    return Ok(());
                }
            }

            std::thread::sleep(Duration::from_millis(500));
        }

        Err(TransportError::CorsairTimeout(60))
    }

    /// Stop Corsair daemon
    pub fn stop_corsair(&mut self) -> Result<(), TransportError> {
        if let Some(ref mut child) = self.corsair_process {
            info!("Stopping Corsair daemon...");

            #[cfg(unix)]
            {
                use nix::sys::signal::{kill, Signal};
                use nix::unistd::Pid;

                if let Ok(pid) = child.id().try_into() {
                    let _ = kill(Pid::from_raw(pid), Signal::SIGTERM);
                    std::thread::sleep(Duration::from_secs(2));
                }
            }

            if child.try_wait()?.is_none() {
                warn!("Corsair didn't stop gracefully, forcing kill");
                child.kill()?;
            }

            child.wait()?;
        }

        self.corsair_process = None;

        // Clean up socket
        let socket_path = Path::new(&self.tor_config.corsair_socket);
        if socket_path.exists() {
            let _ = std::fs::remove_file(socket_path);
        }

        Ok(())
    }

    /// Get Corsair socket path
    pub fn corsair_socket(&self) -> &str {
        &self.tor_config.corsair_socket
    }

    /// Request new Tor identity
    pub fn new_identity(&self) -> Result<(), TransportError> {
        if self.mode != TransportMode::Tor {
            return Ok(());
        }

        if !self.is_corsair_running() {
            return Err(TransportError::CorsairNotRunning);
        }

        // Send new identity request to Corsair
        // This would use the binary IPC protocol
        info!("Requesting new Tor identity...");

        // TODO: Implement new identity request via Corsair IPC

        Ok(())
    }
}

impl Drop for TransportManager {
    fn drop(&mut self) {
        if let Err(e) = self.stop_corsair() {
            error!("Error stopping Corsair on drop: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{NetworkConfig, TorConfig};

    #[test]
    fn test_transport_mode_default() {
        let tor_config = TorConfig::default();
        let network_config = NetworkConfig::default();
        let manager = TransportManager::new(tor_config, network_config);

        assert_eq!(manager.mode(), TransportMode::Direct);
    }

    #[test]
    fn test_transport_mode_tor_enabled() {
        let mut tor_config = TorConfig::default();
        tor_config.enabled = true;
        tor_config.auto_start_corsair = false; // Don't try to start
        let network_config = NetworkConfig::default();
        let manager = TransportManager::new(tor_config, network_config);

        assert_eq!(manager.mode(), TransportMode::Tor);
    }
}
