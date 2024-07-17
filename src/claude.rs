//! This module handles interactions with the Claude AI API.

use worker::*;
use reqwest::Client;
use crate::types::{TimestampedChatMessage, ClaudeRequest, ClaudeResponse, ClaudeMessage};

/// Formats a conversation for sending to the Claude API.
///
/// # Arguments
///
/// * `conversation` - A vector of tuples representing the conversation history.
///                    Each tuple contains a role ("user" or "assistant") and a message.
///
/// # Returns
///
/// A vector of `ClaudeMessage` structs formatted for the API request.
pub fn format_conversation(conversation: Vec<(&str, &str)>) -> Vec<ClaudeMessage> {
    conversation.into_iter()
        .map(|(role, content)| ClaudeMessage {
            role: role.to_string(),
            content: content.to_string(),
            name: None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_conversation() {
        let conversation = vec![
            ("user", "Hello, Claude!"),
            ("assistant", "Hello! How can I assist you today?"),
            ("user", "Tell me about Rust programming."),
        ];

        let formatted = format_conversation(conversation);

        assert_eq!(formatted.len(), 3);
        assert_eq!(formatted[0].role, "user");
        assert_eq!(formatted[0].content, "Hello, Claude!");
        assert_eq!(formatted[1].role, "assistant");
        assert_eq!(formatted[1].content, "Hello! How can I assist you today?");
        assert_eq!(formatted[2].role, "user");
        assert_eq!(formatted[2].content, "Tell me about Rust programming.");
    }
}

/// Calls the Claude API with a given conversation history.
///
/// # Arguments
///
/// * `conversation` - The conversation history to send to Claude
/// * `api_key` - The API key for authentication with the Claude API
/// * `topic` - The current DevOps topic being discussed
///
/// # Returns
///
/// A `Result<String>` containing the AI's response text or an error.
pub async fn call_claude_api_with_history(conversation: &[TimestampedChatMessage], api_key: &str, topic: &str) -> Result<String> {
    let client = Client::new();
    let url = "https://api.anthropic.com/v1/messages";

    let claude_messages: Vec<ClaudeMessage> = conversation.iter().map(|msg| ClaudeMessage {
        role: msg.role.clone(),
        content: msg.content.clone(),
        name: None,
    }).collect();

    let system_prompt = format!(
        "You are an AI assistant for a DevOps learning platform. Your role is to provide expert guidance and assistance on DevOps topics including:

    - Version control with Git
    - Continuous Integration and Continuous Delivery (CI/CD)
    - Container technologies like Docker
    - Container orchestration with Kubernetes
    - Infrastructure as Code (IaC)
    - Cloud platforms and services
    - Monitoring and observability
    - DevOps best practices and methodologies

    Respond to user queries with accurate, up-to-date information on these topics. Provide explanations, examples, and step-by-step instructions when appropriate. If asked about a specific tool or technology, include details on its purpose, key features, and common use cases in DevOps workflows.

    Important guidelines:

    1. Only answer questions related to DevOps topics. If a user asks about an unrelated subject, politely redirect them to DevOps-relevant questions.

    2. Do not provide any information on bypassing security measures, hacking, or unauthorized system access.

    3. If asked to perform actions outside your capabilities (e.g. executing code, accessing external systems), explain that you're a text-based assistant focused on providing DevOps knowledge.

    4. Do not share personal information about real individuals or disclose sensitive details about specific organizations.

    5. If unsure about an answer, acknowledge your uncertainty and suggest reliable resources for further information.

    6. Encourage best practices for security, scalability, and efficiency in DevOps processes.

    7. When discussing tools or platforms, maintain a neutral stance and focus on technical merits rather than promoting specific products.

    Your goal is to help users learn and apply DevOps concepts effectively while maintaining a secure and focused learning environment.

    The current topic of discussion is: {}",
        topic
    );
    let claude_request = ClaudeRequest {
        model: "claude-3-5-sonnet-20240620".to_string(),
        max_tokens: 1024,
        messages: claude_messages,
        system: Some(system_prompt),
    };

    let response = match client.post(url)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&claude_request)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => return Err(Error::from(format!("Failed to send request: {}", e))),
    };

    if !response.status().is_success() {
        return Err(Error::from(format!("API request failed: {}", response.status())));
    }

    let claude_response: ClaudeResponse = match response.json().await {
        Ok(resp) => resp,
        Err(e) => return Err(Error::from(format!("Failed to parse API response: {}", e))),
    };
    
    claude_response.content.first()
        .map(|content| content.text.clone())
        .ok_or_else(|| Error::from("No content in API response"))
}
