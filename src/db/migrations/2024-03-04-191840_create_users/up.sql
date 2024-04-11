-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "users"(
	"id" UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
	"login" TEXT NOT NULL,
	"hashed_password" TEXT NOT NULL,
	"name" TEXT NOT NULL,
	"email" TEXT NOT NULL,
	"is_admin" BOOL NOT NULL,
    "avatar" TEXT NOT NULL
);
