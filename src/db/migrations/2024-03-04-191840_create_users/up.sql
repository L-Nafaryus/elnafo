-- Your SQL goes here
CREATE TABLE "users"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"login" TEXT NOT NULL,
	"hashed_password" TEXT NOT NULL,
	"name" TEXT NOT NULL,
	"email" TEXT NOT NULL,
	"is_admin" BOOL NOT NULL
);
