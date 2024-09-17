use database::{ model::{Message, User}, Database };

pub struct UserController<'a> {
    database: &'a mut Database
}
pub struct MessageController<'a> {
    database: &'a mut Database
}
pub trait Controller<'a> {
    type ModelController;
    fn new(database: &'a mut Database) -> Self;
    fn get(&self, id: i64) -> impl std::future::Future<Output = Option<Self::ModelController>> + Send;
}

impl<'a> Controller<'a> for UserController<'a> {
    type ModelController = User;

    fn new(database: &'a mut Database) -> Self {
        UserController {
            database
        }
    }

    async fn get(&self, id: i64) -> Option<Self::ModelController> {
        let object = Self::ModelController { id_user: Some(id), ..Default::default() };
        let sql = self.database.select_from_id(&object);
        let result: Option<Self::ModelController> = sqlx::query_as(&sql)
            .fetch_optional(&self.database.conn)
            .await.expect("Database error");
        result
    }
}

impl<'a> Controller<'a> for MessageController<'a> {
    type ModelController = Message;

    fn new(database: &'a mut Database) -> Self {
        MessageController {
            database
        }
    }

    async fn get(&self, id: i64) -> Option<Self::ModelController> {
        let object = Self::ModelController { id_message: Some(id), ..Default::default() };
        let sql = self.database.select_from_id(&object);
        let result: Option<Self::ModelController> = sqlx::query_as(&sql)
            .fetch_optional(&self.database.conn)
            .await.expect("Database error");
        result
    }
}
