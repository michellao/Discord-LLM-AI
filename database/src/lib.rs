pub mod model;
mod format_sql;
use format_sql::FormatSql;
use model::Model;
use sqlx::{Pool, Postgres};
use serde::Serialize;

pub struct Database<'a> {
    conn: &'a Pool<Postgres>,
}

impl<'a> Database<'a> {
    pub async fn new(conn: &'a Pool<Postgres>) -> Self {
        sqlx::raw_sql(
            "DROP TABLE IF EXISTS message;
            DROP TABLE IF EXISTS user_llm;",
        ).execute(conn).await.unwrap();
        sqlx::raw_sql(
            "CREATE TABLE IF NOT EXISTS user_llm (
                id_user SERIAL PRIMARY KEY,
                is_bot BOOLEAN NOT NULL DEFAULT FALSE,
                discord_id INTEGER NOT NULL
            );"
        ).execute(conn).await.unwrap();
        sqlx::raw_sql(
            "CREATE TABLE IF NOT EXISTS message (
                id_message SERIAL PRIMARY KEY,
                user_id INTEGER,
                content TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES user_llm (id_user)
            )"
        ).execute(conn).await.unwrap();
        Self {
            conn
        }
    }

    pub async fn select_from_id<T: Serialize + Model>(&mut self, object: &T) {
        let format_sql = FormatSql::new(object);
        let rows_string = format_sql.format_rows_select();
        let sql = format!("SELECT {} FROM {} WHERE {} = {}", rows_string, object.to_data_type().to_string(), object.get_primary_key_name(), object.get_id());
        let result = format_sql.query_sql(&self.conn, &sql).await;
    }

    pub async fn insert<T: Serialize + Model>(&mut self, object: &T) -> bool {
        let format_sql = FormatSql::new(object);
        let row_names = format_sql.format_rows_insert();
        let insert_values = format_sql.format_sql_set_placeholder();
        let sql = format!("INSERT INTO {} ({}) VALUES ({})", object.to_data_type().to_string(), row_names, insert_values);
        let result = format_sql.execute_sql(&self.conn, &sql).await;
        match result {
            Err(e) => {
                println!("{}", e);
                return false;
            },
            Ok(r) => r.rows_affected() > 0
        }
    }

    pub async fn update<T: Serialize + Model>(&mut self, object: &T) -> bool {
        let format_sql = FormatSql::new(object);
        let format_sql_key = format_sql.format_sql_key_value();
        let sql = format!("UPDATE {} SET {} WHERE {} = {}", object.to_data_type().to_string(), format_sql_key, object.get_primary_key_name(), object.get_id());
        let result = format_sql.execute_sql(&self.conn, &sql).await;
        match result {
            Err(_) => false,
            Ok(r) => r.rows_affected() > 0
        }
    }

    pub async fn delete_object<T: Serialize + Model>(&mut self, object: &T) -> bool {
        let format_sql = FormatSql::new(object);
        let sql = format!("DELETE FROM {} WHERE {} = {}", object.to_data_type().to_string(), object.get_primary_key_name(), object.get_id());
        let result = format_sql.execute_sql(&self.conn, &sql).await;
        match result {
            Err(_) => false,
            Ok(r) => r.rows_affected() > 0
        }
    }
}
