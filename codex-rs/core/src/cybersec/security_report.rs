//! Security reporting and issue tracking.

use crate::cybersec::{SecurityThreat, ThreatLevel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IssueType {
    Malware,
    ClipboardHijack,
    NetworkAnomaly,
    FileIntegrity,
    SystemVulnerability,
    UnauthorizedAccess,
    SuspiciousProcess,
    DataExfiltration,
}

impl IssueType {
    pub fn as_str(&self) -> &'static str {
        match self {
            IssueType::Malware => "Malware",
            IssueType::ClipboardHijack => "Clipboard Hijack",
            IssueType::NetworkAnomaly => "Network Anomaly",
            IssueType::FileIntegrity => "File Integrity",
            IssueType::SystemVulnerability => "System Vulnerability",
            IssueType::UnauthorizedAccess => "Unauthorized Access",
            IssueType::SuspiciousProcess => "Suspicious Process",
            IssueType::DataExfiltration => "Data Exfiltration",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            IssueType::Malware => "🦠",
            IssueType::ClipboardHijack => "📋",
            IssueType::NetworkAnomaly => "🌐",
            IssueType::FileIntegrity => "📁",
            IssueType::SystemVulnerability => "🔧",
            IssueType::UnauthorizedAccess => "🔓",
            IssueType::SuspiciousProcess => "⚙️",
            IssueType::DataExfiltration => "📤",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIssue {
    pub id: String,
    pub issue_type: IssueType,
    pub title: String,
    pub description: String,
    pub severity: ThreatLevel,
    pub status: IssueStatus,
    #[serde(skip, default = "Instant::now")]
    pub detected_at: Instant,
    #[serde(skip)]
    pub resolved_at: Option<Instant>,
    pub affected_files: Vec<String>,
    pub mitigation_steps: Vec<String>,
    pub technical_details: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IssueStatus {
    Active,
    Investigating,
    Mitigated,
    Resolved,
    FalsePositive,
}

impl IssueStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            IssueStatus::Active => "Active",
            IssueStatus::Investigating => "Investigating",
            IssueStatus::Mitigated => "Mitigated",
            IssueStatus::Resolved => "Resolved",
            IssueStatus::FalsePositive => "False Positive",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            IssueStatus::Active => "🔴",
            IssueStatus::Investigating => "🔍",
            IssueStatus::Mitigated => "🟡",
            IssueStatus::Resolved => "✅",
            IssueStatus::FalsePositive => "❌",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityReport {
    pub id: String,
    #[serde(skip, default = "Instant::now")]
    pub generated_at: Instant,
    pub scan_duration: std::time::Duration,
    pub issues: Vec<SecurityIssue>,
    pub summary: ReportSummary,
    pub system_info: SystemInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSummary {
    pub total_issues: usize,
    pub critical_issues: usize,
    pub high_issues: usize,
    pub medium_issues: usize,
    pub low_issues: usize,
    pub resolved_issues: usize,
    pub security_score: f64, // 0.0 to 100.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub hostname: String,
    pub scan_scope: String,
    pub scanner_version: String,
}

impl SecurityIssue {
    pub fn new(
        issue_type: IssueType,
        title: String,
        description: String,
        severity: ThreatLevel,
    ) -> Self {
        Self {
            id: format!("{}-{}", issue_type.as_str().to_lowercase().replace(' ', "-"), Instant::now().elapsed().as_millis()),
            issue_type,
            title,
            description,
            severity,
            status: IssueStatus::Active,
            detected_at: Instant::now(),
            resolved_at: None,
            affected_files: Vec::new(),
            mitigation_steps: Vec::new(),
            technical_details: HashMap::new(),
        }
    }

    pub fn from_threat(threat: &SecurityThreat) -> Self {
        let issue_type = match threat.threat_type.as_str() {
            s if s.contains("Malware") => IssueType::Malware,
            s if s.contains("Clipboard") => IssueType::ClipboardHijack,
            s if s.contains("Network") => IssueType::NetworkAnomaly,
            s if s.contains("Process") => IssueType::SuspiciousProcess,
            _ => IssueType::SystemVulnerability,
        };

        let mut issue = Self::new(
            issue_type,
            threat.threat_type.clone(),
            threat.description.clone(),
            threat.threat_level.clone(),
        );

        issue.affected_files = threat.affected_resources.clone();
        issue.mitigation_steps = threat.recommendations.clone();
        issue.technical_details.insert("confidence".to_string(), format!("{:.1}%", threat.confidence * 100.0));
        issue.technical_details.insert("threat_id".to_string(), threat.id.clone());

        issue
    }

    pub fn resolve(&mut self) {
        self.status = IssueStatus::Resolved;
        self.resolved_at = Some(Instant::now());
    }

    pub fn mark_false_positive(&mut self) {
        self.status = IssueStatus::FalsePositive;
        self.resolved_at = Some(Instant::now());
    }

    pub fn add_mitigation_step(&mut self, step: String) {
        self.mitigation_steps.push(step);
    }

    pub fn add_technical_detail(&mut self, key: String, value: String) {
        self.technical_details.insert(key, value);
    }

    pub fn format_for_display(&self) -> String {
        let status_line = format!("{} {} [{}]", 
            self.issue_type.emoji(), 
            self.title, 
            self.severity.as_str()
        );

        let status_info = format!("{} {}", 
            self.status.emoji(), 
            self.status.as_str()
        );

        let mut output = format!("{}\nStatus: {}\n{}", status_line, status_info, self.description);

        if !self.affected_files.is_empty() {
            output.push_str(&format!("\n📁 Affected Files:\n"));
            for file in &self.affected_files {
                output.push_str(&format!("  • {}\n", file));
            }
        }

        if !self.mitigation_steps.is_empty() {
            output.push_str(&format!("\n🛠️  Mitigation Steps:\n"));
            for (i, step) in self.mitigation_steps.iter().enumerate() {
                output.push_str(&format!("  {}. {}\n", i + 1, step));
            }
        }

        output
    }
}

impl SecurityReport {
    pub fn new() -> Self {
        Self {
            id: format!("report-{}", Instant::now().elapsed().as_millis()),
            generated_at: Instant::now(),
            scan_duration: std::time::Duration::from_secs(0),
            issues: Vec::new(),
            summary: ReportSummary::default(),
            system_info: SystemInfo::default(),
        }
    }

    pub fn add_issue(&mut self, issue: SecurityIssue) {
        self.issues.push(issue);
        self.update_summary();
    }

    pub fn add_threat(&mut self, threat: &SecurityThreat) {
        let issue = SecurityIssue::from_threat(threat);
        self.add_issue(issue);
    }

    pub fn resolve_issue(&mut self, issue_id: &str) -> bool {
        if let Some(issue) = self.issues.iter_mut().find(|i| i.id == issue_id) {
            issue.resolve();
            self.update_summary();
            true
        } else {
            false
        }
    }

    fn update_summary(&mut self) {
        let total_issues = self.issues.len();
        let critical_issues = self.issues.iter().filter(|i| matches!(i.severity, ThreatLevel::Critical)).count();
        let high_issues = self.issues.iter().filter(|i| matches!(i.severity, ThreatLevel::High)).count();
        let medium_issues = self.issues.iter().filter(|i| matches!(i.severity, ThreatLevel::Medium)).count();
        let low_issues = self.issues.iter().filter(|i| matches!(i.severity, ThreatLevel::Low)).count();
        let resolved_issues = self.issues.iter().filter(|i| matches!(i.status, IssueStatus::Resolved)).count();

        // Calculate security score (0-100)
        let mut score = 100.0;
        score -= critical_issues as f64 * 25.0;
        score -= high_issues as f64 * 15.0;
        score -= medium_issues as f64 * 8.0;
        score -= low_issues as f64 * 3.0;
        score = score.max(0.0);

        // Bonus for resolved issues
        if total_issues > 0 {
            let resolution_rate = resolved_issues as f64 / total_issues as f64;
            score += resolution_rate * 10.0;
        }

        self.summary = ReportSummary {
            total_issues,
            critical_issues,
            high_issues,
            medium_issues,
            low_issues,
            resolved_issues,
            security_score: score.min(100.0),
        };
    }

    pub fn format_summary(&self) -> String {
        let score_emoji = if self.summary.security_score >= 90.0 {
            "🟢"
        } else if self.summary.security_score >= 70.0 {
            "🟡"
        } else {
            "🔴"
        };

        format!(
            "{} Security Score: {:.1}/100\n📊 Issues: {} Total ({} Critical, {} High, {} Medium, {} Low)\n✅ Resolved: {}",
            score_emoji,
            self.summary.security_score,
            self.summary.total_issues,
            self.summary.critical_issues,
            self.summary.high_issues,
            self.summary.medium_issues,
            self.summary.low_issues,
            self.summary.resolved_issues
        )
    }

    pub fn get_active_issues(&self) -> Vec<&SecurityIssue> {
        self.issues.iter()
            .filter(|i| matches!(i.status, IssueStatus::Active | IssueStatus::Investigating))
            .collect()
    }

    pub fn export_to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl Default for ReportSummary {
    fn default() -> Self {
        Self {
            total_issues: 0,
            critical_issues: 0,
            high_issues: 0,
            medium_issues: 0,
            low_issues: 0,
            resolved_issues: 0,
            security_score: 100.0,
        }
    }
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            os: std::env::consts::OS.to_string(),
            hostname: hostname::get()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            scan_scope: "System".to_string(),
            scanner_version: "1.0.0".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_issue_creation() {
        let issue = SecurityIssue::new(
            IssueType::Malware,
            "Test Malware".to_string(),
            "Test description".to_string(),
            ThreatLevel::High,
        );

        assert_eq!(issue.issue_type, IssueType::Malware);
        assert_eq!(issue.severity, ThreatLevel::High);
        assert_eq!(issue.status, IssueStatus::Active);
    }

    #[test]
    fn test_security_report() {
        let mut report = SecurityReport::new();
        
        let issue = SecurityIssue::new(
            IssueType::Malware,
            "Test".to_string(),
            "Test".to_string(),
            ThreatLevel::Critical,
        );

        report.add_issue(issue);
        
        assert_eq!(report.summary.total_issues, 1);
        assert_eq!(report.summary.critical_issues, 1);
        assert!(report.summary.security_score < 100.0);
    }
}
