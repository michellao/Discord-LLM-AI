// @generated automatically by Diesel CLI.

diesel::table! {
    conversation (id_conversation) {
        id_conversation -> Int8,
    }
}

diesel::table! {
    message (id_message) {
        id_message -> Int8,
        user_id -> Nullable<Int8>,
        conversation_id -> Nullable<Int8>,
        content -> Text,
    }
}

diesel::table! {
    user_conversation (user_id, conversation_id) {
        user_id -> Int8,
        conversation_id -> Int8,
    }
}

diesel::table! {
    user_llm (id_user) {
        id_user -> Int8,
        is_bot -> Bool,
        discord_id -> Int8,
    }
}

diesel::joinable!(message -> conversation (conversation_id));
diesel::joinable!(message -> user_llm (user_id));
diesel::joinable!(user_conversation -> conversation (conversation_id));
diesel::joinable!(user_conversation -> user_llm (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    conversation,
    message,
    user_conversation,
    user_llm,
);
