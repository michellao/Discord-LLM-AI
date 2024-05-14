mod ai;
mod db;

use db::Database;
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

    handle_response.edit(
        ctx,
        poise::CreateReply::default()
            .content(response)
    ).await?;
    Ok(())
}

#[derive(Debug)]
struct User {
    id_user: u64,
    is_bot: bool,
    discord_id: u64
}
#[derive(Debug)]
struct Message {
    id_message: u64,
    user: User,
    content: String
}

fn setup_ai() -> GenerationAI {
    let host = std::env::var("OLLAMA_HOST").expect("missing OLLAMA_HOST");
    let port_string = std::env::var("OLLAMA_PORT").expect("missing OLLAMA_PORT");
    let text_model = std::env::var("TEXT_MODEL").expect("missing TEXT_MODEL");
    let port = match port_string.as_str().parse::<u16>() {
        Ok(r) => r,
        Err(_) => panic!("Invalid port")
    };
    GenerationAI::new(text_model, host, port)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database = Database::new();
    
    let generation_ai = setup_ai();

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
