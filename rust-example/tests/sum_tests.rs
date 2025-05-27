// Import necessary Actix Web test utilities, HTTP status types, and web routing components
use actix_web::{body::to_bytes, http::StatusCode, test, web, App};

// Import the handler module from your main crate.
// Note: crate name is "rust-example" in Cargo.toml, which becomes "rust_example" here.
use rust_example::handlers;

//
// Test 1: Provide both a and b query parameters
//
#[actix_rt::test]
async fn test_sum_valid_params() {
    // Create a test server with /sum route
    let app = test::init_service(App::new().route("/sum", web::get().to(handlers::sum))).await;

    // Simulate a GET request to /sum?a=10&b=15
    let req = test::TestRequest::get().uri("/sum?a=10&b=15").to_request();
    let resp = test::call_service(&app, req).await;

    // Verify HTTP 200 OK
    assert_eq!(resp.status(), StatusCode::OK);

    // Extract response body and verify sum is correct
    let body = to_bytes(resp.into_body()).await.unwrap();
    assert_eq!(body, "25");
}

//
// Test 2: Provide only parameter a
//
#[actix_rt::test]
async fn test_sum_param_a_only() {
    let app = test::init_service(App::new().route("/sum", web::get().to(handlers::sum))).await;

    // Simulate request with only a=10
    let req = test::TestRequest::get().uri("/sum?a=10").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body = to_bytes(resp.into_body()).await.unwrap();
    assert_eq!(body, "10"); // b is missing, so treated as 0
}

//
// Test 3: Provide only parameter b
//
#[actix_rt::test]
async fn test_sum_param_b_only() {
    let app = test::init_service(App::new().route("/sum", web::get().to(handlers::sum))).await;

    // Simulate request with only b=15
    let req = test::TestRequest::get().uri("/sum?b=15").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body = to_bytes(resp.into_body()).await.unwrap();
    assert_eq!(body, "15"); // a is missing, so treated as 0
}

//
// Test 4: Provide extra irrelevant query parameters
//
#[actix_rt::test]
async fn test_sum_param_additional_parameters() {
    let app = test::init_service(App::new().route("/sum", web::get().to(handlers::sum))).await;

    // Only b=15 is relevant for the sum; x and y should be ignored
    let req = test::TestRequest::get()
        .uri("/sum?y=21&x=10&b=15")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body = to_bytes(resp.into_body()).await.unwrap();
    assert_eq!(body, "15"); // a is missing → 0 + b=15 → 15
}

//
// Test 5: Provide no parameters at all
//
#[actix_rt::test]
async fn test_sum_missing_params() {
    let app = test::init_service(App::new().route("/sum", web::get().to(handlers::sum))).await;

    // Request with no query parameters
    let req = test::TestRequest::get().uri("/sum").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body = to_bytes(resp.into_body()).await.unwrap();
    // assert_eq!(body, "0"); // both a and b default to 0
    assert_eq!(1, 0);
}
