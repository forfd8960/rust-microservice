-- Add migration script here
CREATE TYPE gender AS ENUM('female', 'male', 'unknown');

CREATE TABLE user_stats(
    email varchar(128) NOT NULL PRIMARY KEY,
    name varchar(64) NOT NULL,
    gender gender DEFAULT 'unknown',
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    last_visited_at timestamptz NOT NULL,
    last_watched_at timestamptz NOT NULL,
    recent_watched int [],
    viewed_but_not_started int [],
    started_but_not_finished int [],
    finished int [],
    last_email_notification timestamptz NOT NULL,
    last_in_app_notification timestamptz NOT NULL,
    last_sms_notification timestamptz NOT NULL
);