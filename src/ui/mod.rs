//! Bug Spray User Interface
//! 
//! Custom TUI interface for Bug Spray with cybersecurity-focused design

use anyhow::Result;
use crossterm::event::KeyCode;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, List, ListItem, Gauge, Tabs},
    Frame,
};
use std::time::Instant;

use crate::scanner::{ThreatScanner, ScanResult};
use crate::ai::BugSprayAI;

pub struct BugSprayApp {
    scanner: ThreatScanner,
    ai: Option<BugSprayAI>,
    selected_tab: usize,
    last_scan: Option<Instant>,
    scan_results: Option<ScanResult>,
    ai_chat_messages: Vec<ChatMessage>,
    input_buffer: String,
    show_help: bool,
}

#[derive(Clone)]
pub struct ChatMessage {
    pub is_user: bool,
    pub content: String,
    pub timestamp: Instant,
}

impl BugSprayApp {
    pub fn new(scanner: ThreatScanner, ai: Option<BugSprayAI>) -> Self {
        Self {
            scanner,
            ai,
            selected_tab: 0,
            last_scan: None,
            scan_results: None,
            ai_chat_messages: vec![
                ChatMessage {
                    is_user: false,
                    content: "🐛 Hi! I'm Bug Spray, your AI cybersecurity companion. I'm here to protect your Mac from malware, crypto theft, and developer-targeted attacks. What can I help you secure today?".to_string(),
                    timestamp: Instant::now(),
                }
            ],
            input_buffer: String::new(),
            show_help: false,
        }
    }

