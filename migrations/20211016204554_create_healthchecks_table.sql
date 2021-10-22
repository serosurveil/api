-- Add migration script here
CREATE TABLE healthchecks(
   id uuid NOT NULL, PRIMARY KEY (id),
   site TEXT NOT NULL,
   latency FLOAT NOT NULL,
   time timestamptz NOT NULL
);