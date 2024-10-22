use crate::{insert_model::NewUser, model::User, Database};
use super::Controller;
use diesel::prelude::*;

pub struct UserController<'a> {
    database: &'a Database
}

impl<'a> UserController<'a> {
    pub fn get_by_discord_id(&self, discord: &i64) -> Option<User> {
        use crate::schema::user_llm::dsl::*;
        let connection = &mut self.database.get_connection();
        let result = user_llm
            .filter(discord_id.eq(discord))
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
    type InsertionModel = NewUser;

    fn new(database: &'a Database) -> Self {
        Self {
            database
        }
    }

    fn get(&self, id: i64) -> Option<Self::ModelController> {
        use crate::schema::user_llm::dsl::*;
        let connection = &mut self.database.get_connection();
        let result = user_llm
            .filter(id_user.eq(id))
            .select(User::as_select())
            .first(connection);
        match result {
            Ok(u) => Some(u),
            Err(_) => None
        }
    }

    fn insert(&self, model: &Self::InsertionModel) -> Self::ModelController {
        use crate::schema::user_llm;
        let connection = &mut self.database.get_connection();
        let result = diesel::insert_into(user_llm::table)
            .values(model)
            .returning(User::as_returning())
            .get_result(connection)
            .expect("Error saving new user");
        result
    }

    fn delete(&self, model: &Self::ModelController) -> bool {
        use crate::schema::user_llm::dsl::*;
        let connection = &mut self.database.get_connection();
        let r = diesel::delete(user_llm.find(model.id()))
            .execute(connection)
            .expect("Error deleting user");
        r > 0
    }
}