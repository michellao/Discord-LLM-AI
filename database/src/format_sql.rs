use serde::Serialize;
use serde_json::{Map, Value};
use sqlx::{sqlite::SqliteQueryResult, Error, Row, Sqlite, SqliteConnection};
use crate::model::{Model, User, DataType, Message};

pub struct FormatSql {
    type_object: DataType,
    object: Map<String, Value>,
}

impl FormatSql {
    pub fn new<T: Serialize + Model>(object: &T) -> Self {
        let serialize = serde_json::to_value(object).unwrap();
        let serialize_object = serialize.as_object().unwrap();
        Self { type_object: object.get_type(), object: serialize_object.to_owned() }
    }

    /// For example
    /// ```rust
    /// let u = User { id_user: 5345224344, discord_id: 42342342342353, is_bot: false };
    /// let format_sql = FormatSql::new(u);
    /// let r = format_sql.format_rows_select();
    /// ```
    /// Output :
    ///
    /// `id_user, discord_id, is_bot`
    pub fn format_rows_select(&self) -> String {
        let mut format = String::from("");
        for (i, k) in self.object.keys().enumerate() {
            format.push_str(k.as_str());
            if i < self.object.len() - 1 {
                format.push_str(", ");
            }
        }
        format
    }

    pub fn object_to_model(value: Value) -> Option<Box<dyn Model>> {
        let list_models = DataType::all();
        for type_value in list_models {
            let clone_value = value.clone();
            match type_value {
                DataType::User => {
                    let model = serde_json::from_value::<User>(clone_value);
                    match model {
                        Err(_) => continue,
                        Ok(r) => return Some(Box::new(r))
                    }
                },
                DataType::Message => {
                    let model = serde_json::from_value::<Message>(clone_value);
                    match model {
                        Err(_) => continue,
                        Ok(r) => return Some(Box::new(r))
                    };
                }
            }
        }
        None
    }

    fn format_type_sql(value: &Value) -> String {
        let mut format_sql = String::from("");
        if value.is_string() {
            format_sql = String::from(format!("\"{}\"", value));
        } else if value.is_number() {
            format_sql = String::from(format!("{}", value));
        } else if value.is_boolean() {
            format_sql = String::from(format!("{}", value));
        } else if value.is_object() {
            let model = Self::object_to_model(value.to_owned());
            let primary_key_value = match model {
                None => 0,
                Some(r) => r.get_id()
            };
            format_sql = String::from(format!("{}", primary_key_value));
        }
        format_sql
    }

    /* pub fn format_values_as_params(self) -> Vec<>> {
        let mut params: Vec<String> = Vec::new();
        for value in self.object.values() {
            match value {
                Value::Bool(b) => params.push(Box::new(b)),
                Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        params.push(Box::new(i));
                    } else if let Some(f) = n.as_f64() {
                        params.push(Box::new(f));
                    }
                },
                Value::String(s) => params.push(Box::new(s)),
                _ => (),
            }
        }
        params
    } */

    /// For example
    /// ```rust
    /// let u = User { id_user: 5345224344, discord_id: 42342342342353, is_bot: false };
    /// ```
    /// Output :
    ///
    /// `id_user = $1, discord_id = $2, is_bot = $3`
    pub fn format_sql_key_value(&self) -> String {
        let mut sql_format = String::from("");
        for (i, k) in self.object.keys().enumerate() {
            sql_format.push_str(format!("{} = ${}", k, i + 1).as_str());
            if self.object.len() - 1 > i {
                sql_format.push_str(", ");
            }
        }
        sql_format
    }

    pub fn format_sql_set_placeholder(&self) -> String {
        let mut sql_format = String::from("");
        for (i, _k) in self.object.keys().enumerate() {
            sql_format.push_str(format!("${}", i + 1).as_str());
            if self.object.len() - 1 > i {
                sql_format.push_str(", ");
            }
        }
        sql_format
    }

    pub async fn query_sql(&self, conn: &mut SqliteConnection, sql: &str) {
        let mut handle: sqlx::query::Query<'_, Sqlite, _> = sqlx::query(sql);
        for v in self.object.values() {
            handle = handle.bind(v);
        }
        let result = handle.fetch_all(conn).await;
        match result {
            Err(e) => println!("{}", e),
            Ok(r) => {
                for row in r {
                    println!("{}", row.len());
                }
            }
        }
    }

    pub async fn execute_sql(&self, conn: &mut SqliteConnection, sql: &str) -> Result<SqliteQueryResult, Error> {
        let mut handle: sqlx::query::Query<'_, Sqlite, _> = sqlx::query(sql);
        for v in self.object.values() {
            handle = handle.bind(v);
        }
        let result = handle.execute(conn).await;
        result
    }
}
