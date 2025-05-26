// Import core Actix Web components:
// - `web` for routing and data handling
// - `App` for building the application instance
// - `HttpServer` to run the web server
use actix_web::{web, App, HttpServer};

// Declare the local module `handlers`, where the `/sum` route handler is defined
mod handlers;

// Main entry point for the Actix Web server.
// The `#[actix_web::main]` attribute sets up the async runtime.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create and run an HTTP server
    HttpServer::new(|| {
        // Construct the application with a single route:
        // - `GET /sum` is handled by the `handlers::sum` function
        App::new().route("/sum", web::get().to(handlers::sum))
    })
    // Bind the server to all IP addresses on port 8080
    .bind("0.0.0.0:8080")?
    // Start the server and await its completion
    .run()
    .await
}
