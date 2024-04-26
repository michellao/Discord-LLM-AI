use ollama_rs::generation::{completion::request::GenerationRequest, parameters::{KeepAlive, TimeUnit}};

pub struct GenerationTemplate {
    model_name: String,
}

impl GenerationTemplate {
    pub fn new(model_name: String) -> GenerationTemplate {
        GenerationTemplate {
            model_name
        }
    }

    pub fn template(&self, prompt: String) -> GenerationRequest {
        let generation_request = GenerationRequest::new(self.model_name.to_string(), prompt);
        generation_request.keep_alive(KeepAlive::Until { time: 300, unit: TimeUnit::Minutes })
    }
}