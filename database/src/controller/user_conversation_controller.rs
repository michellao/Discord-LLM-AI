use crate::{model::{Conversation, User, UserConversation}, Database};
use diesel::prelude::*;

use super::{conversation_controller::ConversationController, Controller};

pub struct UserConversationController<'a> {
    database: &'a mut Database
}

impl<'a> UserConversationController<'a> {
    pub fn new(database: &'a mut Database) -> Self {
        Self {
            database
        }
    }

    pub fn insert(&mut self, model: &UserConversation) -> UserConversation {
        use crate::schema::user_conversation;
        let connection = self.database.get_connection();
        let result = diesel::insert_into(user_conversation::table)
            .values(model)
            .returning(UserConversation::as_returning())
            .get_result(connection)
            .expect("Error saving new user");
        result
    }

    pub fn new_conversation(&mut self, user: &User) -> Conversation {
        let new_conversation = Conversation { id_conversation: None };
        let mut conversation_controller = ConversationController::new(&mut self.database);
        let conversation = conversation_controller.insert(&new_conversation);
        let conversation_user = UserConversation { conversation_id: conversation.id_conversation.expect("Error saving conversation to database"), user_id: user.id_user };
        self.insert(&conversation_user);
        conversation
    }

    pub fn get_last_conversation(&mut self, user: &User) -> Option<Conversation> {
        use crate::schema::conversation;
        let connection = self.database.get_connection();
        let result = UserConversation::belonging_to(user)
            .inner_join(conversation::table)
            .select(Conversation::as_select())
            .first(connection)
            .optional()
            .expect("Error reading user_conversation");
        result
    }
}