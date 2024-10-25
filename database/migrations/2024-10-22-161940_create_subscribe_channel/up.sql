-- Your SQL goes here
CREATE TABLE subscribe_channel (
    discord_channel_id BIGINT PRIMARY KEY,
    expire_in TIMESTAMP NOT NULL
);

-- Need to have pg_cron enabled
SELECT cron.schedule (
    'cleanup-on-expire',
    '30 16 * * *',
    $$ DELETE FROM subscribe_channel WHERE expire_in < NOW() $$
);
