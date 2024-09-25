use crate::Database;
pub mod message_controller;
pub mod user_controller;

pub trait Controller<'a> {
    type ModelController;
    type InsertionModel;
    fn new(database: &'a mut Database) -> Self;
    fn get(&mut self, id: i64) -> Option<Self::ModelController>;
    fn insert(&mut self, model: Self::InsertionModel) -> Self::ModelController;
}
