use crate::{insert_model::NewMessage, model::{Message, User}, Database};
use super::Controller;
use diesel::prelude::*;

pub struct MessageController<'a> {
    database: &'a Database
}

impl<'a> MessageController<'a> {
    pub fn get_by_user(&self, user: &User) -> Vec<Message> {
        use crate::schema::message::dsl::*;
        let connection = &mut self.database.get_connection();
        let result = message
            .filter(user_id.eq(user.id()))
            .select(Message::as_select())
            .load(connection);
        result.unwrap_or_else(|_| vec![])
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