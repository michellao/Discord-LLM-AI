use database::{model::User, Database};
use sqlx::{Connection, SqliteConnection};

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let pool = SqliteConnection::connect("sqlite::memory:").await?;
    let mut database = Database::new(pool).await;
    let mut user = User { id_user: 5345224344, discord_id: 42342342342353, is_bot: false };
    // let message = Message { id_message: 432, user, content: String::from("gdfgfdg") };
    let success_insert = database.insert(&user).await;
    println!("{}", success_insert);
    Ok(())
}
