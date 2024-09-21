use diesel::PgConnection;

pub mod model;
pub mod schema;

pub struct Database {
    pub conn: PgConnection,
}

impl Database {
    pub async fn new(conn: PgConnection) -> Self {
        Self {
            conn
        }
    }
}
