use serde::{Deserialize, Serialize};

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
            DataType::User => String::from("user")
        }
    }
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    pub id_user: Option<u64>,
    pub is_bot: Option<bool>,
    pub discord_id: Option<u64>
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Message {
    pub id_message: Option<u64>,
    pub user_id: Option<User>,
    pub content: Option<String>
}

impl Model for Message {
    fn to_table_name(&self) -> String {
        String::from("message")
    }

    fn get_primary_key_name(&self) -> String {
        String::from("id_message")
    }

    fn get_id(&self) -> u64 {
        self.id_message.unwrap_or_else(|| {
            0
        })
    }
}

impl Model for User {
    fn to_table_name(&self) -> String {
        String::from("user")
    }

    fn get_primary_key_name(&self) -> String {
        String::from("id_user")
    }

    fn get_id(&self) -> u64 {
        self.id_user.unwrap_or_else(|| {
            0
        })
    }
}

pub trait Model {
    fn to_table_name(&self) -> String;
    fn get_primary_key_name(&self) -> String;
    fn get_id(&self) -> u64;
}
