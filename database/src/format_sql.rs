use serde::Serialize;
use serde_json::{Map, Number, Value};
use sqlx::{postgres::PgRow, Error, Pool, Postgres};
use crate::model::{DataType, Message, Model, User};

pub struct FormatSql<'a, T> {
    model: &'a T,
    object: Map<String, Value>,
}

impl<'a, T: Serialize + Model> FormatSql<'a, T> {
    pub fn new(object: &'a T) -> Self {
        let serialize = serde_json::to_value(object).unwrap();
        let serialize_object = serialize.as_object().expect("Is normally a model");
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

    pub fn format_rows_insert(&self) -> String {
        let mut format = String::from("");
        for (i, (k, v)) in self.object.iter().enumerate() {
            if !v.is_null() {
                if i < self.object.len() && i > 0 {
                    format.push_str(", ");
                }
                format.push_str(k.as_str());
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

    fn convert_value(value: &Value) -> Value {
        match value {
            Value::Object(_) => {
                let model = Self::object_to_model(value).expect("Error because it doesn't implement a struct");
                let res = Value::Number(Number::from(model.get_id()));
                return res;
            }
            _ => value.to_owned()
        }
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
        let mut diff = 0;
        for (i, (k, v)) in self.object.iter().enumerate() {
            if !v.is_null() {
                if i < self.object.len() && i > 0 {
                    sql_format.push_str(", ");
                }
                sql_format.push_str(format!("{} = ${}", k, i + 1 - diff).as_str());
            } else {
                diff += 1;
            }
        }
        sql_format
    }

    pub fn format_sql_set_placeholder(&self) -> String {
        let mut sql_format = String::from("");
        let mut diff = 0;
        for (i, (_k, v)) in self.object.iter().enumerate() {
            if !v.is_null() {
                if i < self.object.len() && i > 0 {
                    sql_format.push_str(", ");
                }
                sql_format.push_str(format!("${}", i + 1 - diff).as_str());
            } else {
                diff += 1;
            }
        }
        sql_format
    }

    pub async fn execute_sql(&self, conn: &Pool<Postgres>, sql: &str) -> Result<PgRow, Error> {
        let mut handle: sqlx::query::Query<'_, Postgres, _> = sqlx::query(sql);
        for v in self.object.values() {
            let convert = Self::convert_value(v);
            match convert {
                Value::Number(n) => {
                    if n.is_i64() {
                        let r = n.as_i64();
                        handle = handle.bind(r);
                    } else if n.is_f64() {
                        let r = n.as_f64();
                        handle = handle.bind(r);
                    }
                },
                Value::Null => continue,
                Value::Bool(b) => handle = handle.bind(b),
                _ => handle = handle.bind(convert)
            }
        }
        let result = handle.fetch_one(conn).await;
        result
    }
}
