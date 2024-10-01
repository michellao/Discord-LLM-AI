use crate::{model::Conversation, Database};
use super::Controller;
use diesel::prelude::*;

pub struct ConversationController<'a> {
    database: &'a mut Database
}

impl<'a> ConversationController<'a> {
}

impl<'a> Controller<'a> for ConversationController<'a> {
    type ModelController = Conversation;
    type InsertionModel = Conversation;

    fn new(database: &'a mut Database) -> Self {
        Self {
            database
        }
    }

    fn get(&mut self, id: i64) -> Option<Self::ModelController> {
        use crate::schema::conversation::dsl::*;
        let connection = self.database.get_connection();
        let result = conversation
            .filter(id_conversation.eq(id))
            .select(Conversation::as_select())
            .first(connection);
        match result {
            Ok(m) => Some(m),
            Err(_) => None
        }
    }

    fn insert(&mut self, model: &Self::InsertionModel) -> Self::ModelController {
        use crate::schema::conversation;
        let connection = self.database.get_connection();
        let result = diesel::insert_into(conversation::table)
            .values(model)
            .returning(Conversation::as_returning())
            .get_result(connection)
            .expect("Error saving new user");
        result
    }
}