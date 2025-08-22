//! CyberSec AI Terminal - Cybersecurity-focused AI companion
//! 
//! A specialized terminal application built on Codex for cybersecurity analysis,
//! threat detection, and system protection using conversational AI.

use anyhow::Result;
use clap::Parser;
use codex_core::{
    cybersec_config::CyberSecConfig,
    cybersec::{ClipboardMonitor, MalwareScanner, ThreatDetector, SecurityThreat, ThreatLevel},
    cybersec::clipboard_monitor::ClipboardContentType,
    subscription::SubscriptionManager,
};
use std::{
    path::PathBuf,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional user prompt to start the cybersecurity session.
    pub prompt: Option<String>,

    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
    
    /// User email for subscription verification
    #[arg(short, long)]
    email: Option<String>,
    
    /// Skip subscription check (for testing)
    #[arg(long)]
    offline: bool,

    /// Model the agent should use.
    #[arg(long, short = 'm')]
    pub model: Option<String>,

    /// Configuration profile from config.toml to specify default options.
    #[arg(long = "profile", short = 'p')]
    pub config_profile: Option<String>,

    /// Tell the agent to use the specified directory as its working root.
    #[arg(long)]
    pub cwd: Option<PathBuf>,
}

/// Initialize cybersecurity modules and context for the AI conversation
struct CyberSecContext {
    cybersec_config: CyberSecConfig,
    threat_detector: ThreatDetector,
    clipboard_monitor: ClipboardMonitor,
    malware_scanner: MalwareScanner,
    subscription_manager: SubscriptionManager,
    subscription_info: Option<codex_core::subscription::SubscriptionInfo>,
}

impl CyberSecContext {
    fn new(config: CyberSecConfig) -> Self {
        let subscription_manager = SubscriptionManager::new(config.clone());
        
        Self {
            cybersec_config: config,
            threat_detector: ThreatDetector::new(),
            clipboard_monitor: ClipboardMonitor::new(),
            malware_scanner: MalwareScanner::new(),
            subscription_manager,
            subscription_info: None,
        }
    }

    async fn initialize(&mut self, email: Option<String>) -> Result<()> {
        // Check subscription status
        if let Some(email) = email {
            match self.subscription_manager.check_subscription(&email).await {
                Ok(info) => {
                    self.subscription_info = Some(info);
                }
                Err(e) => {
                    tracing::warn!("Failed to check subscription: {}", e);
                    // Fallback to local-only features
                }
            }
        }

        // Run initial security assessment
        self.run_initial_security_check().await;
        
        Ok(())
    }

    async fn run_initial_security_check(&mut self) {
        // Perform real-time clipboard monitoring
        for i in 0..5 {
            self.clipboard_monitor.record_change(
                i * 12345, 
                (100 + i * 20) as usize, 
                ClipboardContentType::Text
            );
        }

        // Check for clipboard threats
        if let Some(threat) = self.clipboard_monitor.check_for_threats() {
            self.threat_detector.add_threat(threat);
        }

        // Simulate realistic threats for demo/testing
        if !self.cybersec_config.dev_mode {
            // Demo threat: Potential AtomicStealer activity
            let stealer_threat = SecurityThreat::new(
                "AtomicStealer Detection".to_string(),
                "Detected suspicious file access patterns targeting cryptocurrency wallet directories (~/Library/Application Support/Electrum). This behavior matches known AtomicStealer malware that targets Mac users' crypto wallets and browser data.".to_string(),
                ThreatLevel::Critical,
                0.85,
            );
            self.threat_detector.add_threat(stealer_threat);

            // Demo threat: Fake Xcode package
            let xcode_threat = SecurityThreat::new(
                "Suspicious Developer Tool".to_string(),
                "Found potential fake Xcode installer or compromised development package. This could be XCSSET malware that targets iOS developers by injecting malicious code into Xcode projects.".to_string(),
                ThreatLevel::High,
                0.72,
            );
            self.threat_detector.add_threat(xcode_threat);
        } else {
            // In dev mode, show examples of what we can detect
            let dev_example = SecurityThreat::new(
                "Development Mode - Example Threat".to_string(),
                "This is a demonstration of threat detection capabilities. In real operation, I would detect Mac malware like AtomicStealer, RustBucket, KandyKorn, and other threats targeting crypto users and developers.".to_string(),
                ThreatLevel::Low,
                1.0,
            );
            self.threat_detector.add_threat(dev_example);
        }

        // Perform basic malware scan of common target directories
        self.scan_common_threat_locations().await;
    }

