use ollama_rs::generation::{completion::request::GenerationRequest, options::GenerationOptions, parameters::{KeepAlive, TimeUnit}};

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
        let options = GenerationOptions::default()
            .temperature(0.6)
            .num_predict(256)
            .num_ctx(8192);
        generation_request
            .keep_alive(KeepAlive::Until { time: 300, unit: TimeUnit::Minutes })
            .options(options)
    }
}