use crate::{insert_model::NewMessage, model::{Message, User}, Database};
use super::Controller;
use diesel::prelude::*;
use crate::model::Conversation;

pub struct MessageController<'a> {
    database: &'a Database
}

impl<'a> MessageController<'a> {
    pub fn get_by_user(&self, user: &User) -> Vec<Message> {
        let connection = &mut self.database.get_connection();
        let result = Message::belonging_to(user)
            .select(Message::as_select())
            .get_results(connection)
            .unwrap_or_default();
        result
    }

    pub fn get_by_conversation(&self, conversation: &Conversation) -> Vec<(User, Message)> {
        use crate::schema::user_llm;
        let connection = &mut self.database.get_connection();
        let messages_in_conversation = Message::belonging_to(conversation)
            .inner_join(user_llm::table)
            .select((User::as_select(), Message::as_select()))
            .load::<(User, Message)>(connection)
            .expect("Error loading conversations");
        messages_in_conversation
    }

    pub fn delete_messages_by_user(&self, user: &User) -> bool {
        use crate::schema::message::dsl::*;
        let connection = &mut self.database.get_connection();
        let result = diesel::delete(message.filter(user_id.eq(user.id())))
            .execute(connection);
        match result {
            Err(_) => false,
            Ok(s) => s > 0
        }
    }
}

impl<'a> Controller<'a> for MessageController<'a> {
    type ModelController = Message;
    type InsertionModel = NewMessage<'a>;

    fn new(database: &'a Database) -> Self {
        MessageController {
            database
        }
    }

    fn get(&self, id: i64) -> Option<Self::ModelController> {
        use crate::schema::message::dsl::*;
        let connection = &mut self.database.get_connection();
        let result = message
            .filter(id_message.eq(id))
            .select(Message::as_select())
            .first(connection);
        match result {
            Ok(m) => Some(m),
            Err(_) => None
        }
    }

    fn insert(&self, model: &Self::InsertionModel) -> Self::ModelController {
        use crate::schema::message;
        let connection = &mut self.database.get_connection();
        let result = diesel::insert_into(message::table)
            .values(model)
            .returning(Message::as_returning())
            .get_result(connection)
            .expect("Error saving new user");
        result
    }
}