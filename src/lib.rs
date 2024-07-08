use worker::*;
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize)]
struct Topic {
    id: String,
    title: String,
    description: String,
    steps: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct GenericResponse {
    status: u16,
    message: String,
}

#[derive(Debug, Deserialize)]
struct ProgressUpdate {
    completed_step: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Progress {
    topic_id: String,
    completed_steps: Vec<usize>,
    current_step: usize,
}

#[derive(Debug, Deserialize)]
struct ChatMessage {
    message: String,
}

#[derive(Debug, Serialize)]
struct ChatResponse {
    response: String,
}

#[derive(Debug, Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<ClaudeMessage>,
}

#[derive(Debug, Serialize)]
struct ClaudeMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeResponse {
    content: Vec<ClaudeContent>,
    id: String,
    model: String,
    role: String,
    stop_reason: Option<String>,
    stop_sequence: Option<String>,
    #[serde(rename = "type")]
    response_type: String,
    usage: ClaudeUsage,
}

#[derive(Debug, Deserialize)]
struct ClaudeContent {
    text: String,
    #[serde(rename = "type")]
    content_type: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeUsage {
    input_tokens: u32,
    output_tokens: u32,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_log!(
        "Received {} request for {}",
        req.method().to_string(),
        req.path()
    );
 
    // Handle CORS preflight requests
    if req.method() == Method::Options {
        return handle_cors_preflight();
    }
    
    let router = Router::new();
    router
        .get_async("/api/topics", handle_get_topics)
        .get_async("/api/topics/:topicId", handle_get_topic)
        .post_async("/api/progress/:topicId", handle_post_progress)
        .get_async("/api/progress/:topicId", handle_get_progress)
        .post_async("/api/chat/:topicId", handle_post_chat)
        .run(req, env)
        .await
        .map(|mut res| {
            add_cors_headers(&mut res);
            res
        })
}

fn handle_cors_preflight() -> Result<Response> {
    let mut headers = Headers::new();
    headers.set("Access-Control-Allow-Origin", "https://devops-ai-react.pages.dev")?;
    headers.set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "Content-Type")?;
    headers.set("Access-Control-Max-Age", "86400")?;
    
