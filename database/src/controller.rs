use crate::model::*;
use crate::Database;
use diesel::prelude::*;

pub struct UserController<'a> {
    database: &'a mut Database
}
pub struct MessageController<'a> {
    database: &'a mut Database
}

impl<'a> UserController<'a> {
    pub fn get_by_discord_id(&mut self, disco_id: i64) -> Option<User> {
        use crate::schema::user_llm::dsl::*;
        let connection = self.database.get_connection();
        let result = user_llm
            .filter(discord_id.eq(disco_id))
            .select(User::as_select())
            .first(connection);
        match result {
            Ok(u) => Some(u),
            Err(_) => None
        }
    }
}

impl<'a> MessageController<'a> {
    pub fn get_by_user(&mut self, user: &User) -> Vec<Message> {
        use crate::schema::message::dsl::*;
        let connection = self.database.get_connection();
        let result = message
            .filter(user_id.eq(user.id()))
            .select(Message::as_select())
            .load(connection);
        result.unwrap_or_else(|_| vec![])
    }

    pub fn delete_messages_by_user(&mut self, user: &User) -> bool {
        use crate::schema::message::dsl::*;
        let connection = self.database.get_connection();
        let result = diesel::delete(message.filter(user_id.eq(user.id())))
            .execute(connection);
        match result {
            Err(_) => false,
            Ok(s) => s > 0
        }
    }
}

pub trait Controller<'a> {
    type ModelController;
    fn new(database: &'a mut Database) -> Self;
    fn get(&mut self, id: i64) -> Option<Self::ModelController>;
}

impl<'a> Controller<'a> for UserController<'a> {
    type ModelController = User;

    fn new(database: &'a mut Database) -> Self {
        UserController {
            database
        }
    }

    fn get(&mut self, id: i64) -> Option<Self::ModelController> {
        use crate::schema::user_llm::dsl::*;
        let connection = self.database.get_connection();
        let result = user_llm
            .filter(id_user.eq(id))
            .select(User::as_select())
            .first(connection);
        match result {
            Ok(u) => Some(u),
            Err(_) => None
        }
    }
}

impl<'a> Controller<'a> for MessageController<'a> {
    type ModelController = Message;

    fn new(database: &'a mut Database) -> Self {
        MessageController {
            database
        }
    }

    fn get(&mut self, id: i64) -> Option<Self::ModelController> {
        use crate::schema::message::dsl::*;
        let connection = self.database.get_connection();
        let result = message
            .filter(id_message.eq(id))
            .select(Message::as_select())
            .first(connection);
        match result {
            Ok(m) => Some(m),
            Err(_) => None
        }
    }
}
