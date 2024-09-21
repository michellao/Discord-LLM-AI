use diesel::prelude::*;
use crate::schema::*;

#[derive(Debug, Queryable, Selectable, Insertable, PartialEq)]
#[diesel(table_name = user_llm)]
pub struct User {
    pub id_user: Option<i64>,
    pub is_bot: Option<bool>,
    pub discord_id: i64
}

#[derive(Debug, Queryable, Selectable, Insertable, PartialEq)]
#[diesel(table_name = conversation)]
pub struct Conversation {
    pub id_conversation: i64
}

#[derive(Debug, Queryable, Selectable, Associations, Insertable, PartialEq)]
#[diesel(table_name = user_conversation)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Conversation))]
pub struct UserConversation {
    pub user_id: i64,
    pub conversation_id: i64,
}

#[derive(Debug, Queryable, Selectable, Associations, Insertable, PartialEq)]
#[diesel(table_name = message)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Conversation))]
pub struct Message {
    pub id_message: Option<i64>,
    pub user_id: i64,
    pub content: String,
    pub conversation_id: i64
}
