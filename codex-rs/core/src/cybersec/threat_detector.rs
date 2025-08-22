//! Core threat detection types and functionality.

use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThreatLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

impl ThreatLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            ThreatLevel::None => "None",
            ThreatLevel::Low => "Low",
            ThreatLevel::Medium => "Medium", 
            ThreatLevel::High => "High",
            ThreatLevel::Critical => "Critical",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            ThreatLevel::None => "✅",
            ThreatLevel::Low => "🟨",
            ThreatLevel::Medium => "🟧",
            ThreatLevel::High => "🔴",
            ThreatLevel::Critical => "🚨",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityThreat {
    pub id: String,
    pub threat_type: String,
    pub description: String,
    pub threat_level: ThreatLevel,
    pub confidence: f64, // 0.0 to 1.0
    pub affected_resources: Vec<String>,
    pub recommendations: Vec<String>,
    #[serde(skip, default = "Instant::now")]
    pub detected_at: Instant,
}

impl SecurityThreat {
    pub fn new(
        threat_type: String,
        description: String,
        threat_level: ThreatLevel,
        confidence: f64,
    ) -> Self {
        Self {
            id: format!("{}-{}", threat_type.to_lowercase().replace(' ', "-"), Instant::now().elapsed().as_millis()),
            threat_type,
            description,
            threat_level,
            confidence,
            affected_resources: vec![],
            recommendations: vec![],
            detected_at: Instant::now(),
        }
    }

    pub fn add_affected_resource(&mut self, resource: String) {
        self.affected_resources.push(resource);
    }

    pub fn add_recommendation(&mut self, recommendation: String) {
        self.recommendations.push(recommendation);
    }

    pub fn format_for_display(&self) -> String {
        format!(
            "{} {} [{}] Confidence: {:.0}%\n{}\n{}",
            self.threat_level.emoji(),
            self.threat_type,
            self.threat_level.as_str(),
            self.confidence * 100.0,
            self.description,
            if !self.recommendations.is_empty() {
                format!("💡 Recommendations:\n{}", 
                    self.recommendations.iter()
                        .map(|r| format!("  • {}", r))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            } else {
                String::new()
            }
        )
    }
}

pub struct ThreatDetector {
    active_threats: Vec<SecurityThreat>,
    resolved_threats: Vec<SecurityThreat>,
}

impl ThreatDetector {
    pub fn new() -> Self {
        Self {
            active_threats: Vec::new(),
            resolved_threats: Vec::new(),
        }
    }

    pub fn add_threat(&mut self, threat: SecurityThreat) {
        self.active_threats.push(threat);
    }

    pub fn resolve_threat(&mut self, threat_id: &str) -> bool {
        if let Some(pos) = self.active_threats.iter().position(|t| t.id == threat_id) {
            let threat = self.active_threats.remove(pos);
            self.resolved_threats.push(threat);
            true
        } else {
            false
        }
    }

    pub fn get_active_threats(&self) -> &[SecurityThreat] {
        &self.active_threats
    }

    pub fn get_threats_by_level(&self, level: ThreatLevel) -> Vec<&SecurityThreat> {
        self.active_threats.iter()
            .filter(|t| t.threat_level == level)
            .collect()
    }

    pub fn get_highest_threat_level(&self) -> ThreatLevel {
        self.active_threats.iter()
            .map(|t| &t.threat_level)
            .max_by_key(|level| match level {
                ThreatLevel::None => 0,
                ThreatLevel::Low => 1,
                ThreatLevel::Medium => 2,
                ThreatLevel::High => 3,
                ThreatLevel::Critical => 4,
            })
            .cloned()
            .unwrap_or(ThreatLevel::None)
    }

    pub fn clear_all_threats(&mut self) {
        self.resolved_threats.extend(self.active_threats.drain(..));
    }

    pub fn get_threat_summary(&self) -> String {
        if self.active_threats.is_empty() {
            return "✅ No active security threats detected".to_string();
        }

        let critical = self.get_threats_by_level(ThreatLevel::Critical).len();
        let high = self.get_threats_by_level(ThreatLevel::High).len();
        let medium = self.get_threats_by_level(ThreatLevel::Medium).len();
        let low = self.get_threats_by_level(ThreatLevel::Low).len();

        let mut parts = vec![];
        if critical > 0 { parts.push(format!("🚨 {} Critical", critical)); }
        if high > 0 { parts.push(format!("🔴 {} High", high)); }
        if medium > 0 { parts.push(format!("🟧 {} Medium", medium)); }
        if low > 0 { parts.push(format!("🟨 {} Low", low)); }

        format!("⚠️ Active Threats: {}", parts.join(", "))
    }
}

impl Default for ThreatDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threat_creation() {
        let threat = SecurityThreat::new(
            "Test Threat".to_string(),
            "A test threat".to_string(),
            ThreatLevel::Medium,
            0.8,
        );

        assert_eq!(threat.threat_type, "Test Threat");
        assert_eq!(threat.threat_level, ThreatLevel::Medium);
        assert_eq!(threat.confidence, 0.8);
    }

    #[test]
    fn test_threat_detector() {
        let mut detector = ThreatDetector::new();
        
        let threat = SecurityThreat::new(
            "Test".to_string(),
            "Test description".to_string(),
            ThreatLevel::High,
            0.9,
        );
        
        let threat_id = threat.id.clone();
        detector.add_threat(threat);
        
        assert_eq!(detector.get_active_threats().len(), 1);
        assert_eq!(detector.get_highest_threat_level(), ThreatLevel::High);
        
        detector.resolve_threat(&threat_id);
        assert_eq!(detector.get_active_threats().len(), 0);
    }
}
