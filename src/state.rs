use crate::{researcher::Researcher, writer::Writer};

#[derive(Clone)]
pub struct AppState {
    pub researcher: Researcher,
    pub writer: Writer,
}

impl AppState {
    pub fn new(openai_api_key: String, serper_api_key: String) -> Self {
        let researcher = Researcher::new(openai_api_key.clone(), serper_api_key);
        let writer = Writer::new(openai_api_key);

        Self { researcher, writer }
    }
}
