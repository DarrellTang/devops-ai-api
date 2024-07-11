//! This module contains handler functions for all API endpoints.

use worker::*;
use crate::types::{Topic, Step, Progress, ProgressUpdate, ChatMessage, ChatResponse, GenericResponse};
use crate::claude;

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

    let topics = vec![
        Topic {
            id: "github-setup".to_string(),
            title: "GitHub Setup".to_string(),
            description: "Learn how to set up your GitHub account".to_string(),
            initial_message: "Welcome to the GitHub Setup guide! This interactive tutorial will help you set up and use a GitHub account. Click on the first step to begin.".to_string(),
            steps: vec![
                Step {
                    title: "Create a GitHub account".to_string(),
                    prompt: "I'd like to create a GitHub account. Can you provide me with step-by-step instructions on how to do this? Please include information on:\n1. How to navigate to the GitHub signup page\n2. What information I'll need to provide\n3. How to choose a good username\n4. Tips for creating a secure password\n5. How to verify my email address\n6. Any important settings I should configure after creating my account\nThank you!".to_string(),
                },
                Step {
                    title: "Set up your profile".to_string(),
                    prompt: "Now that I have created my GitHub account, how do I set up my profile? Please provide guidance on:\n1. How to access my profile settings\n2. What information should I include in my profile\n3. How to add a profile picture\n4. How to write an effective bio\n5. Any other important profile elements I should consider\nThank you!".to_string(),
                },
                // Add more steps as needed
            ],
        },
        // Add more topics as needed
    ];

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

    // In a real implementation, you would fetch the topic from a database
    // For now, we'll return a hardcoded topic if the ID matches, or a 404 if it doesn't
    let topic = match topic_id {
        "github-setup" => Topic {
            id: "github-setup".to_string(),
            title: "GitHub Setup".to_string(),
            description: "Learn how to set up your GitHub account".to_string(),
            initial_message: "Welcome to the GitHub Setup guide! This interactive tutorial will help you set up and use a GitHub account. Click on the first step to begin.".to_string(),
            steps: vec![
                Step {
                    title: "Create a GitHub account".to_string(),
                    prompt: "I'd like to create a GitHub account. Can you provide me with step-by-step instructions on how to do this? Please include information on:\n1. How to navigate to the GitHub signup page\n2. What information I'll need to provide\n3. How to choose a good username\n4. Tips for creating a secure password\n5. How to verify my email address\n6. Any important settings I should configure after creating my account\nThank you!".to_string(),
                },
                Step {
                    title: "Set up your profile".to_string(),
                    prompt: "Now that I have created my GitHub account, how do I set up my profile? Please provide guidance on:\n1. How to access my profile settings\n2. What information should I include in my profile\n3. How to add a profile picture\n4. How to write an effective bio\n5. Any other important profile elements I should consider\nThank you!".to_string(),
                },
                // Add more steps as needed
            ],
        },
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

    let kv = ctx.kv("PROGRESS_STORE")?;
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

    let kv = ctx.kv("PROGRESS_STORE")?;
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

    let topic_id: &str = ctx.param("topicId").map(|s| s.as_str()).unwrap_or("");
    console_log!("Chat message for topic ID: {}", topic_id);

    if !topic_exists(topic_id) {
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

    let api_key = ctx.secret("ANTHROPIC_API_KEY")?.to_string();

    match claude::call_claude_api(&chat_message.message, &api_key).await {
        Ok(response) => Response::from_json(&ChatResponse { response }),
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
    console_log!("Resetting progress for topic ID: {}", topic_id);

    if !topic_exists(&topic_id) {
        return Response::error("Topic not found", 404);
    }

    let kv = ctx.kv("PROGRESS_STORE")?;
    let progress = Progress {
        topic_id: topic_id.clone(),
        completed_steps: vec![],
        current_step: 0,
    };

    kv.put(&topic_id, serde_json::to_string(&progress)?)?
        .execute().await?;

    Response::from_json(&GenericResponse {
        status: 200,
        message: format!("Progress reset for topic {}.", topic_id),
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
