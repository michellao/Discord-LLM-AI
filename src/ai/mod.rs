use inference_ai::InferenceAI;
use tokio::sync::Mutex;
use database::model::{Message as MessageDb, User};
use inference_ai::model::{Message, Role};

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
        llamacpp.completion(prompt).await.unwrap_or_else(|| String::from("Error completion"))
    }

    pub async fn clear_conversation(&self) {
        let mut llamacpp = self.llamacpp.lock().await;
        llamacpp.reset_openai_completion(self.model_name.to_owned());
    }

    pub async fn init_conversation(&self, messages: Vec<(User, MessageDb)>) {
        let mut openai_completion = InferenceAI::initialize_openai_completion(self.model_name.to_owned());
        for (u, m) in messages {
            let role = if u.is_bot {
                Role::Assistant
            } else {
                Role::User
            };
            let message = Message { content: m.content, role };
            openai_completion.new_message(message);
        }
        let mut llamacpp = self.llamacpp.lock().await;
        llamacpp.set_openai_completion(openai_completion);
    }
}
