use ollama_rs::{generation::{completion::request::GenerationRequest, options::GenerationOptions, parameters::{KeepAlive, TimeUnit}}, Ollama};
use tokio::sync::Mutex;

pub struct GenerationAI {
    model_name: String,
    ollama: Mutex<Ollama>
}

pub trait TextGeneration {
    async fn generate(&self, prompt: String) -> String;
}

impl GenerationAI {
    pub fn new(model_name: String, host: String, port: u16) -> Self {
        Self {
            model_name,
            ollama: Mutex::new(Ollama::new(host, port))
        }
    }

    fn template_ollama_request(&self, prompt: String) -> GenerationRequest {
        let generation_request = GenerationRequest::new(self.model_name.to_string(), prompt);
        let options = GenerationOptions::default()
            .temperature(0.6)
            .num_predict(390)
            .num_ctx(8192);
        generation_request
            .keep_alive(KeepAlive::Until { time: 300, unit: TimeUnit::Minutes })
            .options(options)
    }
}

impl TextGeneration for GenerationAI {
    async fn generate(&self, prompt: String) -> String {
        let ollama = self.ollama.lock().await;
        let res = ollama.generate(self.template_ollama_request(prompt)).await;
        match res {
            Ok(data) => data.response,
            Err(_) => String::from("Error response")
        }
    }
}