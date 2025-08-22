//! Cybersecurity-focused configuration for the AI companion terminal.

use serde::{Deserialize, Serialize};
use std::env;

/// Cybersecurity configuration that extends the base Config
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CyberSecConfig {
    /// Whether the application is running in development mode
    pub dev_mode: bool,
    
    /// Stripe configuration for subscription management
    pub stripe: StripeConfig,
    
    /// License configuration
    pub license: LicenseConfig,
    
    /// Cybersecurity scanning settings
    pub scanning: ScanningConfig,
    
    /// Features that require subscription
    pub subscription_features: SubscriptionFeatures,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct StripeConfig {
    /// Stripe secret key for API access
    pub secret_key: Option<String>,
    
    /// Stripe price ID for subscription plan
    pub price_id: Option<String>,
    
    /// Whether Stripe integration is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LicenseConfig {
    /// Public key for license verification
    pub public_key: Option<String>,
    
    /// License token (JWT format)
    pub token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ScanningConfig {
    /// Enable clipboard monitoring for hijack detection
    pub clipboard_monitoring: bool,
    
    /// Enable malware detection
    pub malware_detection: bool,
    
    /// Enable network traffic analysis
    pub network_analysis: bool,
    
    /// Enable system file integrity checking
    pub file_integrity: bool,
    
    /// Scan interval in seconds
    pub scan_interval: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SubscriptionFeatures {
    /// Whether user can fix detected issues (requires subscription)
    pub can_fix_issues: bool,
    
    /// Whether user can access advanced malware analysis
    pub advanced_analysis: bool,
    
    /// Whether user can use automated remediation
    pub automated_remediation: bool,
    
    /// Whether user can export security reports
    pub export_reports: bool,
}

impl Default for CyberSecConfig {
    fn default() -> Self {
        Self {
            dev_mode: false,
            stripe: StripeConfig::default(),
            license: LicenseConfig::default(),
            scanning: ScanningConfig::default(),
            subscription_features: SubscriptionFeatures::default(),
        }
    }
}

impl Default for StripeConfig {
    fn default() -> Self {
        Self {
            secret_key: None,
            price_id: None,
            enabled: true,
        }
    }
}

impl Default for LicenseConfig {
    fn default() -> Self {
        Self {
            public_key: None,
            token: None,
        }
    }
}

impl Default for ScanningConfig {
    fn default() -> Self {
        Self {
            clipboard_monitoring: true,
            malware_detection: true,
            network_analysis: false, // Disabled by default as it's more resource intensive
            file_integrity: true,
            scan_interval: 30, // 30 seconds
        }
    }
}

impl Default for SubscriptionFeatures {
    fn default() -> Self {
        Self {
            can_fix_issues: false,
            advanced_analysis: false,
            automated_remediation: false,
            export_reports: false,
        }
    }
}

impl CyberSecConfig {
    /// Load cybersecurity configuration from environment variables
    pub fn from_env() -> Self {
        let dev_mode = env::var("DEV_MODE")
            .map(|v| v.to_lowercase() == "true")
            .unwrap_or(false);
            
        let stripe_secret_key = env::var("STRIPE_SECRET_KEY").ok();
        let stripe_price_id = env::var("STRIPE_PRICE_ID").ok();
        
        let license_public_key = env::var("LICENSE_PUBLIC_KEY").ok();
        let license_token = env::var("LICENSE_TOKEN").ok();
        
        // In dev mode or with valid license, enable subscription features
        let has_valid_subscription = dev_mode || license_token.is_some();
        
        Self {
            dev_mode,
            stripe: StripeConfig {
                secret_key: stripe_secret_key,
                price_id: stripe_price_id,
                enabled: true,
            },
            license: LicenseConfig {
                public_key: license_public_key,
                token: license_token,
            },
            scanning: ScanningConfig::default(),
            subscription_features: SubscriptionFeatures {
                can_fix_issues: has_valid_subscription,
                advanced_analysis: has_valid_subscription,
                automated_remediation: has_valid_subscription,
                export_reports: has_valid_subscription,
            },
        }
    }
    
    /// Check if the user has an active subscription or is in dev mode
    pub fn has_active_subscription(&self) -> bool {
        self.dev_mode || self.license.token.is_some()
    }
    
    /// Check if a specific feature is available to the user
    pub fn feature_available(&self, feature: &str) -> bool {
        if self.dev_mode {
            return true;
        }
        
        match feature {
            "fix_issues" => self.subscription_features.can_fix_issues,
            "advanced_analysis" => self.subscription_features.advanced_analysis,
            "automated_remediation" => self.subscription_features.automated_remediation,
            "export_reports" => self.subscription_features.export_reports,
            _ => false,
        }
    }
    
    /// Get a user-friendly message about subscription requirements
    pub fn subscription_message(&self, feature: &str) -> String {
        if self.has_active_subscription() {
            return "Feature available with your subscription.".to_string();
        }
        
        match feature {
            "fix_issues" => "🔒 Issue remediation requires a subscription. I can detect problems, but fixing them requires upgrading.".to_string(),
            "advanced_analysis" => "🔒 Advanced malware analysis requires a subscription. Basic detection is available.".to_string(),
            "automated_remediation" => "🔒 Automated remediation requires a subscription. Manual steps can be provided.".to_string(),
            "export_reports" => "🔒 Report export requires a subscription. View results in the terminal.".to_string(),
            _ => "🔒 This feature requires a subscription.".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_dev_mode_enables_features() {
        env::set_var("DEV_MODE", "true");
        let config = CyberSecConfig::from_env();
        
        assert!(config.dev_mode);
        assert!(config.has_active_subscription());
        assert!(config.feature_available("fix_issues"));
        assert!(config.feature_available("advanced_analysis"));
        
        env::remove_var("DEV_MODE");
    }
    
    #[test]
    fn test_subscription_required_without_dev_mode() {
        env::remove_var("DEV_MODE");
        env::remove_var("LICENSE_TOKEN");
        
        let config = CyberSecConfig::from_env();
        
        assert!(!config.dev_mode);
        assert!(!config.has_active_subscription());
        assert!(!config.feature_available("fix_issues"));
        
        let message = config.subscription_message("fix_issues");
        assert!(message.contains("subscription"));
    }
}
