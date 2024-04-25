use std::sync::Mutex;

use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use poise::serenity_prelude as serenity;

struct Data {
    ollama: Mutex<Ollama>
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn text(
    ctx: Context<'_>,
    #[description = "Prompt"] prompt: Option<String>,
) -> Result<(), Error> {
    let ollama = ctx.data().ollama.lock().unwrap();
    let res = ollama.generate(GenerationRequest::new("llama3:8b-instruct-q5_0".to_string(), prompt.unwrap()));
    let response = format!("{}");
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let ollama = Ollama::default();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![text()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    ollama: Mutex::new(ollama)
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
