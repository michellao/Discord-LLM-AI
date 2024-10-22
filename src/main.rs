mod ai;
mod commands;
mod utility;

use database::Database;
use dotenv::dotenv;
use ai::GenerationAI;
use poise::serenity_prelude as serenity;

pub struct DataDiscord {
    generation_ai: GenerationAI,
    database: Database
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, DataDiscord, Error>;

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

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    if let Ok(sentry_url) = std::env::var("SENTRY_URL") {
        let _guard = sentry::init((sentry_url, sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        }));
    }

    let database_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");

    let database = Database::new(database_url);

    let generation_ai = setup_ai();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::text(), commands::clear_conversation(), commands::subscribe_conversation()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx, &framework.options().commands, serenity::GuildId::new(936376273138245652)).await?;
                Ok(DataDiscord {
                    generation_ai,
                    database
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    println!("Ready!");
    client.unwrap().start().await.unwrap();
    Ok(())
}
