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

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id_user: u64,
    pub is_bot: bool,
    pub discord_id: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id_message: u64,
    pub user: User,
    pub content: String
}

impl Model for Message {
    fn get_type(&self) -> DataType {
        DataType::Message
    }

    fn to_table_name(&self) -> String {
        String::from("message")
    }

    fn get_primary_key_name(&self) -> String {
        String::from("id_message")
    }

    fn get_id(&self) -> u64 {
        self.id_message
    }
}

impl Model for User {
    fn get_type(&self) -> DataType {
        DataType::User
    }

    fn to_table_name(&self) -> String {
        String::from("user")
    }

    fn get_primary_key_name(&self) -> String {
        String::from("id_user")
    }

    fn get_id(&self) -> u64 {
        self.id_user
    }
}

pub trait Model {
    fn get_type(&self) -> DataType;
    fn to_table_name(&self) -> String;
    fn get_primary_key_name(&self) -> String;
    fn get_id(&self) -> u64;
}
