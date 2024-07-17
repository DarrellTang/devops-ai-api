//! This module serves as the main entry point for the DevOps AI API Worker.
//! It sets up the router and handles incoming requests.

use worker::*;

mod types;
mod handlers;
mod claude;
mod utils;
mod topics;

/// The main entry point for the Worker.
///
/// This function is called for each incoming request to the Worker.
/// It sets up CORS, initializes the router, and delegates to the appropriate handler.
///
/// # Arguments
///
/// * `req` - The incoming request
/// * `env` - The Worker environment
/// * `_ctx` - The execution context (unused)
///
/// # Returns
///
/// A `Result<Response>` representing the HTTP response to be sent back to the client.
#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // Log the incoming request details
    console_log!(
        "Received {} request for {}",
        req.method().to_string(),
        req.path()
    );
 
    // Handle CORS preflight requests
    if req.method() == Method::Options {
        return utils::handle_cors_preflight();
    }
    
    // Initialize the router and set up the routes
    let router = Router::new();
    router
        .get_async("/api/topics", handlers::handle_get_topics)
        .get_async("/api/topics/:topicId", handlers::handle_get_topic)
        .post_async("/api/progress/:topicId", handlers::handle_post_progress)
        .get_async("/api/progress/:topicId", handlers::handle_get_progress)
        .post_async("/api/chat/:topicId", handlers::handle_post_chat)
        .get_async("/api/conversation/:topicId", handlers::handle_get_conversation)
        .post_async("/api/reset/:topicId", handlers::handle_reset_progress)
        .run(req, env)
        .await
        .map(|mut res| {
            // Add CORS headers to the response
            utils::add_cors_headers(&mut res);
            res
        })
}
