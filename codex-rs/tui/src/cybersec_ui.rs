//! Cybersecurity-focused UI components and styling.

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Tabs},
};
use codex_core::cybersec::{SecurityThreat, ThreatLevel};
use codex_core::subscription::SubscriptionInfo;

/// Cybersecurity-themed colors
pub struct CyberSecTheme {
    pub primary: Color,
    pub secondary: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub background: Color,
    pub text: Color,
}

impl Default for CyberSecTheme {
    fn default() -> Self {
        Self {
            primary: Color::Cyan,       // Classic "hacker" cyan
            secondary: Color::Green,    // Matrix green
            success: Color::Green,
            warning: Color::Yellow,
            danger: Color::Red,
            background: Color::Black,
            text: Color::White,
        }
    }
}

pub struct SecurityDashboard {
    theme: CyberSecTheme,
    threats: Vec<SecurityThreat>,
    subscription: Option<SubscriptionInfo>,
    selected_tab: usize,
}

impl SecurityDashboard {
    pub fn new() -> Self {
        Self {
            theme: CyberSecTheme::default(),
            threats: Vec::new(),
            subscription: None,
            selected_tab: 0,
        }
    }

    pub fn update_threats(&mut self, threats: Vec<SecurityThreat>) {
        self.threats = threats;
    }

    pub fn update_subscription(&mut self, subscription: SubscriptionInfo) {
        self.subscription = Some(subscription);
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % 4; // 4 tabs total
    }

    pub fn prev_tab(&mut self) {
        self.selected_tab = if self.selected_tab == 0 { 3 } else { self.selected_tab - 1 };
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(3), // Tabs
                Constraint::Min(0),    // Content
                Constraint::Length(3), // Status bar
            ])
            .split(area);

        // Render header
        self.render_header(frame, chunks[0]);
        
        // Render tabs
        self.render_tabs(frame, chunks[1]);
        
        // Render content based on selected tab
        match self.selected_tab {
            0 => self.render_threats_view(frame, chunks[2]),
            1 => self.render_scanning_view(frame, chunks[2]),
            2 => self.render_reports_view(frame, chunks[2]),
            3 => self.render_settings_view(frame, chunks[2]),
            _ => {}
        }
        
        // Render status bar
        self.render_status_bar(frame, chunks[3]);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let ascii_art = r#"
 ▄████▄▓██   ██▓ ▄▄▄▄   ▓█████  ██▀███   ██████ ▓█████  ▄████▄  
