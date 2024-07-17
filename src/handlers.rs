//! This module contains handler functions for all API endpoints.

use worker::*;
use crate::types::{Topic, Step, Progress, ProgressUpdate, ChatMessage, ChatResponse, GenericResponse, ConversationHistory, TimestampedChatMessage};
use crate::claude;
use crate::topics;
use chrono::Utc;

/// Handles GET request for all topics.
///
/// # Arguments
///
/// * `_req` - The incoming request (unused)
/// * `_ctx` - The route context (unused)
///
/// # Returns
///
/// A `Result<Response>` containing a JSON array of all topics.
pub async fn handle_get_topics(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    console_log!("Handling GET request to /api/topics");

    let topics = topics::get_all_topics();
    Response::from_json(&topics)
}

/// Handles GET request for a specific topic.
///
/// # Arguments
///
/// * `_req` - The incoming request (unused)
/// * `ctx` - The route context containing the topic ID
///
/// # Returns
///
/// A `Result<Response>` containing a JSON object of the requested topic or a 404 error.
pub async fn handle_get_topic(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    console_log!("Handling GET request to /api/topics/:topicId");

    let topic_id: &str = ctx.param("topicId").map(|s| s.as_str()).unwrap_or("");
    console_log!("Requested topic ID: {}", topic_id);

    let topic = match topic_id {
        "github-setup" => topics::get_github_setup_topic(),
        _ => return Response::error("Topic not found", 404),
    };

    Response::from_json(&topic)
}

/// Handles POST request to update progress for a topic.
///
/// # Arguments
///
/// * `req` - The incoming request containing the progress update
/// * `ctx` - The route context containing the topic ID
///
/// # Returns
///
/// A `Result<Response>` confirming the progress update or an error.
pub async fn handle_post_progress(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    console_log!("Handling POST request to /api/progress/:topicId");

    let topic_id: String = ctx.param("topicId").map(|s| s.to_string()).unwrap_or_default();
    console_log!("Topic ID for progress update: {}", topic_id);

    if !topic_exists(&topic_id) {
        return Response::error("Topic not found", 404);
    }

    let progress_update: ProgressUpdate = match req.json().await {
        Ok(update) => update,
        Err(e) => {
            console_error!("Error parsing progress update: {:?}", e);
            return Response::error("Invalid JSON input", 400);
        }
    };

    let kv = ctx.kv("DATA_STORE")?;
    let mut progress: Progress = match kv.get(&topic_id).json().await? {
        Some(p) => p,
        None => Progress {
            topic_id: topic_id.clone(),
            completed_steps: vec![],
            current_step: 0,
        },
    };

    console_log!("Current progress before update: {:?}", progress);

    if progress.completed_steps.contains(&progress_update.completed_step) {
        console_log!("Step {} already completed", progress_update.completed_step);
    } else {
        progress.completed_steps.push(progress_update.completed_step);
        progress.completed_steps.sort(); // Ensure the list is always sorted
        progress.current_step = progress_update.completed_step + 1;
        
        console_log!("Updated progress: {:?}", progress);

        kv.put(&topic_id, serde_json::to_string(&progress)?)?
            .execute().await?;
    }

    Response::from_json(&GenericResponse {
        status: 200,
        message: format!("Progress updated for topic {}.", topic_id),
    })
}

/// Handles GET request to retrieve progress for a topic.
///
/// # Arguments
///
/// * `_req` - The incoming request (unused)
/// * `ctx` - The route context containing the topic ID
///
/// # Returns
///
/// A `Result<Response>` containing a JSON object of the progress or an error.
pub async fn handle_get_progress(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    console_log!("Handling GET request to /api/progress/:topicId");

    let topic_id: String = ctx.param("topicId").map(|s| s.to_string()).unwrap_or_default();
    console_log!("Requested progress for topic ID: {}", topic_id);

    if !topic_exists(&topic_id) {
        return Response::error("Topic not found", 404);
    }

    let kv = ctx.kv("DATA_STORE")?;
    let progress: Progress = match kv.get(&topic_id).json().await? {
        Some(p) => p,
        None => Progress {
            topic_id: topic_id.clone(),
            completed_steps: vec![],
            current_step: 0,
        },
    };

    Response::from_json(&progress)
}

