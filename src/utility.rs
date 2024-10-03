use database::controller::Controller;
use database::controller::user_controller::UserController;
use database::controller::user_conversation_controller::UserConversationController;
use database::insert_model::NewUser;
use database::model::{Conversation, User};
use super::Context;

pub fn get_from_database_or_create_user<'a>(user_controller: &UserController<'a>, ctx: &Context<'a>) -> User {
    let author = ctx.author();
    let discord_id = i64::try_from(author.id.get()).expect("Overflow i64 on discord_id");
    let author_user = match user_controller.get_by_discord_id(&discord_id) {
        None => {
            let u = NewUser { is_bot: author.bot, discord_id };
            let result = user_controller.insert(&u);
            result
        }
        Some(a) => a
    };
    author_user
}

pub fn get_from_database_or_create_bot_user<'a>(user_controller: &UserController<'a>, ctx: &Context<'a>) -> User {
    let discord_id = i64::try_from(ctx.cache().current_user().id.get()).expect("Overflow i64 on discord_id");
    let is_bot = ctx.cache().current_user().bot;
    let user = match user_controller.get_by_discord_id(&discord_id) {
        Some(u) => u,
        None => {
            let u = NewUser { discord_id, is_bot };
            let result = user_controller.insert(&u);
            result
        }
    };
    user
}

pub fn retrieve_conversation(user_conversation_controller: &UserConversationController, author: &User) -> Conversation {
    let last_conversation_with_user = user_conversation_controller.get_last_conversation(author);
    match last_conversation_with_user {
        None => {
            let conversation = user_conversation_controller.new_conversation(author);
            conversation
        },
        Some(c) => c
    }
}