    Ok(Response::ok("").unwrap().with_headers(headers))
}

fn add_cors_headers(res: &mut Response) {
    res.headers_mut()
        .set("Access-Control-Allow-Origin", "https://devops-ai-react.pages.dev").unwrap();
    res.headers_mut()
        .set("Access-Control-Allow-Methods", "GET, POST, OPTIONS").unwrap();
    res.headers_mut()
        .set("Access-Control-Allow-Headers", "Content-Type").unwrap();
}

async fn handle_get_topics(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    console_log!("Handling GET request to /api/topics");

    let topics = vec![
        Topic {
            id: "github-setup".to_string(),
            title: "GitHub Setup".to_string(),
            description: "Learn how to set up your GitHub account".to_string(),
            steps: vec![
                "Create a GitHub account".to_string(),
                "Set up your profile".to_string(),
                "Install Git on your local machine".to_string(),
                "Configure Git with your GitHub credentials".to_string(),
                "Set up SSH keys for secure authentication".to_string(),
                "Create your first repository".to_string(),
                "Clone the repository to your local machine".to_string(),
                "Make changes and commit them".to_string(),
                "Push changes to GitHub".to_string(),
                "Create a branch and make a pull request".to_string(),
                "Collaborate on a project".to_string(),
            ],
        },
        Topic {
            id: "docker-basics".to_string(),
            title: "Docker Basics".to_string(),
            description: "Introduction to Docker containerization".to_string(),
            steps: vec![
                "Install Docker".to_string(),
                "Run your first container".to_string(),
                "Build a custom Docker image".to_string(),
            ],
        },
    ];

    Response::from_json(&topics)
}

async fn handle_get_topic(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    console_log!("Handling GET request to /api/topics/:topicId");

    // Convert Option<&String> to Option<&str>, then unwrap with a default
    let topic_id: &str = ctx.param("topicId").map(|s| s.as_str()).unwrap_or("");
    console_log!("Requested topic ID: {}", topic_id);

    // In a real implementation, you would fetch the topic from a database
    // For now, we'll return a hardcoded topic if the ID matches, or a 404 if it doesn't
    let topic = match topic_id {
        "github-setup" => Topic {
            id: "github-setup".to_string(),
            title: "GitHub Setup".to_string(),
            description: "Learn how to set up your GitHub account".to_string(),
            steps: vec![
                "Create a GitHub account".to_string(),
                "Set up SSH keys".to_string(),
                "Create your first repository".to_string(),
            ],
        },
        "docker-basics" => Topic {
            id: "docker-basics".to_string(),
            title: "Docker Basics".to_string(),
            description: "Introduction to Docker containerization".to_string(),
            steps: vec![
                "Install Docker".to_string(),
                "Run your first container".to_string(),
                "Build a custom Docker image".to_string(),
            ],
        },
        _ => return Response::error("Topic not found", 404),
    };

    Response::from_json(&topic)
}

async fn handle_post_progress(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    console_log!("Handling POST request to /api/progress/:topicId");

    let topic_id: &str = ctx.param("topicId").map(|s| s.as_str()).unwrap_or("");
    console_log!("Topic ID for progress update: {}", topic_id);

    // Check if the topic exists
    if !topic_exists(topic_id) {
        return Response::error("Topic not found", 404);
    }

    // Parse the JSON body
    let progress_update: ProgressUpdate = match req.json().await {
        Ok(update) => update,
        Err(e) => {
            console_error!("Error parsing progress update: {:?}", e);
            return Response::error("Invalid JSON input", 400);
        }
    };

    // Validate input
    if progress_update.completed_step > 100 {  // Assuming max 100 steps per topic
        return Response::error("Invalid step number", 400);
    }

    // In a real implementation, you would update the progress in a database
    // For now, we'll just log the update and return a success message
    console_log!(
        "Updated progress for topic {}: Step {} completed",
        topic_id,
        progress_update.completed_step
    );

    Response::from_json(&GenericResponse {
        status: 200,
        message: format!("Progress updated for topic {}. Step {} marked as completed.", topic_id, progress_update.completed_step),
    })
}

async fn handle_get_progress(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    console_log!("Handling GET request to /api/progress/:topicId");

    let topic_id: &str = ctx.param("topicId").map(|s| s.as_str()).unwrap_or("");
    console_log!("Requested progress for topic ID: {}", topic_id);

    // Check if the topic exists
    if !topic_exists(topic_id) {
        return Response::error("Topic not found", 404);
    }

    // In a real implementation, you would fetch the progress from a database
    // For now, we'll return a hardcoded progress
    let progress = Progress {
        topic_id: topic_id.to_string(),
        completed_steps: vec![0],  // Assuming steps 0 and 1 are completed
        current_step: 1,  // Assuming the user is currently on step 2
    };

    Response::from_json(&progress)
}

async fn handle_post_chat(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    console_log!("Handling POST request to /api/chat/:topicId");

    let topic_id: &str = ctx.param("topicId").map(|s| s.as_str()).unwrap_or("");
    console_log!("Chat message for topic ID: {}", topic_id);

    // Check if the topic exists
    if !topic_exists(topic_id) {
        return Response::error("Topic not found", 404);
    }

    // Parse the JSON body
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

    let api_key = ctx.env.secret("ANTHROPIC_API_KEY")?.to_string();

    match call_claude_api(&chat_message.message, &api_key).await {
        Ok(response) => Response::from_json(&ChatResponse { response }),
        Err(e) => {
            console_error!("Error calling Claude API: {:?}", e);
            Response::error("Failed to generate response", 500)
        }
    }
}

async fn call_claude_api(message: &str, api_key: &str) -> Result<String> {
    let client = Client::new();
    let url = "https://api.anthropic.com/v1/messages";

    let claude_request = ClaudeRequest {
        model: "claude-3-5-sonnet-20240620".to_string(),
        max_tokens: 1024,
        messages: vec![ClaudeMessage {
            role: "user".to_string(),
            content: message.to_string(),
        }],
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

fn topic_exists(topic_id: &str) -> bool {
    matches!(topic_id, "github-setup" | "docker-basics")
}
