// Import Actix Web components:
// - `web` for extracting query parameters
// - `HttpResponse` to construct HTTP responses
// - `Responder` trait to allow handler return types to be used as responses
use actix_web::{web, HttpResponse, Responder};

// Define an asynchronous handler function named `sum`
// It receives query parameters using `web::Query` mapped to a HashMap<String, String>
pub async fn sum(query: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    
    // Attempt to get the value of query parameter "a"
    // Try to parse it as an i32, default to 0 if missing or invalid
    let a = query
        .get("a")
        .and_then(|v| v.parse::<i32>().ok())
        .unwrap_or(0);

    // Same logic for query parameter "b"
    let b = query
        .get("b")
        .and_then(|v| v.parse::<i32>().ok())
        .unwrap_or(0);

    // Return HTTP 200 OK with the sum of a and b as plain text in the body
    HttpResponse::Ok().body(format!("{}", a + b))
}
