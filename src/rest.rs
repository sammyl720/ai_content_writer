use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{agent::Agent, errors::ApiError, state::AppState};

#[derive(Deserialize, Serialize)]
pub struct Prompt {
    q: String,
}

#[axum::debug_handler]
pub async fn prompt(
    State(state): State<AppState>,
    Json(prompt): Json<Prompt>,
) -> Result<impl IntoResponse, ApiError> {
    let data = state.researcher.prepare_data(&prompt.q).await?;
    let researcher_result = state.researcher.prompt(&prompt.q, data).await?;

    let writer_result = state.writer.prompt(&prompt.q, researcher_result).await?;

    Ok(writer_result)
}
