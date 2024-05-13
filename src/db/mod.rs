use rusqlite::Connection;

pub struct Database {
    conn: Connection
}

impl Database {
    pub fn new() -> Self {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE user_ai (
                user_id INTEGER PRIMARY KEY,
            );",
            ()
        ).unwrap();
        conn.execute(
            "CREATE TABLE message (
                id_message INTEGER PRIMARY KEY,
                content TEXT NOT NULL
            )",
            ()
        ).unwrap();
        Self {
            conn
        }
    }

    pub fn delete(&self) {
        self.conn.execute("
            DROP TABLE message;
            ",
            ()
        ).unwrap();
    }
}
