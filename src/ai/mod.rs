use inference_ai::InferenceAI;
use tokio::sync::Mutex;

pub struct GenerationAI {
    llamacpp: Mutex<InferenceAI>
}

impl GenerationAI {
    pub fn new(model_name: String, host: String, port: u16) -> Self {
        Self {
            llamacpp: Mutex::new(InferenceAI::new(model_name, host, port)),
        }
    }

    pub async fn generate(&self, prompt: String) -> String {
        let mut llamacpp = self.llamacpp.lock().await;
        match llamacpp.completion(&prompt).await {
            None => String::from("Error completion"),
            Some(response) => response
        }
    }
}
