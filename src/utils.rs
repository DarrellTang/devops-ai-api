//! This module contains utility functions used across the application.

use worker::*;

/// Handles CORS preflight requests.
///
/// This function sets up the necessary headers for CORS preflight requests.
///
/// # Returns
///
/// A `Result<Response>` with the appropriate CORS headers set.
pub fn handle_cors_preflight() -> Result<Response> {
    let mut headers = Headers::new();
    headers.set("Access-Control-Allow-Origin", "https://devops-ai-react.pages.dev")?;
    headers.set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "Content-Type")?;
    headers.set("Access-Control-Max-Age", "86400")?;
    
    Ok(Response::ok("").unwrap().with_headers(headers))
}

/// Adds CORS headers to a response.
///
/// This function adds the necessary CORS headers to an existing response.
///
/// # Arguments
///
/// * `res` - A mutable reference to the Response to which headers will be added
///
/// # Panics
///
/// This function will panic if it fails to set any of the CORS headers.
pub fn add_cors_headers(res: &mut Response) {
    res.headers_mut()
        .set("Access-Control-Allow-Origin", "https://devops-ai-react.pages.dev")
        .expect("Failed to set Access-Control-Allow-Origin header");
    res.headers_mut()
        .set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .expect("Failed to set Access-Control-Allow-Methods header");
    res.headers_mut()
        .set("Access-Control-Allow-Headers", "Content-Type")
        .expect("Failed to set Access-Control-Allow-Headers header");
}
