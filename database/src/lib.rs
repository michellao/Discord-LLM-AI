pub mod model;
mod format_sql;
use format_sql::FormatSql;
use model::{Model, DataType};
use sqlx::{Pool, Sqlite};
use serde::Serialize;

pub struct Database {
    conn: Pool<Sqlite>,
}

impl Database {
    pub async fn new(conn: Pool<Sqlite>) -> Self {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS user (
                id_user INTEGER PRIMARY KEY,
                is_bot INTEGER NOT NULL DEFAULT FALSE,
                discord_id INTEGER DEFAULT NULL
            );"
        ).execute(&conn).await.unwrap();
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS message (
                id_message INTEGER PRIMARY KEY,
                user_id INTEGER,
                content TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES user(id_user)
            )"
        ).execute(&conn).await.unwrap();
        Self {
            conn
        }
    }

    pub fn select_from_id<T: Serialize + Model>(&mut self, table_name: DataType, id: u64) {
        let rows_string = String::from("*");
        let sql = format!("SELECT {} FROM {};", rows_string, table_name.to_string());

    }

    pub async fn insert<T: Serialize + Model>(&mut self, object: &T) -> bool {
        let format_sql = FormatSql::new(object);
        let row_names = format_sql.format_rows_select();
        let insert_values = format_sql.format_sql_set_placeholder();
        let sql = format!("INSERT INTO {} ({}) VALUES ({})", object.to_table_name(), row_names, insert_values);
        let result = format_sql.execute_sql(&self.conn, &sql).await;
        match result {
            Err(_) => false,
            Ok(r) => r.rows_affected() > 0
        }
    }

    pub async fn update<T: Serialize + Model>(&mut self, object: &T) -> bool {
        let format_sql = FormatSql::new(object);
        let format_sql_key = format_sql.format_sql_key_value();
        let sql = format!("UPDATE {} SET {} WHERE {} = {}", object.to_table_name(), format_sql_key, object.get_primary_key_name(), object.get_id());
        let result = format_sql.execute_sql(&self.conn, &sql).await;
        match result {
            Err(_) => false,
            Ok(r) => r.rows_affected() > 0
        }
    }

    pub async fn delete_object<T: Serialize + Model>(&mut self, object: &T) -> bool {
        let format_sql = FormatSql::new(object);
        let sql = format!("DELETE FROM {} WHERE {} = {}", object.to_table_name(), object.get_primary_key_name(), object.get_id());
        let result = format_sql.execute_sql(&self.conn, &sql).await;
        match result {
            Err(_) => false,
            Ok(r) => r.rows_affected() > 0
        }
    }
}
