use crate::{model::SubscribeChannel, Database};
use super::Controller;
use diesel::prelude::*;

pub struct SubscribeChannelController<'a> {
    database: &'a Database
}

impl<'a> Controller<'a> for SubscribeChannelController<'a> {
    type ModelController = SubscribeChannel;

    type InsertionModel = SubscribeChannel;

    fn new(database: &'a Database) -> Self {
        Self {
            database
        }
    }

    fn get(&self, id: i64) -> Option<Self::ModelController> {
        use crate::schema::subscribe_channel::dsl::*;
        let connection = &mut self.database.get_connection();
        let result = subscribe_channel
            .find(id)
            .first(connection);
        match result {
            Ok(sc) => Some(sc),
            Err(_) => None
        }
    }

    fn insert(&self, model: &Self::InsertionModel) -> Self::ModelController {
        use crate::schema::subscribe_channel;
        let connection = &mut self.database.get_connection();
        let result = diesel::insert_into(subscribe_channel::table)
            .values(model)
            .returning(SubscribeChannel::as_returning())
            .get_result(connection)
            .expect("Error saving subscribe channel");
        result
    }

    fn delete(&self, model: &Self::ModelController) -> bool {
        use crate::schema::subscribe_channel::dsl::*;
        let connection = &mut self.database.get_connection();
        let r = diesel::delete(subscribe_channel.find(model.id()))
            .execute(connection)
            .expect("Error deleting subscribe channel");
        r > 0
    }
}