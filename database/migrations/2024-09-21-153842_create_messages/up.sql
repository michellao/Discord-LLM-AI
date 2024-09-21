-- Your SQL goes here
CREATE TABLE message (
    id_message BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id BIGINT,
    conversation_id BIGINT,
    content TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user_llm (id_user),
    FOREIGN KEY (conversation_id) REFERENCES conversation (id_conversation)
);