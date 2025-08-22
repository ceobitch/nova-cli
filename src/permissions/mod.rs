//! macOS Permissions Manager for Bug Spray
//! 
//! Handles requesting and checking macOS permissions needed for security scanning

use anyhow::Result;
use std::process::Command;

pub struct MacPermissions {
    app_bundle_id: String,
}

impl MacPermissions {
    pub fn new() -> Self {
        Self {
            app_bundle_id: "com.bugspray.security".to_string(),
        }
    }

    /// Check if Bug Spray has Full Disk Access permission
    pub async fn has_full_disk_access(&self) -> Result<bool> {
        // Try to access a protected directory
        let output = Command::new("ls")
            .arg("/Library/Application Support/com.apple.TCC/")
            .output();

        match output {
            Ok(result) => Ok(result.status.success()),
            Err(_) => Ok(false),
        }
    }

    /// Check if Bug Spray has Accessibility permission
    pub async fn has_accessibility_access(&self) -> Result<bool> {
        // Check TCC database for accessibility permission
        let output = Command::new("sqlite3")
            .arg("/Library/Application Support/com.apple.TCC/TCC.db")
            .arg(&format!(
                "SELECT COUNT(*) FROM access WHERE service='kTCCServiceAccessibility' AND client='{}' AND auth_value=1;",
                self.app_bundle_id
            ))
            .output();

        match output {
            Ok(result) => {
                let count = String::from_utf8_lossy(&result.stdout);
                Ok(count.trim().parse::<i32>().unwrap_or(0) > 0)
            }
            Err(_) => Ok(false),
        }
    }

    /// Request Full Disk Access permission
    pub async fn request_full_disk_access(&self) -> Result<()> {
        // Open System Preferences to the appropriate pane
        Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles")
            .spawn()?;

        println!("🔐 Please grant Full Disk Access to Bug Spray in System Preferences");
        Ok(())
    }

    /// Request Accessibility permission
    pub async fn request_accessibility_access(&self) -> Result<()> {
        // Open System Preferences to the appropriate pane
        Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
            .spawn()?;

        println!("♿ Please grant Accessibility permission to Bug Spray in System Preferences");
        Ok(())
    }

    /// Check if we can run sudo commands (for system-level scanning)
    pub async fn can_run_sudo(&self) -> Result<bool> {
        let output = Command::new("sudo")
            .arg("-n")
            .arg("true")
            .output();

        match output {
            Ok(result) => Ok(result.status.success()),
            Err(_) => Ok(false),
        }
    }

    /// Request sudo access for system-level operations
    pub async fn request_sudo_access(&self) -> Result<()> {
        println!("🔐 Bug Spray may need administrator access for deep system scanning.");
        println!("You'll be prompted for your password when needed.");
        
        // Test sudo access
        let _output = Command::new("sudo")
            .arg("echo")
            .arg("Bug Spray admin access granted")
            .output()?;

        Ok(())
    }

    /// Get current permission status summary
    pub async fn get_permission_status(&self) -> Result<PermissionStatus> {
        Ok(PermissionStatus {
            full_disk_access: self.has_full_disk_access().await?,
            accessibility: self.has_accessibility_access().await?,
            sudo_available: self.can_run_sudo().await?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PermissionStatus {
    pub full_disk_access: bool,
    pub accessibility: bool,
    pub sudo_available: bool,
}

impl PermissionStatus {
    pub fn is_fully_authorized(&self) -> bool {
        self.full_disk_access && self.accessibility
    }

    pub fn missing_permissions(&self) -> Vec<String> {
        let mut missing = Vec::new();
        
        if !self.full_disk_access {
            missing.push("Full Disk Access".to_string());
        }
        
        if !self.accessibility {
            missing.push("Accessibility".to_string());
        }
        
        missing
    }

    pub fn get_status_summary(&self) -> String {
        if self.is_fully_authorized() {
            "✅ All permissions granted - Bug Spray has full protection capabilities".to_string()
        } else {
            let missing = self.missing_permissions();
            format!(
                "⚠️ Missing permissions: {} - Some features may be limited",
                missing.join(", ")
            )
        }
    }
}
