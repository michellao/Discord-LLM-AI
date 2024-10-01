use diesel::prelude::*;

#[derive(Debug, Queryable, Identifiable, Selectable, PartialEq)]
#[diesel(table_name = crate::schema::user_llm)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id_user))]
pub struct User {
    pub id_user: i64,
    pub is_bot: bool,
    pub discord_id: i64
}

#[derive(Debug, Queryable, Identifiable, Selectable, Insertable, PartialEq)]
#[diesel(table_name = crate::schema::conversation)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id_conversation))]
pub struct Conversation {
    pub id_conversation: Option<i64>
}

#[derive(Debug, Queryable, Identifiable, Selectable, Insertable, Associations, PartialEq)]
#[diesel(table_name = crate::schema::user_conversation)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Conversation))]
#[diesel(primary_key(user_id, conversation_id))]
pub struct UserConversation {
    pub user_id: i64,
    pub conversation_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Selectable, Associations, PartialEq)]
#[diesel(table_name = crate::schema::message)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Conversation))]
#[diesel(primary_key(id_message))]
pub struct Message {
    pub id_message: i64,
    pub user_id: i64,
    pub content: String,
    pub conversation_id: i64
}
