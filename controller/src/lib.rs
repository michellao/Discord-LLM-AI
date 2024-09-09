use database::model::{Model, User, Message};

pub struct UserController;
pub struct MessageController;
pub trait Controller {
    fn new(id: Option<u64>) -> Self;
    fn get(&self, id: u64) -> Box<dyn Model>;
    fn create(&self, model: &impl Model) -> bool;
}

impl Controller for UserController {
    fn new(id: Option<u64>) -> Self {
        UserController
    }

    fn get(&self, id: u64) -> Box<dyn Model> {
        Box::new(User {id_user: 43, discord_id: 43, is_bot: false})
    }

    fn create(&self, model: &impl Model) -> bool {
        false
    }
}

impl Controller for MessageController {
    fn new(id: Option<u64>) -> Self {
        MessageController
    }

    fn get(&self, id_message: u64) -> Box<dyn Model> {
        Box::new(Message { id_message: 43, user: User { id_user: 43, discord_id: 43, is_bot: false }, content: String::from("gdfgfdg") })
    }

    fn create(&self, model: &impl Model) -> bool {
        false
    }
}
