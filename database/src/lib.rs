use diesel::PgConnection;

pub mod model;
pub mod schema;
pub mod controller;

pub struct Database {
    conn: PgConnection,
}

impl Database {
    pub fn new(conn: PgConnection) -> Self {
        Self {
            conn
        }
    }

    pub fn get_connection(&mut self) -> &mut PgConnection {
        &mut self.conn
    }
}
