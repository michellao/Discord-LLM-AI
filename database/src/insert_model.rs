use diesel::Insertable;

#[derive(Insertable)]
#[diesel(table_name = crate::schema::user_llm)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub is_bot: bool,
    pub discord_id: i64
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::message)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMessage<'a> {
    pub user_id: &'a i64,
    pub content: &'a str,
    pub conversation_id: &'a i64
}