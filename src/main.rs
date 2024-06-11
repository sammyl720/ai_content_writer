use axum::{
    routing::{get, post},
    Router,
};
use rest::prompt;
use shuttle_runtime::SecretStore;
use state::AppState;

async fn hello_world() -> &'static str {
    "Hello, world!"
}
mod agent;
mod common;
mod errors;
mod researcher;
mod rest;
mod state;
mod writer;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let openai_api_key = secrets.get("OPENAI_API_KEY").unwrap();
    let serper_api_key = secrets.get("SERPER_API_KEY").unwrap();

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/prompt", post(prompt))
        .with_state(AppState::new(openai_api_key, serper_api_key));

    Ok(router.into())
}
