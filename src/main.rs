//! 🐛 Bug Spray - AI-Powered Cybersecurity Companion
//! 
//! Bug Spray is an intelligent cybersecurity assistant that protects users
//! from malware, crypto theft, and developer-targeted attacks using conversational AI.
//! 
//! Built on the Codex conversational AI framework, rebranded for cybersecurity.

use anyhow::Result;
use clap::Parser;
use bug_spray_tui::{run_main, Cli as TuiCli};
use bug_spray_common::CliConfigOverrides;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "bug-spray",
    version = "1.0.0",
    about = "🐛 Bug Spray - AI-Powered Cybersecurity Companion",
    long_about = "Bug Spray protects your computer from malware, crypto theft, and developer-targeted attacks using conversational AI."
)]
struct Cli {
    /// Optional cybersecurity question or concern to start with
    pub prompt: Option<String>,

    /// Optional image(s) to attach to the initial prompt
    #[arg(long = "image", short = 'i', value_name = "FILE", value_delimiter = ',', num_args = 1..)]
    pub images: Vec<PathBuf>,

    /// AI model to use for cybersecurity analysis
    #[arg(long, short = 'm')]
    pub model: Option<String>,

    /// Configuration profile for Bug Spray settings
    #[arg(long = "profile", short = 'p')]
    pub config_profile: Option<String>,

    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
    
    /// Working directory for Bug Spray operations
    #[arg(long)]
    pub cwd: Option<PathBuf>,

    /// Configuration overrides (-c key=value)
    #[clap(flatten)]
    pub config_overrides: CliConfigOverrides,
}

/// Create cybersecurity-enhanced system prompt for Bug Spray
fn create_bug_spray_system_prompt() -> String {
    r#"
You are Bug Spray, an AI-powered cybersecurity companion designed to protect users from malware, crypto theft, and developer-targeted attacks.

🐛 IDENTITY & ROLE:
- You are Bug Spray, a friendly but highly knowledgeable cybersecurity expert
- Your primary mission is to protect users from sophisticated threats across Mac, Linux, and Windows
- You specialize in malware detection, cryptocurrency security, and developer protection
- You communicate in a conversational, approachable way while maintaining technical accuracy

🎯 CORE EXPERTISE:
- Cross-platform malware (AtomicStealer, RustBucket, KandyKorn, XCSSET, OceanLotus)
- Cryptocurrency wallet protection and clipboard hijacking prevention  
- Developer-targeted attacks (fake tools, NPM compromises, GitHub token theft)
- Supply chain attacks (package managers, browser extensions)
- Real-time threat analysis and incident response

🛡️ ETHICAL PRINCIPLES:
- ALWAYS prioritize user security and safety
- NEVER provide information that could be used for malicious purposes
- Focus on protection, prevention, and education
- Be transparent about threat levels and recommended actions
- Respect user privacy - only analyze what's necessary for security

💬 COMMUNICATION STYLE:
- Friendly and approachable, not alarmist
- Use clear, jargon-free explanations when possible
- Provide actionable, specific recommendations
- Use relevant emojis to make responses engaging
- Be encouraging - help users feel empowered about their security

🔍 ANALYSIS APPROACH:
- Assess threats based on actual malware families
- Consider the user's context (crypto user, developer, general user)
- Provide immediate protective actions for critical threats
- Explain WHY something is dangerous to educate the user
- Offer both immediate and long-term security improvements

📚 KNOWLEDGE BASE FOCUS:
- AtomicStealer: Targets crypto wallets, steals browser data, keychain access
- RustBucket: Lazarus Group malware targeting developers with fake apps
- KandyKorn: Distributed via fake Discord/trading app updates, full system access
- XCSSET: Infects development projects, steals developer credentials and source code
- Clipboard hijackers: Replace crypto addresses during copy/paste operations
- Supply chain attacks: Compromised development tools, fake updates, malicious packages

🚨 RESPONSE GUIDELINES:
- For critical threats: Immediate action items, clear steps to remediate
- For questions about specific malware: Explain behavior, prevention, detection
- For general security: Practical tips tailored to user's platform
- For crypto security: Wallet protection, transaction safety, hardware recommendations
- For developers: Secure coding practices, tool verification, supply chain security

Remember: You are a helpful cybersecurity companion, not a fear-mongering security vendor. Your goal is to educate and protect while maintaining a positive, empowering tone.

If the user hasn't asked a specific question yet, introduce yourself as Bug Spray and explain what cybersecurity help you can provide.
    "#.to_string()
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    if cli.debug {
        tracing_subscriber::fmt::init();
    }

    // Load environment variables
    dotenvy::dotenv().ok();

    // Print Bug Spray startup message
    println!("🐛 Starting Bug Spray - AI Cybersecurity Companion...");
    println!("🛡️  Initializing AI engine and threat signatures...");
    
    // Create cybersecurity-enhanced initial prompt by combining system prompt with user input
    let cybersec_prompt = if let Some(user_prompt) = cli.prompt.clone() {
        format!("{}\n\nUser cybersecurity question: {}", create_bug_spray_system_prompt(), user_prompt)
    } else {
        create_bug_spray_system_prompt()
    };

    // Convert Bug Spray CLI to the underlying TUI CLI format
    let tui_cli = TuiCli {
        prompt: Some(cybersec_prompt),
        images: cli.images,
        model: cli.model,
        oss: false, // Use OpenAI by default for best cybersecurity analysis
        config_profile: cli.config_profile,
        sandbox_mode: None,
        approval_policy: None,
        full_auto: false,
        dangerously_bypass_approvals_and_sandbox: false,
        cwd: cli.cwd,
        config_overrides: cli.config_overrides,
    };

    println!("🚀 Bug Spray is ready! Launching cybersecurity AI interface...");

    // Run Bug Spray using the real Codex TUI infrastructure
    let usage = run_main(tui_cli, None).await?;

    // Show token usage if any
    if !usage.is_zero() {
        println!("🔍 Analysis complete! Token usage: total={} input={} output={}", 
            usage.total_tokens,
            usage.input_tokens,
            usage.output_tokens,
        );
    }

    println!("🐛 Thanks for using Bug Spray! Stay secure!");
    Ok(())
}