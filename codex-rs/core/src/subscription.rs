//! Stripe subscription management and validation.

use crate::cybersec_config::CyberSecConfig;
use base64::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionInfo {
    pub is_active: bool,
    pub subscription_id: Option<String>,
    pub customer_id: Option<String>,
    pub plan_name: String,
    pub expires_at: Option<u64>,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeCustomer {
    pub id: String,
    pub email: String,
    pub subscriptions: Vec<StripeSubscription>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeSubscription {
    pub id: String,
    pub status: String,
    pub current_period_end: u64,
    pub items: Vec<StripeSubscriptionItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeSubscriptionItem {
    pub price: StripePrice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripePrice {
    pub id: String,
    pub nickname: Option<String>,
}

pub struct SubscriptionManager {
    config: CyberSecConfig,
    client: Option<reqwest::Client>,
}

impl SubscriptionManager {
    pub fn new(config: CyberSecConfig) -> Self {
        let client = if config.stripe.enabled && config.stripe.secret_key.is_some() {
            Some(reqwest::Client::new())
        } else {
            None
        };

        Self { config, client }
    }

    /// Check if the user has an active subscription
    pub async fn check_subscription(&self, customer_email: &str) -> Result<SubscriptionInfo, Box<dyn std::error::Error>> {
        // If in dev mode, always return active subscription
        if self.config.dev_mode {
            return Ok(SubscriptionInfo {
                is_active: true,
                subscription_id: Some("dev-mode".to_string()),
                customer_id: Some("dev-customer".to_string()),
                plan_name: "Development Plan".to_string(),
                expires_at: None,
                features: vec![
                    "fix_issues".to_string(),
                    "advanced_analysis".to_string(),
                    "automated_remediation".to_string(),
                    "export_reports".to_string(),
                ],
            });
        }

        // Check if we have a license token (offline validation)
        if let Some(license_token) = &self.config.license.token {
            return self.validate_license_token(license_token);
        }

        // Check Stripe subscription (online validation)
        if let Some(client) = &self.client {
            if let Some(secret_key) = &self.config.stripe.secret_key {
                return self.check_stripe_subscription(client, secret_key, customer_email).await;
            }
        }

        // No valid subscription found
        Ok(SubscriptionInfo {
            is_active: false,
            subscription_id: None,
            customer_id: None,
            plan_name: "Free Plan".to_string(),
            expires_at: None,
            features: vec![], // No premium features
        })
    }

    /// Validate a license token (JWT-like format)
    fn validate_license_token(&self, token: &str) -> Result<SubscriptionInfo, Box<dyn std::error::Error>> {
        // In a real implementation, this would validate the JWT signature
        // For now, we'll do basic JSON parsing
        if let Ok(decoded) = base64::prelude::BASE64_STANDARD.decode(token.split('.').nth(1).unwrap_or("")) {
            if let Ok(claims) = serde_json::from_slice::<serde_json::Value>(&decoded) {
                let exp = claims.get("exp").and_then(|v| v.as_u64()).unwrap_or(0);
                let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
                
                if exp > current_time {
                    return Ok(SubscriptionInfo {
                        is_active: true,
                        subscription_id: Some("license-token".to_string()),
                        customer_id: claims.get("device_id").and_then(|v| v.as_str()).map(String::from),
                        plan_name: claims.get("product").and_then(|v| v.as_str()).unwrap_or("Licensed Plan").to_string(),
                        expires_at: Some(exp),
                        features: vec![
                            "fix_issues".to_string(),
                            "advanced_analysis".to_string(),
                            "automated_remediation".to_string(),
                            "export_reports".to_string(),
                        ],
                    });
                }
            }
        }

        Err("Invalid license token".into())
    }

    /// Check Stripe subscription status
    async fn check_stripe_subscription(
        &self,
        client: &reqwest::Client,
        secret_key: &str,
        customer_email: &str,
    ) -> Result<SubscriptionInfo, Box<dyn std::error::Error>> {
        let auth_header = format!("Bearer {}", secret_key);

        // First, find the customer by email
        let customers_response = client
            .get("https://api.stripe.com/v1/customers")
            .header("Authorization", &auth_header)
            .query(&[("email", customer_email), ("limit", "1")])
            .send()
            .await?;

        if !customers_response.status().is_success() {
            return Err(format!("Stripe API error: {}", customers_response.status()).into());
        }

        let customers_data: serde_json::Value = customers_response.json().await?;
        let default_customers = vec![];
        let customers = customers_data["data"].as_array().unwrap_or(&default_customers);

        if customers.is_empty() {
            return Ok(SubscriptionInfo {
                is_active: false,
                subscription_id: None,
                customer_id: None,
                plan_name: "Free Plan".to_string(),
                expires_at: None,
                features: vec![],
            });
        }

        let customer_id = customers[0]["id"].as_str().unwrap_or("");

        // Get customer's subscriptions
        let subscriptions_response = client
            .get(&format!("https://api.stripe.com/v1/customers/{}/subscriptions", customer_id))
            .header("Authorization", &auth_header)
            .query(&[("status", "active")])
            .send()
            .await?;

        if !subscriptions_response.status().is_success() {
            return Err(format!("Stripe API error: {}", subscriptions_response.status()).into());
        }

        let subscriptions_data: serde_json::Value = subscriptions_response.json().await?;
        let default_subscriptions = vec![];
        let subscriptions = subscriptions_data["data"].as_array().unwrap_or(&default_subscriptions);

        for subscription in subscriptions {
            let status = subscription["status"].as_str().unwrap_or("");
            if status == "active" {
                let subscription_id = subscription["id"].as_str().unwrap_or("").to_string();
                let current_period_end = subscription["current_period_end"].as_u64().unwrap_or(0);
                
                // Check if this subscription matches our price ID
                let default_items = vec![];
                let items = subscription["items"]["data"].as_array().unwrap_or(&default_items);
                for item in items {
                    let price_id = item["price"]["id"].as_str().unwrap_or("");
                    if Some(price_id) == self.config.stripe.price_id.as_deref() {
                        return Ok(SubscriptionInfo {
                            is_active: true,
                            subscription_id: Some(subscription_id),
                            customer_id: Some(customer_id.to_string()),
                            plan_name: "CyberSec Pro".to_string(),
                            expires_at: Some(current_period_end),
                            features: vec![
                                "fix_issues".to_string(),
                                "advanced_analysis".to_string(),
                                "automated_remediation".to_string(),
                                "export_reports".to_string(),
                            ],
                        });
                    }
                }
            }
        }

        Ok(SubscriptionInfo {
            is_active: false,
            subscription_id: None,
            customer_id: Some(customer_id.to_string()),
            plan_name: "Free Plan".to_string(),
            expires_at: None,
            features: vec![],
        })
    }

    /// Create a Stripe checkout session for subscription
    pub async fn create_checkout_session(&self, customer_email: &str, success_url: &str, cancel_url: &str) -> Result<String, Box<dyn std::error::Error>> {
        if !self.config.stripe.enabled {
            return Err("Stripe is not enabled".into());
        }

        let client = self.client.as_ref().ok_or("Stripe client not initialized")?;
        let secret_key = self.config.stripe.secret_key.as_ref().ok_or("Stripe secret key not configured")?;
        let price_id = self.config.stripe.price_id.as_ref().ok_or("Stripe price ID not configured")?;

        let auth_header = format!("Bearer {}", secret_key);

        let params = [
            ("mode", "subscription"),
            ("customer_email", customer_email),
            ("success_url", success_url),
            ("cancel_url", cancel_url),
            ("line_items[0][price]", price_id),
            ("line_items[0][quantity]", "1"),
        ];

        let response = client
            .post("https://api.stripe.com/v1/checkout/sessions")
            .header("Authorization", &auth_header)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Stripe API error: {}", response.status()).into());
        }

        let session_data: serde_json::Value = response.json().await?;
        let checkout_url = session_data["url"].as_str().ok_or("No checkout URL in response")?;

        Ok(checkout_url.to_string())
    }

    /// Get subscription features based on current status
    pub fn get_available_features(&self, subscription: &SubscriptionInfo) -> Vec<&str> {
        if self.config.dev_mode || subscription.is_active {
            vec!["fix_issues", "advanced_analysis", "automated_remediation", "export_reports"]
        } else {
            vec![] // Free tier has no premium features
        }
    }

    /// Check if a specific feature is available
    pub fn is_feature_available(&self, subscription: &SubscriptionInfo, feature: &str) -> bool {
        if self.config.dev_mode {
            return true;
        }

        subscription.is_active && subscription.features.contains(&feature.to_string())
    }

    /// Get user-friendly subscription status message
    pub fn get_subscription_status_message(&self, subscription: &SubscriptionInfo) -> String {
        if self.config.dev_mode {
            return "🔧 Development Mode - All features unlocked".to_string();
        }

        if subscription.is_active {
            if let Some(expires_at) = subscription.expires_at {
                let expires_date = chrono::DateTime::from_timestamp(expires_at as i64, 0)
                    .map(|dt| dt.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "Unknown".to_string());
                format!("✅ {} - Expires: {}", subscription.plan_name, expires_date)
            } else {
                format!("✅ {} - Active", subscription.plan_name)
            }
        } else {
            "❌ Free Plan - Upgrade to unlock advanced features".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dev_mode_subscription() {
        let mut config = CyberSecConfig::default();
        config.dev_mode = true;
        
        let manager = SubscriptionManager::new(config);
        
        // In a real async test, we'd use tokio::test
        // For now, just test the structure
        assert!(manager.config.dev_mode);
    }

    #[test]
    fn test_license_token_parsing() {
        let config = CyberSecConfig::default();
        let manager = SubscriptionManager::new(config);
        
        // Test with invalid token
        let result = manager.validate_license_token("invalid.token.here");
        assert!(result.is_err());
    }
}
