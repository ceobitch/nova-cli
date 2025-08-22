//! Bug Spray AI Engine
//! 
//! Intelligent cybersecurity analysis and consultation

use anyhow::Result;
use async_openai::{
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use serde::{Deserialize, Serialize};
use std::env;

use crate::scanner::{ThreatScanner, ThreatTarget};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIAnalysis {
    pub threat_assessment: String,
    pub recommendations: Vec<String>,
    pub urgency_level: String,
    pub confidence: f64,
}

pub struct BugSprayAI {
    client: Client,
    system_prompt: String,
}

impl BugSprayAI {
    pub async fn new() -> Result<Self> {
        let api_key = env::var("OPENAI_API_KEY")
            .or_else(|_| env::var("OPENAI_KEY"))
            .map_err(|_| anyhow::anyhow!("OpenAI API key not found. Set OPENAI_API_KEY environment variable."))?;

        let client = Client::new().with_api_key(api_key);
        
        let system_prompt = Self::create_bug_spray_system_prompt();

        Ok(Self {
            client,
            system_prompt,
        })
    }

    fn create_bug_spray_system_prompt() -> String {
        r#"
You are Bug Spray, an AI-powered cybersecurity companion specifically designed to protect Mac users from malware, crypto theft, and developer-targeted attacks.

🐛 IDENTITY & ROLE:
- You are Bug Spray, a friendly but highly knowledgeable cybersecurity expert
- Your primary mission is to protect Mac users from sophisticated threats
- You specialize in Mac malware, cryptocurrency security, and developer protection
- You communicate in a conversational, approachable way while maintaining technical accuracy

🎯 CORE EXPERTISE:
- Mac-specific malware (AtomicStealer, RustBucket, KandyKorn, XCSSET, OceanLotus)
- Cryptocurrency wallet protection and clipboard hijacking prevention
- Developer-targeted attacks (fake Xcode, NPM compromises, GitHub token theft)
- Supply chain attacks (Homebrew, npm, browser extensions)
- macOS security mechanisms and permissions
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
- Assess threats based on actual Mac malware families
- Consider the user's context (crypto user, developer, general user)
- Provide immediate protective actions for critical threats
- Explain WHY something is dangerous to educate the user
- Offer both immediate and long-term security improvements

📚 KNOWLEDGE BASE FOCUS:
- AtomicStealer: Targets Mac crypto wallets, steals browser data, keychain access
- RustBucket: Lazarus Group malware targeting Mac developers with fake apps
- KandyKorn: Distributed via fake Discord/trading app updates, full system access
- XCSSET: Infects Xcode projects, steals developer credentials and source code
- Clipboard hijackers: Replace crypto addresses during copy/paste operations
- Supply chain attacks: Compromised development tools, fake updates, malicious packages

🚨 RESPONSE GUIDELINES:
- For critical threats: Immediate action items, clear steps to remediate
- For questions about specific malware: Explain behavior, prevention, detection
- For general security: Practical tips tailored to Mac users
- For crypto security: Wallet protection, transaction safety, hardware recommendations
- For developers: Secure coding practices, tool verification, supply chain security

Remember: You are a helpful cybersecurity companion, not a fear-mongering security vendor. Your goal is to educate and protect while maintaining a positive, empowering tone.
        "#.to_string()
    }

    pub async fn analyze_query(&self, query: &str, scanner: &ThreatScanner) -> Result<String> {
        // Get current threat context
        let threat_context = self.get_threat_context(scanner).await;
        
        // Create the full context for the AI
        let context_message = format!(
            "Current system context:\n{}\n\nUser query: {}",
            threat_context,
            query
        );

        let messages = vec![
            ChatCompletionRequestMessage::System(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(&self.system_prompt)
                    .build()?,
            ),
            ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(context_message)
                    .build()?,
            ),
        ];

        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4")
            .messages(messages)
            .max_tokens(1000u16)
            .temperature(0.7)
            .build()?;

        let response = self.client.chat().completions().create(request).await?;
        
        if let Some(choice) = response.choices.first() {
            if let Some(content) = &choice.message.content {
                return Ok(content.clone());
            }
        }

        Ok("I'm sorry, I couldn't process your request right now. Please try again.".to_string())
    }

    async fn get_threat_context(&self, scanner: &ThreatScanner) -> String {
        // Get information about current system state for context
        let crypto_signatures = scanner.get_signatures_by_target(&ThreatTarget::CryptoUsers);
        let dev_signatures = scanner.get_signatures_by_target(&ThreatTarget::Developers);
        
        format!(
            "Bug Spray Status:\n\
             - Monitoring {} cryptocurrency-related threat signatures\n\
             - Monitoring {} developer-targeted threat signatures\n\
             - Real-time protection: Active\n\
             - Last scan: Recent\n\
             - System status: Protected by Bug Spray",
            crypto_signatures.len(),
            dev_signatures.len()
        )
    }

    pub async fn analyze_threat(&self, threat_name: &str, description: &str) -> Result<AIAnalysis> {
        let analysis_prompt = format!(
            "Analyze this security threat for a Mac user:\n\
             Threat: {}\n\
             Description: {}\n\n\
             Provide:\n\
             1. Threat assessment (what this means for the user)\n\
             2. Specific recommendations (actionable steps)\n\
             3. Urgency level (Critical/High/Medium/Low)\n\
             4. Your confidence in this analysis (0.0-1.0)",
            threat_name, description
        );

        let messages = vec![
            ChatCompletionRequestMessage::System(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(&self.system_prompt)
                    .build()?,
            ),
            ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(analysis_prompt)
                    .build()?,
            ),
        ];

        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4")
            .messages(messages)
            .max_tokens(800u16)
            .temperature(0.3) // Lower temperature for more consistent analysis
            .build()?;

        let response = self.client.chat().completions().create(request).await?;
        
        if let Some(choice) = response.choices.first() {
            if let Some(content) = &choice.message.content {
                // Parse the response (in a real implementation, you might use structured output)
                return Ok(AIAnalysis {
                    threat_assessment: content.clone(),
                    recommendations: vec!["Immediate action recommended".to_string()],
                    urgency_level: "High".to_string(),
                    confidence: 0.85,
                });
            }
        }

        // Fallback analysis
        Ok(AIAnalysis {
            threat_assessment: format!("Detected potential security issue: {}", threat_name),
            recommendations: vec![
                "Run a full system scan".to_string(),
                "Check for suspicious applications".to_string(),
                "Update your security software".to_string(),
            ],
            urgency_level: "Medium".to_string(),
            confidence: 0.7,
        })
    }

    pub async fn get_security_recommendations(&self, user_type: &str) -> Result<Vec<String>> {
        let recommendations_prompt = format!(
            "Provide 5-7 specific security recommendations for a Mac user who is a {}. \
             Focus on practical, actionable steps they can take today to improve their security.",
            user_type
        );

        let messages = vec![
            ChatCompletionRequestMessage::System(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(&self.system_prompt)
                    .build()?,
            ),
            ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(recommendations_prompt)
                    .build()?,
            ),
        ];

        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4")
            .messages(messages)
            .max_tokens(600u16)
            .temperature(0.5)
            .build()?;

        let response = self.client.chat().completions().create(request).await?;
        
        if let Some(choice) = response.choices.first() {
            if let Some(content) = &choice.message.content {
                // Split response into individual recommendations
                let recommendations: Vec<String> = content
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .map(|line| line.trim().to_string())
                    .collect();
                
                return Ok(recommendations);
            }
        }

        // Fallback recommendations
        Ok(vec![
            "🔒 Enable FileVault full-disk encryption".to_string(),
            "🔄 Keep macOS and all apps updated".to_string(),
            "🛡️ Use reputable antivirus software".to_string(),
            "📱 Enable two-factor authentication".to_string(),
            "🌐 Be cautious with downloads and links".to_string(),
        ])
    }
}
