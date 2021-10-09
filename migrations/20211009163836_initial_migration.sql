-- Add migration script here
CREATE SCHEMA IF NOT EXISTS akasuki;

CREATE TABLE IF NOT EXISTS akasuki.guild_options (
    guild_id                  BIGINT NOT NULL,
    logging_enabled           BOOLEAN NOT NULL DEFAULT false,
    starboard_enabled         BOOLEAN NOT NULL DEFAULT false,
    starboard_channel         BIGINT      NULL DEFAULT NULL,
    logging_channel           BIGINT      NULL DEFAULT NULL,
    PRIMARY KEY(guild_id)
)