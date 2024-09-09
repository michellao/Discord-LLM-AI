use serde::Serialize;
use serde_json::{Map, Number, Value};
use sqlx::{sqlite::SqliteQueryResult, Error, Pool, Sqlite};
use crate::model::{DataType, Message, Model, User};

pub struct FormatSql<'a, T> {
    model: &'a T,
    object: Map<String, Value>,
}

impl<'a, T: Serialize + Model> FormatSql<'a, T> {
    pub fn new(object: &'a T) -> Self {
        let serialize = serde_json::to_value(object).unwrap();
        let serialize_object = serialize.as_object().unwrap();
        Self { model: object, object: serialize_object.to_owned() }
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

    pub fn object_to_model(value: &Value) -> Option<Box<dyn Model>> {
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

    fn convert_object_if_necessary_to_id(value: &Value) -> Value {
        if value.is_object() {
            let model = Self::object_to_model(value).expect("Error because it doesn't implement a struct");
            let res = Value::Number(Number::from(model.get_id()));
            return res;
        }
        value.to_owned()
    }

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

    pub async fn query_sql(&self, conn: &Pool<Sqlite>, select_sql: &str) {
        let mut handle: sqlx::query::Query<'_, Sqlite, _> = sqlx::query(select_sql);
        
        let result = handle.fetch_one(conn).await;
        match result {
            Err(e) => println!("Query data error: {}", e),
            Ok(r) => {
                
            }
        }
    }

    pub async fn execute_sql(&self, conn: &Pool<Sqlite>, sql: &str) -> Result<SqliteQueryResult, Error> {
        let mut handle: sqlx::query::Query<'_, Sqlite, _> = sqlx::query(sql);
        for v in self.object.values() {
            let convert = Self::convert_object_if_necessary_to_id(v);
            handle = handle.bind(convert);
        }
        let result = handle.execute(conn).await;
        result
    }
}