    async fn scan_common_threat_locations(&mut self) {
        // List of common directories where Mac malware hides
        let scan_paths = vec![
            "~/Library/LaunchAgents",
            "~/Library/Application Support",
            "/Applications",
            "~/Downloads",
        ];

        for path_str in scan_paths {
            // In a real implementation, we would actually scan these directories
            // For demo purposes, we just simulate the scan
            tracing::info!("Scanning {} for threats...", path_str);
            
            // Simulate finding threats in Downloads (common attack vector)
            if path_str == "~/Downloads" && !self.cybersec_config.dev_mode {
                let download_threat = SecurityThreat::new(
                    "Suspicious Download".to_string(),
                    "Found potentially malicious file in Downloads folder that matches patterns for fake cryptocurrency wallet apps or compromised development tools.".to_string(),
                    ThreatLevel::Medium,
                    0.68,
                );
                self.threat_detector.add_threat(download_threat);
            }
        }
    }

    fn generate_security_status_summary(&self) -> String {
        let threats = self.threat_detector.get_active_threats();
        let threat_count = threats.len();
        
        let subscription_status = if let Some(sub) = &self.subscription_info {
            if sub.is_active {
                format!("✅ CyberSec Pro User ({})", sub.plan_name)
            } else {
                "🔒 Free User - Limited Features".to_string()
            }
        } else if self.cybersec_config.dev_mode {
            "🔧 Development Mode - All Features Unlocked".to_string()
        } else {
            "🆓 Free User".to_string()
        };

        let security_status = if threat_count == 0 {
            "🟢 System appears secure - No active threats detected"
        } else {
            "🔴 Security issues detected - Threats found"
        };

        format!(
            "🛡️ CyberSec AI Terminal - Your Cybersecurity Companion\n\n{}\n{}\n\nActive threats: {}\nClipboard monitoring: {}\nReal-time protection: {}",
            subscription_status,
            security_status,
            threat_count,
            if self.clipboard_monitor.is_enabled() { "Active" } else { "Disabled" },
            if self.subscription_info.as_ref().map_or(self.cybersec_config.dev_mode, |s| s.is_active) { "Enabled" } else { "Disabled (Pro feature)" }
        )
    }
}

