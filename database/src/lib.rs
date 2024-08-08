use std::{fmt::format, vec};

use rusqlite::Connection;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(conn: Connection) -> Self {
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

    fn transform_rows_to_string(table_names: &Vec<&str>) -> String {
        let mut rows_string = String::from("");
        for i in 0..table_names.capacity() {
            let string: String;
            if table_names.capacity() - 1 > i {
                string = format!("{}, ", table_names[i]);
            } else {
                string = format!("{}", table_names[i]);
            }
            rows_string.push_str(&string);
        }
        rows_string
    }

    pub fn select(&self, object: &impl Data) {
        let row_names = object.to_obj();
        let rows_string = Self::transform_rows_to_string(&row_names);
        let sql = format!("SELECT {} FROM {};", rows_string, object.to_table_name());
        println!("{sql}");
        let result = self.conn.execute(
            &sql,
            ()
        );
        println!("{:?}", result);
    }

    pub fn insert(&self, object: &impl Data) {
        let row_names = object.to_obj();
        let sql = format!("INSERT INTO {} VALUES ()", object.to_table_name());
        let result = self.conn.execute(
            &sql,
        ()
        );
    }

    pub fn delete(&self, object: &impl Data, primary_id: &i64) -> bool {
        let first_row = object.to_obj()[0];
        let sql = format!("DELETE FROM {} WHERE {} = {}", object.to_table_name(), first_row, primary_id);
        let result = self.conn.execute(
            &sql,
            ()
        );
        match result {
            Err(_) => false,
            Ok(value) => value > 0
        }
    }
}

#[derive(Debug)]
pub struct User {
    pub id_user: u64,
    pub is_bot: bool,
    pub discord_id: u64
}

#[derive(Debug)]
pub struct Message {
    pub id_message: u64,
    pub user: User,
    pub content: String
}

pub trait Data {
    fn to_table_name<'a>(&self) -> &'a str;
    fn to_obj(&self) -> Vec<&str>;
}

impl Data for User {

    fn to_table_name<'a>(&self) -> &'a str {
        "user"
    }
    
    fn to_obj(&self) -> Vec<&str> {
        vec!["id_user", "is_bot" , "discord_id"]
    }
}

impl Data for Message {

    fn to_table_name<'a>(&self) -> &'a str {
        "message"
    }
    
    fn to_obj(&self) -> Vec<&str> {
        vec!["id_message", "user", "content"]
    }
}