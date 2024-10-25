-- This file should undo anything in `up.sql`
DROP TABLE subscribe_channel;

SELECT cron.unschedule('cleanup-on-expire');