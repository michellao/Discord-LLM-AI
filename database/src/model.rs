use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};

pub enum DataType {
    User,
    Message
}

impl DataType {
    pub fn all() -> Vec<Self> {
        vec![Self::User, Self::Message]
    }
}

impl ToString for DataType {
    fn to_string(&self) -> String {
        match self {
            DataType::Message => String::from("message"),
            DataType::User => String::from("user_llm")
        }
    }
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow, PartialEq, Eq)]
pub struct User {
    pub id_user: Option<i64>,
    pub is_bot: Option<bool>,
    pub discord_id: i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id_message: Option<i64>,
    pub user_id: User,
    pub content: String
}

impl Model for Message {
    fn to_data_type(&self) -> DataType {
        DataType::Message
    }

    fn get_primary_key_name(&self) -> String {
        String::from("id_message")
    }

    fn get_id(&self) -> i64 {
        self.id_message.unwrap_or_else(|| {
            0
        })
    }

    fn set_id(&mut self, new_id: i64) {
        self.id_message = Some(new_id);
    }
}

impl Model for User {
    fn to_data_type(&self) -> DataType {
        DataType::User
    }

    fn get_primary_key_name(&self) -> String {
        String::from("id_user")
    }

    fn get_id(&self) -> i64 {
        self.id_user.unwrap_or_else(|| {
            0
        })
    }

    fn set_id(&mut self, new_id: i64) {
        self.id_user = Some(new_id);
    }
}

pub trait Model {
    fn to_data_type(&self) -> DataType;
    fn get_primary_key_name(&self) -> String;
    fn get_id(&self) -> i64;
    fn set_id(&mut self, new_id: i64);
}

impl Default for Message {
    fn default() -> Self {
        Self { id_message: Default::default(), user_id: Default::default(), content: Default::default() }
    }
}

impl Default for User {
    fn default() -> Self {
        Self { id_user: Default::default(), is_bot: Default::default(), discord_id: Default::default() }
    }
}

impl FromRow<'_, PgRow> for Message {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id_message: row.try_get("id_message")?,
            user_id: User { id_user: row.try_get("user_id")?, ..Default::default() },
            content: row.try_get("content")?,
        })
    }
}