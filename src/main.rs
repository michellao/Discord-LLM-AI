mod ai;

use dotenv::dotenv;
use ai::GenerationAI;
use poise::serenity_prelude as serenity;

use crate::ai::TextGeneration;

struct Data {
    generation_ai: GenerationAI,
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
            .content("Generating...")
    };
    let handle_response = ctx.send(reply).await?;

    let generation_ai = &ctx.data().generation_ai;
    let response = generation_ai.generate(prompt).await;
    
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
    let text_model = std::env::var("TEXT_MODEL").expect("missing TEXT_MODEL");
    let generation_ai = GenerationAI::new(text_model, host, 11434);

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
                    generation_ai: generation_ai,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
