use crate::{insert_model::NewUser, model::User, Database};
use super::Controller;
use diesel::prelude::*;

pub struct UserController<'a> {
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

impl<'a> Controller<'a> for UserController<'a> {
    type ModelController = User;
    type InsertionModel = NewUser<'a>;

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

    fn insert(&mut self, model: &Self::InsertionModel) -> Self::ModelController {
        use crate::schema::user_llm;
        let connection = self.database.get_connection();
        let result = diesel::insert_into(user_llm::table)
            .values(model)
            .returning(User::as_returning())
            .get_result(connection)
            .expect("Error saving new user");
        result
    }
}