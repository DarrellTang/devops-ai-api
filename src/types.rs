//! This module contains all the data structures used in the DevOps AI API.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Represents a learning topic in the DevOps AI system.
#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    /// Unique identifier for the topic
    pub id: String,
    /// Title of the topic
    pub title: String,
    /// Brief description of the topic
    pub description: String,
    /// List of steps involved in the topic
    pub steps: Vec<Step>,
    /// Initial message to be displayed when the topic is started
    pub initial_message: String,
}

/// Represents a single step within a learning topic.
#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    /// Title of the step
    pub title: String,
    /// Prompt to be sent to the AI for this step
    pub prompt: String,
    /// Suggested questions for this step
    pub suggested_questions: Vec<String>,
}

/// Represents a generic response structure for API calls.
#[derive(Debug, Deserialize, Serialize)]
pub struct GenericResponse {
    /// HTTP status code
    pub status: u16,
    /// Response message
    pub message: String,
}

/// Represents an update to the user's progress on a topic.
#[derive(Debug, Deserialize)]
pub struct ProgressUpdate {
    /// The step that was completed
    pub completed_step: usize,
    /// Optional flag to reset progress
    pub reset: Option<bool>,
}

/// Represents the overall progress of a user on a specific topic.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Progress {
    /// The ID of the topic
    pub topic_id: String,
    /// List of completed step indices
    pub completed_steps: Vec<usize>,
    /// The current step the user is on
    pub current_step: usize,
}

/// Represents the overall conversation history for a specific topic.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationHistory {
    /// The ID of the topic this conversation is associated with
    pub topic_id: String,
    /// List of messages in the conversation
    pub messages: Vec<TimestampedChatMessage>,
}

/// Represents a single message in the conversation, with a timestamp.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimestampedChatMessage {
    /// The role of the message sender (e.g., "user" or "assistant")
    pub role: String,
    /// The content of the message
    pub content: String,
    /// The timestamp when the message was sent or received
    pub timestamp: DateTime<Utc>,
}

/// Represents a chat message sent by the user.
#[derive(Debug, Deserialize)]
pub struct ChatMessage {
    /// The content of the message
    pub message: String,
}

/// Represents the response to a chat message.
#[derive(Debug, Serialize)]
pub struct ChatResponse {
    /// The AI-generated response
    pub response: String,
    /// Suggested questions based on the current step
    pub suggested_questions: Vec<String>,
}

/// Represents a request to the Claude API.
#[derive(Debug, Serialize)]
pub struct ClaudeRequest {
    /// The model to use for generation
    pub model: String,
    /// Maximum number of tokens to generate
    pub max_tokens: u32,
    /// The conversation history and new message
    pub messages: Vec<ClaudeMessage>,
    /// The system prompt to set the context for the conversation
    pub system: Option<String>,
}

/// Represents a single message in the Claude API request.
#[derive(Debug, Serialize)]
pub struct ClaudeMessage {
    /// The role of the message sender (e.g., "user" or "assistant")
    pub role: String,
    /// The content of the message
    pub content: String,
    /// The name of the message sender (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Represents the response from the Claude API.
#[derive(Debug, Deserialize)]
pub struct ClaudeResponse {
    /// The generated content
    pub content: Vec<ClaudeContent>,
    /// Unique identifier for the response
    pub id: String,
    /// The model used for generation
    pub model: String,
    /// The role of the response (typically "assistant")
    pub role: String,
    /// The reason why the generation stopped
    pub stop_reason: Option<String>,
    /// The stop sequence that ended the generation, if any
    pub stop_sequence: Option<String>,
    /// The type of the response
    #[serde(rename = "type")]
    pub response_type: String,
    /// Usage statistics for the API call
    pub usage: ClaudeUsage,
}

/// Represents the content of a Claude API response.
#[derive(Debug, Deserialize)]
pub struct ClaudeContent {
    /// The generated text
    pub text: String,
    /// The type of the content
    #[serde(rename = "type")]
    pub content_type: String,
}

/// Represents the usage statistics for a Claude API call.
#[derive(Debug, Deserialize)]
pub struct ClaudeUsage {
    /// Number of input tokens
    pub input_tokens: u32,
    /// Number of output tokens
    pub output_tokens: u32,
}
