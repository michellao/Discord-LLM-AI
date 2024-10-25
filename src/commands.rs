use database::{controller::Controller, insert_model::NewMessage};
use database::controller::{message_controller::MessageController, user_controller::UserController, user_conversation_controller::UserConversationController, subscribe_channel_controller::SubscribeChannelController};
use crate::{Context, Error};
use crate::utility::{get_from_database_or_create_bot_user, get_from_database_or_create_user, retrieve_conversation};
use poise::serenity_prelude as serenity;

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
    let message_controller = MessageController::new(database);
    let user_conversation_controller = UserConversationController::new(database);

    let author_bot = get_from_database_or_create_bot_user(&user_controller, &ctx);
    let author_model_user = get_from_database_or_create_user(&user_controller, &ctx);
    let conversation = retrieve_conversation(&user_conversation_controller, &author_model_user);
    let messages = message_controller.get_by_conversation(&conversation);
    generation_ai.init_conversation(messages).await;

    let response = generation_ai.generate(&prompt).await;

    let message_from_user = NewMessage { user_id: &author_model_user.id_user, content: &prompt, conversation_id: &conversation.id_conversation };
    message_controller.insert(&message_from_user);

    println!("Complete response: {}", response);
    let length_response = response.len();
    let character_limit = 2000;
    let message_from_bot = NewMessage { user_id: &author_bot.id_user, content: &response, conversation_id: &conversation.id_conversation };
    message_controller.insert(&message_from_bot);
    let mut complete_response: Vec<&str> = vec![];
    if length_response > character_limit {
        let mut response_to_divide = response.as_str();
        let nb_divide = length_response / character_limit;
        for _i in 0..nb_divide {
            let split_response = response_to_divide.split_at(character_limit);
            complete_response.push(split_response.0);
            response_to_divide = split_response.1;
        }
        complete_response.push(response_to_divide);
    } else {
        complete_response.push(response.as_str());
    }

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
    let database = &ctx.data().database;
    let user_controller = UserController::new(database);
    let generation_ai = &ctx.data().generation_ai;
    let author_user = get_from_database_or_create_user(&user_controller, &ctx);

    let message_controller = MessageController::new(database);
    let is_success = message_controller.delete_messages_by_user(&author_user);

    generation_ai.clear_conversation().await;
    if is_success {
        ctx.say("Delete on database").await?;
    }
    ctx.say("Clear all previous messages").await?;
    Ok(())
}

#[poise::command(context_menu_command = "Subscribe to thread")]
pub async fn subscribe_conversation(
    ctx: Context<'_>,
    msg: serenity::Message
) -> Result<(), Error> {
    use database::model::SubscribeChannel;
    match msg.thread {
        None => ctx.say("No thread detected").await?,
        Some(thread) => {
            let timestamp_expire = thread.thread_metadata.unwrap_or_else(|| panic!("This channel isn't a thread")).archive_timestamp.unwrap_or_default();
            let database = &ctx.data().database;
            let subscribe_controller = SubscribeChannelController::new(database);
            let thread_id: i64 = thread.id.get().try_into().unwrap();
            let subscribe_channel = SubscribeChannel {
                discord_channel_id:thread_id, expire_in: timestamp_expire.naive_local()
            };
            subscribe_controller.insert(&subscribe_channel);
            ctx.say("Detected thread and subscribe to it").await?
        }
    };
    Ok(())
}
