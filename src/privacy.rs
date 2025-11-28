/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Privacy protection features for Compass browser

use crate::config::PrivacyConfig;
use std::collections::HashSet;

/// Privacy manager for the browser
pub struct PrivacyManager {
    config: PrivacyConfig,
    blocked_domains: HashSet<String>,
}

impl PrivacyManager {
    /// Create a new privacy manager
    pub fn new(config: PrivacyConfig) -> Self {
        let blocked_domains = Self::load_blocked_domains();
        Self {
            config,
            blocked_domains,
        }
    }

    /// Load blocked tracking domains
    fn load_blocked_domains() -> HashSet<String> {
        // Common tracking domains - would be loaded from a blocklist in production
        let domains = vec![
            "google-analytics.com",
            "googletagmanager.com",
            "facebook.com/tr",
            "doubleclick.net",
            "analytics.google.com",
            "tracking.example.com",
        ];
        domains.into_iter().map(String::from).collect()
    }

    /// Check if a URL should be blocked
    pub fn should_block_url(&self, url: &str) -> bool {
        if !self.config.block_trackers {
            return false;
        }

        // Parse URL to get domain
        if let Ok(parsed) = url::Url::parse(url) {
            if let Some(host) = parsed.host_str() {
                return self.blocked_domains.iter().any(|d| host.contains(d));
            }
        }
        false
    }

    /// Check if a cookie should be blocked
    pub fn should_block_cookie(&self, domain: &str, is_third_party: bool) -> bool {
        if is_third_party && self.config.block_third_party_cookies {
            return true;
        }
        false
    }

    /// Get headers to add for privacy
    pub fn get_privacy_headers(&self) -> Vec<(String, String)> {
        let mut headers = Vec::new();

        if self.config.do_not_track {
            headers.push(("DNT".to_string(), "1".to_string()));
            headers.push((
                "Sec-GPC".to_string(),
                "1".to_string(),
            ));
        }

        headers
    }

    /// Get fingerprint resistance settings
    pub fn get_fingerprint_resistance_settings(&self) -> FingerprintResistance {
        if self.config.fingerprint_resistance {
            FingerprintResistance::enabled()
        } else {
            FingerprintResistance::disabled()
        }
    }

    /// Check if HTTPS-only mode is enabled
    pub fn is_https_only(&self) -> bool {
        self.config.https_only
    }

    /// Upgrade HTTP URL to HTTPS if in HTTPS-only mode
    pub fn maybe_upgrade_url(&self, url: &str) -> String {
        if self.config.https_only && url.starts_with("http://") {
            url.replacen("http://", "https://", 1)
        } else {
            url.to_string()
        }
    }
}

/// Fingerprint resistance settings
#[derive(Debug, Clone)]
pub struct FingerprintResistance {
    /// Spoof canvas
    pub spoof_canvas: bool,
    /// Spoof WebGL
    pub spoof_webgl: bool,
    /// Spoof audio context
    pub spoof_audio: bool,
    /// Normalize user agent
    pub normalize_user_agent: bool,
    /// Spoof timezone
    pub spoof_timezone: bool,
    /// Spoof language
    pub spoof_language: bool,
    /// Spoofed screen resolution
    pub screen_resolution: Option<(u32, u32)>,
}

impl FingerprintResistance {
    /// Create enabled fingerprint resistance
    pub fn enabled() -> Self {
        Self {
            spoof_canvas: true,
            spoof_webgl: true,
            spoof_audio: true,
            normalize_user_agent: true,
            spoof_timezone: true,
            spoof_language: true,
            screen_resolution: Some((1920, 1080)), // Common resolution
        }
    }

    /// Create disabled fingerprint resistance
    pub fn disabled() -> Self {
        Self {
            spoof_canvas: false,
            spoof_webgl: false,
            spoof_audio: false,
            normalize_user_agent: false,
            spoof_timezone: false,
            spoof_language: false,
            screen_resolution: None,
        }
    }

    /// Get the normalized user agent string
    pub fn get_user_agent(&self) -> Option<String> {
        if self.normalize_user_agent {
            // Use a generic user agent to blend in
            Some("Mozilla/5.0 (Windows NT 10.0; rv:128.0) Gecko/20100101 Firefox/128.0".to_string())
        } else {
            None
        }
    }

    /// Get spoofed timezone
    pub fn get_timezone(&self) -> Option<&'static str> {
        if self.spoof_timezone {
            Some("UTC")
        } else {
            None
        }
    }

    /// Get spoofed language
    pub fn get_language(&self) -> Option<&'static str> {
        if self.spoof_language {
            Some("en-US")
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_blocking() {
        let config = PrivacyConfig::default();
        let manager = PrivacyManager::new(config);

        assert!(manager.should_block_url("https://google-analytics.com/collect"));
        assert!(!manager.should_block_url("https://example.com/"));
    }

    #[test]
    fn test_https_upgrade() {
        let config = PrivacyConfig::default();
        let manager = PrivacyManager::new(config);

        assert_eq!(
            manager.maybe_upgrade_url("http://example.com/"),
            "https://example.com/"
        );
        assert_eq!(
            manager.maybe_upgrade_url("https://example.com/"),
            "https://example.com/"
        );
    }

    #[test]
    fn test_fingerprint_resistance() {
        let fp = FingerprintResistance::enabled();
        assert!(fp.spoof_canvas);
        assert!(fp.get_user_agent().is_some());
        assert_eq!(fp.get_timezone(), Some("UTC"));
    }
}
