//! This module contains handler functions for all API endpoints.

use worker::*;
use crate::types::{Topic, Step, Progress, ProgressUpdate, ChatMessage, ChatResponse, GenericResponse, ConversationHistory, TimestampedChatMessage};
use crate::claude;
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

    let topic = match topic_id {
        "github-setup" => Topic {
            id: "github-setup".to_string(),
            title: "GitHub Setup".to_string(),
            description: "Learn how to set up your GitHub account and start using Git".to_string(),
            initial_message: "Welcome to the GitHub Setup guide! This interactive tutorial will help you set up and use a GitHub account. Here's how it works:

1. Steps and Prompts:
   - On the left, you'll see a list of steps in your GitHub learning journey.
   - Each step is also a pre-written question that you can send to me, your AI assistant.
   - Clicking on a step will send its associated question, and I'll provide detailed instructions or information.

2. Learning Process:
   - Start with the first step and work your way down the list.
   - Click on a step to see instructions for that part of the setup process.
   - Follow the instructions and ask any additional questions you have in the chat.

3. Marking Progress:
   - After completing a step, click the checkmark icon next to the send button to mark it as done.
   - This helps you keep track of your progress and tells me you're ready for the next step.

4. Flexibility:
   - Feel free to click on any step, even out of order, if you need specific information.
   - If you're already familiar with some steps, you can mark them as complete and move on.

5. Additional Questions:
   - At any point, you can type your own questions in the chat for more clarification or help.

Remember, I'm here to assist you throughout the process. Don't hesitate to ask for more explanations or examples if something isn't clear.

Are you ready to begin? Click on the first step whenever you're ready to start your GitHub setup journey!".to_string(),
            steps: vec![
                Step {
                    title: "Create a GitHub account".to_string(),
                    prompt: "Provide a concise, step-by-step guide on how to create a GitHub account, focusing only on the essential steps.".to_string(),
                },
                Step {
                    title: "Install Git on your local machine".to_string(),
                    prompt: "Provide a short, clear explanation on how to install Git on a local machine, mentioning steps for common operating systems.".to_string(),
                },
                Step {
                    title: "Set up SSH keys for secure authentication".to_string(),
                    prompt: "Provide a brief, step-by-step guide on how to set up SSH keys for GitHub authentication.".to_string(),
                },
                Step {
                    title: "Configure Git with your GitHub credentials".to_string(),
                    prompt: "Explain concisely how to configure Git with GitHub credentials, focusing only on the essential commands.".to_string(),
                },
                Step {
                    title: "Create your first repository".to_string(),
                    prompt: "Explain succinctly how to create a new repository on GitHub, covering only the basic steps.".to_string(),
                },
                Step {
                    title: "Clone the repository to your local machine".to_string(),
                    prompt: "Provide a concise explanation of how to clone a GitHub repository to a local machine, including the basic command.".to_string(),
                },
                Step {
                    title: "Make changes and commit them".to_string(),
                    prompt: "Explain briefly how to make changes to files and commit them using Git, focusing on the essential commands.".to_string(),
                },
                Step {
                    title: "Push changes to GitHub".to_string(),
                    prompt: "Provide a short, clear explanation of how to push local commits to GitHub, including the basic command.".to_string(),
                },
                Step {
                    title: "Create a branch and make a pull request".to_string(),
                    prompt: "Explain concisely how to create a branch and make a pull request on GitHub, covering only the essential steps.".to_string(),
                },
                Step {
                    title: "Collaborate on a project".to_string(),
                    prompt: "Provide a brief overview of how to start collaborating on a GitHub project, mentioning key concepts like forking and contributing.".to_string(),
                },
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
    match claude::call_claude_api_with_history(&conversation.messages, &api_key).await {
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
