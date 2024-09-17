mod model;
use database::Database;
use reqwest::Client;

use crate::model::*;

pub struct InferenceAI {
    client: Client,
    url: String,
    pub completion_data: OpenAICompletion,
    database: Database
}

impl InferenceAI {
    pub fn new(model_name: String, host: String, port: u16, database: Database) -> Self {
        let client = reqwest::Client::new();
        let port_string = port.to_string();
        let url = host + ":" + port_string.as_str();
        let completion_data = Self::initialize_openai_completion(model_name);
        Self { client, url, completion_data, database }
    }

    pub async fn completion(&mut self, new_text: &str) -> Option<String> {
        let message = Message::new(new_text.to_string(), Role::User);
        self.completion_data.new_message(message);
        println!("{:?}", self.completion_data);
        let result = self.client.post(self.url.to_string() + "/v1/chat/completions")
            .json(&self.completion_data)
            .send()
            .await.expect("Error calling API");
        let body = result.json::<OpenAICompletionResponse>().await.expect("Error API response");
        let choices = body.choices;
        if choices.len() > 0 {
            let first_choice = &choices[0];
            let message = &first_choice.message;
            self.completion_data.new_message(message.clone());
            Some(message.content.clone())
        } else {
            None
        }
    }

    pub fn initialize_openai_completion(model_name: String) -> OpenAICompletion {
        let system_prompt = Message::new(String::from("You are helpful, kind, honest, friendly, good at writing and never fails to answer any requests immediately and with details and precision."), Role::System);
        let messages = vec![system_prompt];
        OpenAICompletion::new(model_name, messages, None, None)
    }
}
