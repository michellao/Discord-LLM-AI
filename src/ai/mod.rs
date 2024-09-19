use inference_ai::InferenceAI;
use tokio::sync::Mutex;

pub struct GenerationAI {
    model_name: String,
    llamacpp: Mutex<InferenceAI>
}

impl GenerationAI {
    pub fn new(model_name: String, host: String, port: u16) -> Self {
        Self {
            model_name: model_name.clone(),
            llamacpp: Mutex::new(InferenceAI::new(model_name, host, port))
        }
    }

    pub async fn generate(&self, prompt: &str) -> String {
        let mut llamacpp = self.llamacpp.lock().await;
        match llamacpp.completion(prompt).await {
            None => String::from("Error completion"),
            Some(response) => response
        }
    }

    pub async fn clear_conversation(&self) {
        let mut llamacpp = self.llamacpp.lock().await;
        llamacpp.completion_data = InferenceAI::initialize_openai_completion(self.model_name.clone());
    }
}
