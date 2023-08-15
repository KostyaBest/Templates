-- Add migration script here
CREATE TABLE IF NOT EXISTS "users"(
	id SERIAL,
	name VARCHAR(50) NOT NULL,
	last_name VARCHAR(50) NOT NULL
);
