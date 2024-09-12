use std::env;

use database::{model::{Message, User}, Database};
use serde_json::{Number, Value};
use sqlx::postgres::PgPoolOptions;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(env::var("POSTGRES_URI").unwrap().as_str()).await?;
    let mut database = Database::new(&pool).await;
    let user = User { id_user: None, discord_id: 342353, is_bot: Some(true) };
    // let message = Message { id_message: 432, user, content: String::from("gdfgfdg") };
    let success_insert = database.insert(&user).await;
    println!("User insert: {}", success_insert);
    /* user.is_bot = Some(true);
    let success_update = database.update(&user).await;
    println!("User update: {}", success_update);
    let message = Message { id_message: Some(1), user_id: Some(user), content: Some(String::from("value")) };
    let success_insert = database.insert(&message).await; 
    println!("Message insert: {}", success_insert); */
    /* let search_user = User { id_user: Some(3), ..Default::default() };
    let success_select = database.select_from_id(&search_user).await; */
    /* let exist_user: User = sqlx::query_as("SELECT id_user, is_bot, discord_id FROM user WHERE id_user = $1")
        .bind(Value::Number(Number::from(3)))
        .fetch_one(&pool).await.expect("Error awaiting");
    println!("{:?}", exist_user); */
    Ok(())
}