/// Handles POST request for chat messages.
///
/// # Arguments
///
/// * `req` - The incoming request containing the chat message
/// * `ctx` - The route context containing the topic ID
///
/// # Returns
///
/// A `Result<Response>` containing the AI's response or an error.
pub async fn handle_post_chat(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    console_log!("Handling POST request to /api/chat/:topicId");

    let topic_id: String = ctx.param("topicId").map(|s| s.to_string()).unwrap_or_default();
    console_log!("Chat message for topic ID: {}", topic_id);

    if !topic_exists(&topic_id) {
        return Response::error("Topic not found", 404);
    }

    let chat_message: ChatMessage = match req.json().await {
        Ok(message) => message,
        Err(e) => {
            console_error!("Error parsing chat message: {:?}", e);
            return Response::error("Invalid JSON input", 400);
        }
    };

    if chat_message.message.trim().is_empty() {
        return Response::error("Message cannot be empty", 400);
    }

    let kv = ctx.kv("DATA_STORE")?;
    let conversation_key = format!("conversation_{}", topic_id);

    // Retrieve existing conversation or create a new one
    let mut conversation: ConversationHistory = match kv.get(&conversation_key).json().await? {
        Some(c) => c,
        None => ConversationHistory {
            topic_id: topic_id.clone(),
            messages: vec![],
        },
    };

    // Add the new user message to the conversation
    conversation.messages.push(TimestampedChatMessage {
        role: "user".to_string(),
        content: chat_message.message.clone(),
        timestamp: Utc::now(),
    });

    let api_key = ctx.secret("ANTHROPIC_API_KEY")?.to_string();

    // Call Claude API with the full conversation history
    match claude::call_claude_api_with_history(&conversation.messages, &api_key, &topic_id).await {
        Ok(response) => {
            // Add Claude's response to the conversation history
            conversation.messages.push(TimestampedChatMessage {
                role: "assistant".to_string(),
                content: response.clone(),
                timestamp: Utc::now(),
            });

            // Implement conversation management strategy (e.g., truncation)
            if conversation.messages.len() > 50 {  // Adjust this number as needed
                conversation.messages = conversation.messages.split_off(conversation.messages.len() - 50);
            }

            // Store the updated conversation
            kv.put(&conversation_key, serde_json::to_string(&conversation)?)?
              .execute()
              .await?;

            Response::from_json(&ChatResponse { response })
        },
        Err(e) => {
            console_error!("Error calling Claude API: {:?}", e);
            Response::error("Failed to generate response", 500)
        }
    }
}

/// Handles POST request to reset progress for a topic.
///
/// # Arguments
///
/// * `_req` - The incoming request (unused)
/// * `ctx` - The route context containing the topic ID
///
/// # Returns
///
/// A `Result<Response>` confirming the progress reset or an error.
pub async fn handle_reset_progress(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    console_log!("Handling POST request to /api/reset/:topicId");

    let topic_id: String = ctx.param("topicId").map(|s| s.to_string()).unwrap_or_default();
    console_log!("Resetting progress and conversation for topic ID: {}", topic_id);

    if !topic_exists(&topic_id) {
        return Response::error("Topic not found", 404);
    }

    let kv = ctx.kv("DATA_STORE")?;
    
    // Reset progress
    let progress = Progress {
        topic_id: topic_id.clone(),
        completed_steps: vec![],
        current_step: 0,
    };

    kv.put(&topic_id, serde_json::to_string(&progress)?)?
        .execute().await?;

    // Reset conversation history
    let conversation_key = format!("conversation_{}", topic_id);
    kv.delete(&conversation_key).await?;

    Response::from_json(&GenericResponse {
        status: 200,
        message: format!("Progress and conversation reset for topic {}.", topic_id),
    })
}

/// Checks if a topic exists.
///
/// # Arguments
///
/// * `topic_id` - The ID of the topic to check
///
/// # Returns
///
/// A boolean indicating whether the topic exists.
fn topic_exists(topic_id: &str) -> bool {
    matches!(topic_id, "github-setup" | "docker-basics")
}

pub async fn handle_get_conversation(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    console_log!("Handling GET request to /api/conversation/:topicId");

    let topic_id: String = ctx.param("topicId").map(|s| s.to_string()).unwrap_or_default();
    console_log!("Retrieving conversation for topic ID: {}", topic_id);

    if !topic_exists(&topic_id) {
        return Response::error("Topic not found", 404);
    }

    let kv = ctx.kv("DATA_STORE")?;
    let conversation_key = format!("conversation_{}", topic_id);

    match kv.get(&conversation_key).json::<ConversationHistory>().await? {
        Some(conversation) => Response::from_json(&conversation),
        None => {
            let empty_conversation = ConversationHistory {
                topic_id: topic_id.clone(),
                messages: vec![],
            };
            Response::from_json(&empty_conversation)
        }
    }
}
