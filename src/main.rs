mod ai;

use database::Database;
use dotenv::dotenv;
use ai::{GenerationAI, TextGeneration};
use poise::serenity_prelude as serenity;
use rusqlite::Connection;

fn setup_ai() -> GenerationAI {
    let host = std::env::var("LLAMACPP_HOST").expect("missing LLAMACPP_HOST");
    let port_string = std::env::var("LLAMACPP_PORT").expect("missing LLAMACPP_PORT");
    let text_model = std::env::var("TEXT_MODEL").expect("missing TEXT_MODEL");
    let port = match port_string.as_str().parse::<u16>() {
        Ok(r) => r,
        Err(_) => panic!("Invalid port")
    };
    GenerationAI::new(text_model, host, port)
}


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

#[tokio::main]
async fn main() {
    let _guard = sentry::init(("https://bf6bfdebe100d884bbbcc932d54c73ba@o4507341534068736.ingest.de.sentry.io/4507392043712592", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));
    dotenv().ok();
    // let connection = Connection::open_in_memory().unwrap();
    // let database = Database::new(connection);

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
                    generation_ai,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
