use rusqlite::Connection;

pub struct Database {
    conn: Connection
}

impl Database {
    pub fn new() -> Self {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE user (
                id_user INTEGER PRIMARY KEY,
                is_bot INTEGER NOT NULL DEFAULT FALSE,
                discord_id INTEGER DEFAULT NULL
            );",
            ()
        ).unwrap();
        conn.execute(
            "CREATE TABLE message (
                id_message INTEGER PRIMARY KEY,
                user_id INTEGER,
                content TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES user(id_user)
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
