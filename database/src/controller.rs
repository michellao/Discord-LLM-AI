use crate::Database;
pub mod message_controller;
pub mod user_controller;
pub mod conversation_controller;
pub mod user_conversation_controller;
pub mod subscribe_channel_controller;

pub trait Controller<'a> {
    type ModelController;
    type InsertionModel;
    fn new(database: &'a Database) -> Self;
    fn get(&self, id: i64) -> Option<Self::ModelController>;
    fn insert(&self, model: &Self::InsertionModel) -> Self::ModelController;
    fn delete(&self, model: &Self::ModelController) -> bool;
}
