use axum::{
    extract::Json,
    response::Json as JsonResponse,
};
use serde::{Deserialize, Serialize};

// Handler for the root route
pub async fn hello_world() -> &'static str {
    "Hello, World! Welcome to the Rust Demo Service!"
}

// Data model for echo request/response
#[derive(Serialize, Deserialize)]
pub struct EchoRequest {
    message: String,
}

// Handler for the /echo route
pub async fn echo(Json(payload): Json<EchoRequest>) -> JsonResponse<EchoRequest> {
    JsonResponse(EchoRequest {
        message: format!("Echo: {}", payload.message),
    })
}