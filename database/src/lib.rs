use diesel::prelude::*;

pub mod model;
pub mod schema;
pub mod controller;
pub mod insert_model;

pub struct Database {
    database_url: String,
}

impl Database {
    pub fn new(database_url: String) -> Self {
        Self {
            database_url
        }
    }

    pub fn get_connection(&self) -> PgConnection {
        PgConnection::establish(&self.database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", self.database_url))
    }
}
