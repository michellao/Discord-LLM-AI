-- Your SQL goes here
CREATE TABLE user_conversation (
    user_id BIGINT,
    conversation_id BIGINT,
    PRIMARY KEY (user_id, conversation_id),
    FOREIGN KEY (user_id) REFERENCES user_llm (id_user),
    FOREIGN KEY (conversation_id) REFERENCES conversation (id_conversation)
);