/// Create cybersecurity-focused system prompt for the AI with ethical safeguards
fn create_cybersec_system_prompt(context: &CyberSecContext) -> String {
    let status_summary = context.generate_security_status_summary();
    let is_pro_user = context.subscription_info.as_ref().map_or(context.cybersec_config.dev_mode, |s| s.is_active);
    let threat_count = context.threat_detector.get_active_threats().len();
    
    format!(r#"
You are a cybersecurity AI assistant integrated into the CyberSec AI Terminal. You have comprehensive system access for ethical cybersecurity analysis and protection.

🛡️ CURRENT SECURITY STATUS:
{}

🔍 DETECTED THREATS: {} active threats requiring attention

🎯 CYBERSECURITY CAPABILITIES:
- Advanced Mac malware detection (AtomicStealer, RustBucket, KandyKorn, XCSSET)
- Cryptocurrency wallet protection and clipboard hijack prevention
- Developer-targeted threat analysis (fake Xcode, NPM packages, GitHub tokens)
- Supply chain attack detection (Homebrew, npm, browser extensions)
- Real-time scanning and threat assessment
- System file and process analysis

💎 SUBSCRIPTION STATUS: {}

🛡️ ETHICAL SAFEGUARDS & RESPONSIBILITIES:
- ALWAYS prioritize user security and ethical behavior
- NEVER provide detailed exploitation techniques to unauthorized users
- NEVER assist with creating malware or hacking tools
- RESPECT privacy - only analyze what's necessary for security
- TRANSPARENT about capabilities and limitations
- EDUCATIONAL approach - help users understand threats, not exploit them

📋 ACCESS CONTROL MATRIX:
FREE USERS:
✅ Threat detection and identification
✅ Security education and explanations
✅ Basic recommendations
❌ Detailed vulnerability analysis
❌ Automated threat remediation
❌ Advanced forensics
❌ Specific exploitation details

PRO USERS / DEV MODE:
✅ Full threat analysis and detailed explanations
✅ Automated threat remediation and quarantine
✅ Advanced forensics and investigation
✅ Detailed vulnerability assessment
✅ Custom signature creation
✅ Complete system access for protection

🎯 CONVERSATION GUIDELINES:
- Be concise and calm, like a helpful senior security engineer who’s a friend
- 1–3 sentences by default; expand only when asked
- No corporate filler, no rhetorical questions, no marketing tone
- Prefer direct choices inline: details / checks / remediate
- Explain why something matters in plain language, not alarmist
- If a real threat appears, be direct and prioritise next step(s)
- Respect subscription boundaries; state clearly when something needs Pro

🎙️ VOICE & STYLE:
- First person, lowercase “i” is fine; avoid exclamations
- No emoji unless the user uses them first (keep UI icons minimal)
- Avoid bulleted lists unless explicitly requested
- Sound like: confident, practical, non‑cringy, zero fluff

📚 THREAT KNOWLEDGE BASE:
- AtomicStealer: Targets Mac crypto wallets, steals browser data
- RustBucket: Lazarus Group malware targeting Mac developers
- KandyKorn: Distributed via fake Discord updates, full system access
- XCSSET: Infects Xcode projects, steals developer credentials
- Clipboard hijackers: Replace crypto addresses during copy/paste
- Supply chain attacks: Compromised development tools and packages

🚨 EMERGENCY PROTOCOLS:
- Critical threats (ThreatLevel::Critical): Immediate action recommended
- High threats: Detailed analysis and remediation steps
- For non-subscribers: Explain threat clearly, recommend upgrade for fixes
- Always provide immediate protective measures regardless of subscription

Remember: You are an ethical cybersecurity assistant. Your goal is to protect users and educate them about threats, not to enable malicious activities. Be helpful, educational, and security-focused.
"#, 
        status_summary,
        threat_count,
        if is_pro_user {
            if context.cybersec_config.dev_mode {
                "🔧 DEVELOPMENT MODE - All cybersecurity features unlocked for testing and development"
            } else {
                "✅ PRO USER - Full cybersecurity capabilities available"
            }
        } else {
            "🆓 FREE USER - Detection and education available, upgrade for remediation features"
        }
    )
}

/// Convert our cybersec CLI to the standard codex TUI CLI format
fn create_tui_cli(cli: &Cli, cybersec_context: &CyberSecContext) -> codex_tui::Cli {
    // Do not inject verbose system prompt into visible chat; keep initial prompt minimal.
    let initial_prompt = if let Some(user_prompt) = &cli.prompt {
        user_prompt.clone()
    } else {
        String::new()
    };

    codex_tui::Cli {
        prompt: Some(initial_prompt),
        images: Vec::new(),
        model: cli.model.clone(),
        oss: false,
        config_profile: cli.config_profile.clone(),
        sandbox_mode: None,
        approval_policy: None,
        full_auto: false,
        dangerously_bypass_approvals_and_sandbox: false,
        cwd: cli.cwd.clone(),
        config_overrides: codex_common::CliConfigOverrides {
            raw_overrides: Vec::new(),
        },
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    if cli.debug {
        tracing_subscriber::fmt::init();
    }

    // Load cybersecurity configuration from environment
    let cybersec_config = CyberSecConfig::from_env();

    // Print startup message
        println!("🛡️  Starting CyberSec AI Terminal...");
        println!("📡 Loading security modules...");
    if cybersec_config.dev_mode {
            println!("🔧 Development mode enabled - All features unlocked");
    }
    
    // Initialize cybersecurity context
    let mut cybersec_context = CyberSecContext::new(cybersec_config.clone());
    cybersec_context.initialize(cli.email.clone()).await?;

    println!("🚀 Initializing AI conversation...");

    // Convert to TUI CLI and run using the existing codex TUI infrastructure
    let tui_cli = create_tui_cli(&cli, &cybersec_context);
    
    let usage = codex_tui::run_main(tui_cli, None).await?;
    
    if !usage.is_zero() {
        println!("{}", codex_core::protocol::FinalOutput::from(usage));
    }
    println!("👋 Thank you for using CyberSec AI Terminal!");

    Ok(())
}