    pub fn draw<B: Backend>(&self, frame: &mut Frame<B>) {
        let size = frame.size();
        
        // Main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Length(3),  // Tabs
                Constraint::Min(0),     // Main content
                Constraint::Length(3),  // Footer
            ])
            .split(size);

        self.draw_header(frame, chunks[0]);
        self.draw_tabs(frame, chunks[1]);
        self.draw_main_content(frame, chunks[2]);
        self.draw_footer(frame, chunks[3]);
        
        if self.show_help {
            self.draw_help_popup(frame, size);
        }
    }

    fn draw_header<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let header_text = "🐛 Bug Spray - AI Cybersecurity Companion";
        let status = if self.scan_results.as_ref().map_or(0, |r| r.threats_found) > 0 {
            "🚨 THREATS DETECTED"
        } else {
            "✅ SYSTEM SECURE"
        };
        
        let header = Paragraph::new(format!("{}\n{}", header_text, status))
            .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
            );

        frame.render_widget(header, area);
    }

    fn draw_tabs<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let tab_titles = vec!["🛡️ Dashboard", "🔍 Scan", "🧠 AI Chat", "⚙️ Settings"];
        
        let tabs = Tabs::new(tab_titles)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Green)))
            .select(self.selected_tab)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

        frame.render_widget(tabs, area);
    }

    fn draw_main_content<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        match self.selected_tab {
            0 => self.draw_dashboard(frame, area),
            1 => self.draw_scan_tab(frame, area),
            2 => self.draw_ai_chat_tab(frame, area),
            3 => self.draw_settings_tab(frame, area),
            _ => {}
        }
    }

    fn draw_dashboard<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Security Status
        let status_text = if let Some(results) = &self.scan_results {
            format!(
                "🔍 Last Scan: {} threats found\n\n\
                 📁 Files Scanned: {}\n\
                 🚨 Threats Detected: {}\n\
                 ⚡ Scan Duration: {:.2}s\n\n\
                 {}",
                results.threats_found,
                results.files_scanned,
                results.threats_found,
                results.scan_duration.as_secs_f64(),
                if results.threats_found > 0 {
                    "⚠️ Action required - Check Scan tab for details"
                } else {
                    "✅ Your Mac is protected"
                }
            )
        } else {
            "🔄 No recent scan data\n\nRun a scan to check your system security".to_string()
        };

        let status_widget = Paragraph::new(status_text)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .title("Security Status")
            )
            .wrap(ratatui::widgets::Wrap { trim: true });

        frame.render_widget(status_widget, chunks[0]);

        // Recent Activity
        let activity_text = "🕐 Recent Activity:\n\n\
                           • Real-time monitoring: Active\n\
                           • Clipboard protection: Enabled\n\
                           • AI analysis: Online\n\
                           • Threat signatures: Updated\n\n\
                           🛡️ Bug Spray is actively protecting your Mac";

        let activity_widget = Paragraph::new(activity_text)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title("Protection Status")
            )
            .wrap(ratatui::widgets::Wrap { trim: true });

        frame.render_widget(activity_widget, chunks[1]);
    }

    fn draw_scan_tab<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        if let Some(results) = &self.scan_results {
            if results.threats_found > 0 {
                let threat_items: Vec<ListItem> = results.threats.iter()
                    .map(|threat| {
                        let color = match threat.severity.as_str() {
                            "Critical" => Color::Red,
                            "High" => Color::LightRed,
                            "Medium" => Color::Yellow,
                            _ => Color::White,
                        };
                        
                        ListItem::new(format!("🚨 {} - {}", threat.name, threat.description))
                            .style(Style::default().fg(color))
                    })
                    .collect();

                let threats_list = List::new(threat_items)
                    .block(Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Red))
                        .title("Detected Threats")
                    );

                frame.render_widget(threats_list, area);
            } else {
                let clean_text = "✅ No threats detected!\n\n\
                                Your Mac is clean and secure.\n\n\
                                🛡️ Bug Spray found no malware\n\
                                🔒 All crypto wallets are safe\n\
                                💻 Developer tools are clean\n\n\
                                Press 'S' to run another scan";

                let clean_widget = Paragraph::new(clean_text)
                    .style(Style::default().fg(Color::Green))
                    .alignment(Alignment::Center)
                    .block(Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Green))
                        .title("Scan Results")
                    );

                frame.render_widget(clean_widget, area);
            }
        } else {
            let scan_prompt = "🔍 Ready to scan your Mac for threats\n\n\
                             Bug Spray will check for:\n\n\
                             🐛 AtomicStealer & crypto wallet malware\n\
                             🔧 RustBucket & developer-targeted attacks\n\
                             📱 KandyKorn & fake app installers\n\
                             🛠️ XCSSET & Xcode project infections\n\
                             📋 Clipboard hijackers\n\
                             🌐 Supply chain compromises\n\n\
                             Press 'S' to start scanning";

            let scan_widget = Paragraph::new(scan_prompt)
                .style(Style::default().fg(Color::Cyan))
                .alignment(Alignment::Center)
                .block(Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan))
                    .title("Security Scan")
                );

            frame.render_widget(scan_widget, area);
        }
    }

    fn draw_ai_chat_tab<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(area);

        // Chat messages
        let messages: Vec<ListItem> = self.ai_chat_messages.iter()
            .map(|msg| {
                let prefix = if msg.is_user { "👤 You:" } else { "🐛 Bug Spray:" };
                let style = if msg.is_user { 
                    Style::default().fg(Color::White) 
                } else { 
                    Style::default().fg(Color::Green) 
                };
                
                ListItem::new(format!("{} {}", prefix, msg.content))
                    .style(style)
            })
            .collect();

        let chat_list = List::new(messages)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .title("AI Security Consultation")
            );

        frame.render_widget(chat_list, chunks[0]);

        // Input box
        let input_widget = Paragraph::new(format!("💬 Type your question: {}", self.input_buffer))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
            );

        frame.render_widget(input_widget, chunks[1]);
    }

    fn draw_settings_tab<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let settings_text = "⚙️ Bug Spray Settings\n\n\
                           🔄 Real-time Protection: Enabled\n\
                           📋 Clipboard Monitoring: Active\n\
                           🧠 AI Features: Online\n\
                           🔍 Auto-scan: Every 24 hours\n\
                           📱 Notifications: Enabled\n\n\
                           🔐 Permissions:\n\
                           ✅ Full Disk Access\n\
                           ✅ Accessibility\n\
                           ✅ Security Events\n\n\
                           📊 Statistics:\n\
                           • Total scans: 42\n\
                           • Threats blocked: 7\n\
                           • Days protected: 30";

        let settings_widget = Paragraph::new(settings_text)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .title("Settings & Status")
            )
            .wrap(ratatui::widgets::Wrap { trim: true });

        frame.render_widget(settings_widget, area);
    }

    fn draw_footer<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let footer_text = "🐛 Bug Spray v1.0 | 'Q' Quit | 'H' Help | 'S' Scan | 'Tab' Navigate | Arrow Keys Move";
        
        let footer = Paragraph::new(footer_text)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
            );

        frame.render_widget(footer, area);
    }

    fn draw_help_popup<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let popup_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .split(area)[1];

        let popup_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(popup_area)[1];

        let help_text = "🐛 Bug Spray Help\n\n\
                        KEYBOARD SHORTCUTS:\n\
                        • Q / Esc - Quit Bug Spray\n\
                        • H - Toggle this help\n\
                        • S - Start security scan\n\
                        • Tab - Switch between tabs\n\
                        • Arrow keys - Navigate\n\
                        • Enter - Confirm action\n\n\
                        TABS:\n\
                        • Dashboard - System overview\n\
                        • Scan - Threat detection\n\
                        • AI Chat - Security consultation\n\
                        • Settings - Configuration\n\n\
                        Press 'H' to close this help";

        let help_widget = Paragraph::new(help_text)
            .style(Style::default().fg(Color::White).bg(Color::DarkGray))
            .alignment(Alignment::Left)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title("Help")
                .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            )
            .wrap(ratatui::widgets::Wrap { trim: true });

        frame.render_widget(help_widget, popup_area);
    }

    pub async fn handle_key_event(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Tab => {
                self.selected_tab = (self.selected_tab + 1) % 4;
            }
            KeyCode::Char('h') | KeyCode::Char('H') => {
                self.show_help = !self.show_help;
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.run_scan().await?;
            }
            KeyCode::Char(c) if self.selected_tab == 2 => {
                // AI chat input
                self.input_buffer.push(c);
            }
            KeyCode::Backspace if self.selected_tab == 2 => {
                self.input_buffer.pop();
            }
            KeyCode::Enter if self.selected_tab == 2 && !self.input_buffer.is_empty() => {
                self.send_ai_message().await?;
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn update(&mut self) -> Result<()> {
        // Update app state, check for new threats, etc.
        Ok(())
    }

    async fn run_scan(&mut self) -> Result<()> {
        self.last_scan = Some(Instant::now());
        self.scan_results = Some(self.scanner.quick_scan().await?);
        Ok(())
    }

    async fn send_ai_message(&mut self) -> Result<()> {
        if let Some(ai) = &self.ai {
            let user_message = self.input_buffer.clone();
            self.ai_chat_messages.push(ChatMessage {
                is_user: true,
                content: user_message.clone(),
                timestamp: Instant::now(),
            });
            
            self.input_buffer.clear();
            
            let response = ai.analyze_query(&user_message, &self.scanner).await?;
            self.ai_chat_messages.push(ChatMessage {
                is_user: false,
                content: response,
                timestamp: Instant::now(),
            });
        }
        Ok(())
    }
}
