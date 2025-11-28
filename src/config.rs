/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Compass browser configuration

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main Compass browser configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompassConfig {
    /// Privacy settings
    #[serde(default)]
    pub privacy: PrivacyConfig,

    /// Tor settings
    #[serde(default)]
    pub tor: TorConfig,

    /// Window settings
    #[serde(default)]
    pub window: WindowConfig,

    /// Network settings
    #[serde(default)]
    pub network: NetworkConfig,
}

impl Default for CompassConfig {
    fn default() -> Self {
        Self {
            privacy: PrivacyConfig::default(),
            tor: TorConfig::default(),
            window: WindowConfig::default(),
            network: NetworkConfig::default(),
        }
    }
}

impl CompassConfig {
    /// Load configuration from a TOML file
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config: CompassConfig = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Load configuration from string
    pub fn from_str(toml_str: &str) -> anyhow::Result<Self> {
        let config: CompassConfig = toml::from_str(toml_str)?;
        Ok(config)
    }

    /// Get the default config file path
    pub fn default_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("compass")
            .join("config.toml")
    }
}

/// Privacy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    /// Enable fingerprint resistance
    #[serde(default = "default_true")]
    pub fingerprint_resistance: bool,

    /// Block third-party cookies
    #[serde(default = "default_true")]
    pub block_third_party_cookies: bool,

    /// Clear cookies on exit
    #[serde(default)]
    pub clear_cookies_on_exit: bool,

    /// Clear history on exit
    #[serde(default)]
    pub clear_history_on_exit: bool,

    /// Do Not Track header
    #[serde(default = "default_true")]
    pub do_not_track: bool,

    /// Block trackers
    #[serde(default = "default_true")]
    pub block_trackers: bool,

    /// HTTPS-only mode
    #[serde(default = "default_true")]
    pub https_only: bool,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            fingerprint_resistance: true,
            block_third_party_cookies: true,
            clear_cookies_on_exit: false,
            clear_history_on_exit: false,
            do_not_track: true,
            block_trackers: true,
            https_only: true,
        }
    }
}

/// Tor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorConfig {
    /// Enable Tor by default
    #[serde(default)]
    pub enabled: bool,

    /// Corsair daemon socket path
    #[serde(default = "default_corsair_socket")]
    pub corsair_socket: String,

    /// Start Corsair automatically
    #[serde(default = "default_true")]
    pub auto_start_corsair: bool,

    /// New identity on new tab
    #[serde(default)]
    pub new_identity_per_tab: bool,
}

impl Default for TorConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            corsair_socket: default_corsair_socket(),
            auto_start_corsair: true,
            new_identity_per_tab: false,
        }
    }
}

fn default_corsair_socket() -> String {
    "/tmp/corsair.sock".to_string()
}

/// Window configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    /// Default window width
    #[serde(default = "default_width")]
    pub width: u32,

    /// Default window height
    #[serde(default = "default_height")]
    pub height: u32,

    /// Start maximized
    #[serde(default)]
    pub maximized: bool,

    /// Enable fullscreen mode
    #[serde(default)]
    pub fullscreen: bool,

    /// Show toolbar
    #[serde(default = "default_true")]
    pub show_toolbar: bool,

    /// Show status bar
    #[serde(default = "default_true")]
    pub show_status_bar: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: default_width(),
            height: default_height(),
            maximized: false,
            fullscreen: false,
            show_toolbar: true,
            show_status_bar: true,
        }
    }
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Default transport (tcp, tor)
    #[serde(default = "default_transport")]
    pub default_transport: String,

    /// Connection timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout: u64,

    /// Maximum concurrent connections
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,

    /// Enable DNS over HTTPS
    #[serde(default = "default_true")]
    pub dns_over_https: bool,

    /// DoH server
    #[serde(default = "default_doh_server")]
    pub doh_server: String,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            default_transport: default_transport(),
            timeout: default_timeout(),
            max_connections: default_max_connections(),
            dns_over_https: true,
            doh_server: default_doh_server(),
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_width() -> u32 {
    1280
}

fn default_height() -> u32 {
    800
}

fn default_transport() -> String {
    "tcp".to_string()
}

fn default_timeout() -> u64 {
    30
}

fn default_max_connections() -> u32 {
    100
}

fn default_doh_server() -> String {
    "https://cloudflare-dns.com/dns-query".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = CompassConfig::default();
        assert!(config.privacy.fingerprint_resistance);
        assert!(config.privacy.block_third_party_cookies);
        assert!(!config.tor.enabled);
    }

    #[test]
    fn test_parse_config() {
        let toml = r#"
            [privacy]
            fingerprint_resistance = true
            block_third_party_cookies = true

            [tor]
            enabled = true
            corsair_socket = "/tmp/my-corsair.sock"

            [window]
            width = 1920
            height = 1080
        "#;

        let config = CompassConfig::from_str(toml).unwrap();
        assert!(config.tor.enabled);
        assert_eq!(config.tor.corsair_socket, "/tmp/my-corsair.sock");
        assert_eq!(config.window.width, 1920);
    }
}
