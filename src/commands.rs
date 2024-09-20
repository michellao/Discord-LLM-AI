use controller::{Controller, UserController};
use database::{model::{Message, User}, Database};
use crate::{Context,Error};

async fn get_from_database_or_create_user(user_controller: &UserController<'_>, database: &Database, author: &serenity::model::prelude::User) -> User {
    let discord_id = i64::try_from(author.id.get()).expect("Overflow i64 on discord_id");
    let author_user = match user_controller.get_by_discord_id(discord_id).await {
        None => {
            let mut u = User { is_bot: Some(author.bot), discord_id, ..Default::default() };
            let _r = database.insert(&mut u).await;
            u
        }
        Some(a) => a
    };
    author_user
}

async fn get_from_database_or_create_bot_user(user_controller: &UserController<'_>, database: &Database, ctx: Context<'_>) -> User {
    let discord_id = i64::try_from(ctx.cache().current_user().id.get()).expect("Overflow i64 on discord_id");
    let user = match user_controller.get_by_discord_id(discord_id).await {
        None => {
            let mut u = User { is_bot: Some(ctx.cache().current_user().bot), discord_id, ..Default::default() };
            database.insert(&mut u).await;
            u
        }
        Some(a) => a
    };
    user
}

/// Response by an AI
#[poise::command(slash_command)]
pub async fn text(
    ctx: Context<'_>,
    #[description = "Prompt"] prompt: String,
) -> Result<(), Error> {
    let reply = {
        poise::CreateReply::default()
            .content("Generating...")
    };
    let handle_generating_message = ctx.send(reply).await?;

    let generation_ai = &ctx.data().generation_ai;

    let database = &ctx.data().database;
    let user_controller = UserController::new(database);
    let author_user = ctx.author();
    let author_bot = get_from_database_or_create_bot_user(&user_controller, database, ctx).await;
    let author_model_user = get_from_database_or_create_user(&user_controller, database, author_user).await;
    println!("{:?}", author_model_user);

    let response = generation_ai.generate(&prompt).await;

    let mut message_from_user = Message { user_id: author_model_user, content: prompt, ..Default::default() };
    database.insert(&mut message_from_user).await;

    println!("Complete response: {}", response);
    let lenght_response = response.len();
    let character_limit = 2000;
    let mut message_from_bot = Message { user_id: author_bot, content: response.to_owned(), ..Default::default() };
    database.insert(&mut message_from_bot).await;
    let mut complete_response: Vec<&str> = vec![];
    if lenght_response > character_limit {
        let mut response_to_divide = response.as_str();
        let nb_divide = lenght_response / character_limit;
        for _i in 0..nb_divide {
            let split_response = response_to_divide.split_at(character_limit);
            complete_response.push(split_response.0);
            response_to_divide = split_response.1;
        }
        complete_response.push(response_to_divide);
    } else {
        complete_response.push(response.as_str());
    }

    println!("Separate response: {:?}", complete_response);

    for answer in 0..complete_response.len() {
        if answer == 0 {
            handle_generating_message.edit(
                ctx,
                poise::CreateReply::default()
                    .content(complete_response[answer])
            ).await?;
        } else {
            let message = {
                poise::CreateReply::default()
                    .content(complete_response[answer])
            };
            ctx.send(message).await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn clear_conversation(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let generation_ai = &ctx.data().generation_ai;
    generation_ai.clear_conversation().await;
    ctx.say("Clear all previous messages").await?;
    Ok(())
}