▒██▀ ▀█ ▒██  ██▒▓█████▄ ▓█   ▀ ▓██ ▒ ██▒▒██    ▒ ▓█   ▀ ▒██▀ ▀█  
▒▓█    ▄ ▒██ ██░▒██▒ ▄██▒███   ▓██ ░▄█ ▒░ ▓██▄   ▒███   ▒▓█    ▄ 
▒▓▓▄ ▄██▒░ ▐██▓░▒██░█▀  ▒▓█  ▄ ▒██▀▀█▄    ▒   ██▒▒▓█  ▄ ▒▓▓▄ ▄██▒
▒ ▓███▀ ░░ ██▒▓░░▓█  ▀█▓░▒████▒░██▓ ▒██▒▒██████▒▒░▒████▒▒ ▓███▀ ░
░ ░▒ ▒  ░ ██▒▒▒ ░▒▓███▀▒░░ ▒░ ░░ ▒▓ ░▒▓░▒ ▒▓▒ ▒ ░░░ ▒░ ░░ ░▒ ▒  ░
"#;

        let header_text = format!("🛡️  CYBERSEC AI TERMINAL  🛡️\n{}", 
            if let Some(sub) = &self.subscription {
                if sub.is_active { "✅ PRO USER" } else { "🔒 FREE USER" }
            } else { "🔍 LOADING..." }
        );

        let header = Paragraph::new(header_text)
            .style(Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.primary))
                .title("CyberSec AI Terminal")
                .title_style(Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD))
            );

        frame.render_widget(header, area);
    }

    fn render_tabs(&self, frame: &mut Frame, area: Rect) {
        let tab_titles = vec!["🚨 Threats", "🔍 Scanning", "📊 Reports", "⚙️ Settings"];
        
        let tabs = Tabs::new(tab_titles)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(self.theme.secondary)))
            .select(self.selected_tab)
            .style(Style::default().fg(self.theme.text))
            .highlight_style(Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD));

        frame.render_widget(tabs, area);
    }

    fn render_threats_view(&self, frame: &mut Frame, area: Rect) {
        if self.threats.is_empty() {
            let no_threats = Paragraph::new("✅ No active threats detected\n\nYour system appears to be secure.")
                .style(Style::default().fg(self.theme.success))
                .alignment(Alignment::Center)
                .block(Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.success))
                    .title("Threat Status")
                );
            frame.render_widget(no_threats, area);
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Left side - threat list
        let threat_items: Vec<ListItem> = self.threats.iter()
            .map(|threat| {
                let color = match threat.threat_level {
                    ThreatLevel::Critical => self.theme.danger,
                    ThreatLevel::High => Color::LightRed,
                    ThreatLevel::Medium => self.theme.warning,
                    ThreatLevel::Low => Color::LightYellow,
                    ThreatLevel::None => self.theme.text,
                };
                
                let text = format!("{} {} [{}]", 
                    threat.threat_level.emoji(),
                    threat.threat_type,
                    threat.threat_level.as_str()
                );
                
                ListItem::new(text).style(Style::default().fg(color))
            })
            .collect();

        let threat_list = List::new(threat_items)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.danger))
                .title("Active Threats")
            );

        frame.render_widget(threat_list, chunks[0]);

        // Right side - threat details
        if let Some(threat) = self.threats.first() {
            let details = format!(
                "Type: {}\nLevel: {}\nConfidence: {:.0}%\n\nDescription:\n{}\n\nRecommendations:\n{}",
                threat.threat_type,
                threat.threat_level.as_str(),
                threat.confidence * 100.0,
                threat.description,
                threat.recommendations.iter()
                    .enumerate()
                    .map(|(i, rec)| format!("{}. {}", i + 1, rec))
                    .collect::<Vec<_>>()
                    .join("\n")
            );

            let threat_details = Paragraph::new(details)
                .style(Style::default().fg(self.theme.text))
                .wrap(ratatui::widgets::Wrap { trim: true })
                .block(Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.primary))
                    .title("Threat Details")
                );

            frame.render_widget(threat_details, chunks[1]);
        }
    }

    fn render_scanning_view(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),  // Scan controls
                Constraint::Length(3),  // Progress
                Constraint::Min(0),     // Results
            ])
            .split(area);

        // Scan controls
        let controls_text = "🔍 Available Scans:\n\n• Quick Scan (Ctrl+Q) - Fast security check\n• Full Scan (Ctrl+F) - Complete system analysis\n• Custom Scan (Ctrl+C) - Target specific directories";
        
        let controls = Paragraph::new(controls_text)
            .style(Style::default().fg(self.theme.text))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.secondary))
                .title("Scan Controls")
            );

        frame.render_widget(controls, chunks[0]);

        // Progress bar (mock)
        let progress = Gauge::default()
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.primary))
                .title("Scan Progress")
            )
            .gauge_style(Style::default().fg(self.theme.primary))
            .percent(0)
            .label("Ready to scan");

        frame.render_widget(progress, chunks[1]);

        // Scan results
        let results_text = if let Some(sub) = &self.subscription {
            if sub.is_active {
                "📋 Recent Scan Results:\n\n✅ System files: Clean\n✅ Registry: No threats\n⚠️  Downloads folder: 2 suspicious files quarantined\n✅ Running processes: All verified\n\nLast scan: Just now"
            } else {
                "🔒 Advanced scanning requires a subscription.\n\nWith a CyberSec Pro subscription, you get:\n• Real-time threat monitoring\n• Advanced malware detection\n• Automated threat remediation\n• Detailed security reports\n\nUpgrade now to unlock full protection!"
            }
        } else {
            "🔄 Loading scan capabilities..."
        };

        let results = Paragraph::new(results_text)
            .style(Style::default().fg(if self.subscription.as_ref().map_or(false, |s| s.is_active) {
                self.theme.text
            } else {
                self.theme.warning
            }))
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.secondary))
                .title("Scan Results")
            );

        frame.render_widget(results, chunks[2]);
    }

    fn render_reports_view(&self, frame: &mut Frame, area: Rect) {
        let is_pro = self.subscription.as_ref().map_or(false, |s| s.is_active);
        
        let content = if is_pro {
            "📊 Security Reports\n\n🟢 Security Score: 87/100\n\n📈 Threat Trends (Last 7 days):\n• Malware detections: 3 (↓ 40%)\n• Suspicious activity: 12 (↑ 15%)\n• Blocked connections: 156 (↓ 5%)\n\n📋 Available Reports:\n• Daily Security Summary\n• Weekly Threat Analysis\n• Monthly Security Audit\n• Custom Report Builder\n\nPress 'E' to export current report"
        } else {
            "🔒 Security Reports - Pro Feature\n\nUpgrade to CyberSec Pro to access:\n\n📊 Real-time security scoring\n📈 Threat trend analysis\n📋 Detailed security reports\n📤 Report export capabilities\n📧 Email alerts\n🔍 Historical threat data\n\nYour security matters. Upgrade today!"
        };

        let reports = Paragraph::new(content)
            .style(Style::default().fg(if is_pro { self.theme.text } else { self.theme.warning }))
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(if is_pro { self.theme.success } else { self.theme.warning }))
                .title("Security Reports")
            );

        frame.render_widget(reports, area);
    }

    fn render_settings_view(&self, frame: &mut Frame, area: Rect) {
        let subscription_status = if let Some(sub) = &self.subscription {
            if sub.is_active {
                format!("✅ {} - Active", sub.plan_name)
            } else {
                "❌ Free Plan - Limited Features".to_string()
            }
        } else {
            "🔄 Loading subscription status...".to_string()
        };

        let content = format!(
            "⚙️ Settings & Configuration\n\n🔐 Subscription Status:\n{}\n\n🛡️ Security Settings:\n• Real-time protection: Enabled\n• Clipboard monitoring: Enabled\n• Network scanning: {}\n• Auto-quarantine: {}\n\n🔧 System Info:\n• OS: {}\n• Scanner version: 1.0.0\n• Last update: Today\n\n📞 Support:\n• Email: support@cybersec-ai.com\n• Docs: Press 'H' for help\n• Upgrade: Press 'U' to upgrade",
            subscription_status,
            if self.subscription.as_ref().map_or(false, |s| s.is_active) { "Enabled" } else { "Requires Pro" },
            if self.subscription.as_ref().map_or(false, |s| s.is_active) { "Enabled" } else { "Requires Pro" },
            std::env::consts::OS
        );

        let settings = Paragraph::new(content)
            .style(Style::default().fg(self.theme.text))
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.secondary))
                .title("Settings")
            );

        frame.render_widget(settings, area);
    }

    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let threat_count = self.threats.len();
        let highest_level = if threat_count > 0 {
            self.threats.iter()
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
        } else {
            ThreatLevel::None
        };

        let status_text = if threat_count == 0 {
            "🟢 SECURE - No threats detected | Use ← → to navigate tabs | Press 'q' to quit"
        } else {
            "🔴 THREATS DETECTED - Immediate attention required | Use ← → to navigate tabs"
        };

        let status_color = if threat_count == 0 { self.theme.success } else { self.theme.danger };

        let status_bar = Paragraph::new(status_text)
            .style(Style::default().fg(status_color).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(status_color))
            );

        frame.render_widget(status_bar, area);
    }
}

impl Default for SecurityDashboard {
    fn default() -> Self {
        Self::new()
    }
}

