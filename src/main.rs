mod ai;

use std::sync::Arc;

use ai::GenerationTemplate;
use dotenv::dotenv;
use ollama_rs::Ollama;
use poise::serenity_prelude as serenity;
use tokio::sync::Mutex;

struct Data {
    ollama: Mutex<Ollama>,
    generation_template: Arc<GenerationTemplate>
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Response by an AI
#[poise::command(slash_command)]
async fn text(
    ctx: Context<'_>,
    #[description = "Prompt"] prompt: String,
) -> Result<(), Error> {
    let reply = {
        poise::CreateReply::default()
            .content("...")
    };
    let handle_response = ctx.send(reply).await?;
    let response = {
        let ollama = ctx.data().ollama.lock().await;
        println!("{:?}", ollama);
        let generation_template = Arc::clone(&ctx.data().generation_template);
        let generation_request = generation_template.template(prompt);
        println!("{:?}", generation_request);
        let result = ollama.generate(generation_request).await;
        println!("{:?}", result);
        match result {
            Ok(r) => r.response,
            Err(_) => String::from("Error response"),
        }
    };
    println!("response: {}", response);
    // Check when the response is more than 2000 characters
    handle_response.edit(
        ctx,
        poise::CreateReply::default()
            .content(response)
    ).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let host = std::env::var("OLLAMA_HOST").expect("missing OLLAMA_HOST");
    let _port = std::env::var("OLLAMA_PORT").expect("missing OLLAMA_PORT");

    let ollama = Ollama::new(host, 11434);
    let generation_template = GenerationTemplate::new(std::env::var("TEXT_MODEL").expect("missing TEXT_MODEL"));

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![text()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx, &framework.options().commands, serenity::GuildId::new(936376273138245652)).await?;
                Ok(Data {
                    ollama: Mutex::new(ollama),
                    generation_template: Arc::new(generation_template)
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
