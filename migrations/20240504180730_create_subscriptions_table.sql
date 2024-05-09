-- Add migration script here

CREATE TABLE subscriptions
(
    PRIMARY KEY (id),
    id            uuid        NOT NULL,
    email         TEXT        NOT NULL UNIQUE,
    name          TEXT        NOT NULL,
    subscribed_at timestamptz NOT NULL
);