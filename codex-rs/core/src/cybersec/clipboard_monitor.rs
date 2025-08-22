//! Clipboard monitoring for detecting potential hijacking attempts.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use crate::cybersec::{SecurityThreat, ThreatLevel};

/// Maximum number of clipboard changes to track
const MAX_CLIPBOARD_HISTORY: usize = 100;

/// Threshold for rapid clipboard changes (suspicious activity)
const RAPID_CHANGE_THRESHOLD: Duration = Duration::from_millis(500);

/// Maximum suspicious changes before triggering alert
const MAX_RAPID_CHANGES: usize = 5;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardChange {
    #[serde(skip, default = "Instant::now")]
    pub timestamp: Instant,
    pub content_hash: u64,
    pub content_length: usize,
    pub content_type: ClipboardContentType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClipboardContentType {
    Text,
    Image,
    File,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardAnalysis {
    pub is_suspicious: bool,
    pub rapid_changes: usize,
    pub unusual_patterns: Vec<String>,
    pub threat_level: ThreatLevel,
    pub recommendations: Vec<String>,
}

pub struct ClipboardMonitor {
    history: VecDeque<ClipboardChange>,
    enabled: bool,
    last_content_hash: Option<u64>,
}

impl ClipboardMonitor {
    pub fn new() -> Self {
        Self {
            history: VecDeque::with_capacity(MAX_CLIPBOARD_HISTORY),
            enabled: true,
            last_content_hash: None,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Record a clipboard change event
    pub fn record_change(&mut self, content_hash: u64, content_length: usize, content_type: ClipboardContentType) {
        if !self.enabled {
            return;
        }

        let change = ClipboardChange {
            timestamp: Instant::now(),
            content_hash,
            content_length,
            content_type,
        };

        // Add to history
        if self.history.len() >= MAX_CLIPBOARD_HISTORY {
            self.history.pop_front();
        }
        self.history.push_back(change);
        
        self.last_content_hash = Some(content_hash);
    }

    /// Analyze clipboard activity for suspicious patterns
    pub fn analyze_activity(&self) -> ClipboardAnalysis {
        if !self.enabled || self.history.is_empty() {
            return ClipboardAnalysis {
                is_suspicious: false,
                rapid_changes: 0,
                unusual_patterns: vec![],
                threat_level: ThreatLevel::None,
                recommendations: vec![],
            };
        }

        let rapid_changes = self.count_rapid_changes();
        let unusual_patterns = self.detect_unusual_patterns();
        
        let is_suspicious = rapid_changes > MAX_RAPID_CHANGES || !unusual_patterns.is_empty();
        
        let threat_level = if rapid_changes > MAX_RAPID_CHANGES * 2 {
            ThreatLevel::High
        } else if rapid_changes > MAX_RAPID_CHANGES {
            ThreatLevel::Medium
        } else if !unusual_patterns.is_empty() {
            ThreatLevel::Low
        } else {
            ThreatLevel::None
        };

        let mut recommendations = vec![];
        if rapid_changes > MAX_RAPID_CHANGES {
            recommendations.push("Consider checking running processes for clipboard manipulation software".to_string());
            recommendations.push("Verify recent application installations".to_string());
        }
        if unusual_patterns.contains(&"large_content_changes".to_string()) {
            recommendations.push("Monitor for applications that might be injecting large amounts of data".to_string());
        }

        ClipboardAnalysis {
            is_suspicious,
            rapid_changes,
            unusual_patterns,
            threat_level,
            recommendations,
        }
    }

    /// Count rapid changes within threshold time
    fn count_rapid_changes(&self) -> usize {
        if self.history.len() < 2 {
            return 0;
        }

        let mut rapid_count = 0;
        let recent_time = Instant::now() - Duration::from_secs(60); // Look at last minute

        for window in self.history.iter().collect::<Vec<_>>().windows(2) {
            if let [prev, current] = window {
                if current.timestamp > recent_time &&
                   current.timestamp.duration_since(prev.timestamp) < RAPID_CHANGE_THRESHOLD {
                    rapid_count += 1;
                }
            }
        }

        rapid_count
    }

    /// Detect unusual patterns in clipboard usage
    fn detect_unusual_patterns(&self) -> Vec<String> {
        let mut patterns = vec![];

        if self.history.is_empty() {
            return patterns;
        }

        // Check for unusually large content changes
        let avg_length = self.history.iter()
            .map(|c| c.content_length)
            .sum::<usize>() / self.history.len();

        let recent_changes: Vec<_> = self.history.iter()
            .rev()
            .take(10)
            .collect();

        if let Some(largest) = recent_changes.iter().map(|c| c.content_length).max() {
            if largest > avg_length * 10 && largest > 10000 {
                patterns.push("large_content_changes".to_string());
            }
        }

        // Check for repetitive identical content
        let mut content_counts = std::collections::HashMap::new();
        for change in recent_changes.iter().take(20) {
            *content_counts.entry(change.content_hash).or_insert(0) += 1;
        }

        if content_counts.values().any(|&count| count > 5) {
            patterns.push("repetitive_content".to_string());
        }

        // Check for unusual content type patterns
        let recent_types: Vec<_> = recent_changes.iter()
            .map(|c| &c.content_type)
            .collect();

        let file_type_count = recent_types.iter()
            .filter(|&&t| t == &ClipboardContentType::File)
            .count();

        if file_type_count > recent_types.len() / 2 && recent_types.len() > 10 {
            patterns.push("excessive_file_clipboard_usage".to_string());
        }

        patterns
    }

    /// Generate a security threat based on analysis
    pub fn check_for_threats(&self) -> Option<SecurityThreat> {
        let analysis = self.analyze_activity();
        
        if !analysis.is_suspicious {
            return None;
        }

        let description = if analysis.rapid_changes > MAX_RAPID_CHANGES {
            format!("Detected {} rapid clipboard changes, which may indicate clipboard hijacking malware", analysis.rapid_changes)
        } else {
            format!("Detected unusual clipboard patterns: {}", analysis.unusual_patterns.join(", "))
        };

        Some(SecurityThreat {
            id: format!("clipboard-{}", Instant::now().elapsed().as_millis()),
            threat_type: "Clipboard Hijacking".to_string(),
            description,
            threat_level: analysis.threat_level,
            confidence: if analysis.rapid_changes > MAX_RAPID_CHANGES * 2 { 0.9 } else { 0.6 },
            affected_resources: vec!["System Clipboard".to_string()],
            recommendations: analysis.recommendations,
            detected_at: Instant::now(),
        })
    }

    /// Get recent clipboard activity summary
    pub fn get_activity_summary(&self) -> String {
        if self.history.is_empty() {
            return "No clipboard activity recorded".to_string();
        }

        let recent_count = self.history.iter()
            .filter(|c| c.timestamp.elapsed() < Duration::from_secs(300))
            .count();

        let analysis = self.analyze_activity();
        
        format!(
            "📋 Clipboard Activity: {} changes in last 5 minutes. Rapid changes: {}. Status: {}",
            recent_count,
            analysis.rapid_changes,
            if analysis.is_suspicious { "⚠️ SUSPICIOUS" } else { "✅ Normal" }
        )
    }
}

impl Default for ClipboardMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_monitor_basic() {
        let mut monitor = ClipboardMonitor::new();
        assert!(monitor.is_enabled());
        
        monitor.record_change(12345, 100, ClipboardContentType::Text);
        assert_eq!(monitor.history.len(), 1);
    }

    #[test]
    fn test_rapid_changes_detection() {
        let mut monitor = ClipboardMonitor::new();
        
        // Simulate rapid changes
        for i in 0..10 {
            monitor.record_change(i, 100, ClipboardContentType::Text);
            std::thread::sleep(Duration::from_millis(100)); // Rapid changes
        }
        
        let analysis = monitor.analyze_activity();
        assert!(analysis.rapid_changes > 0);
    }
}
