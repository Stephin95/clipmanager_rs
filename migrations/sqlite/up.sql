CREATE TABLE "clipboard_entries" (
	"id"	INTEGER UNIQUE,
	"clip_text"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);