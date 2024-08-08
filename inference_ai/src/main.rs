use inference_ai::InferenceAI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut inference_ai = InferenceAI::new("http://localhost".to_string(), 8080);
    inference_ai.completion("new_text").await;
    Ok(())
}
