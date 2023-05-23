CREATE TABLE "clipboard_entries" (
	"id"	INTEGER UNIQUE,
	"clip_text"	TEXT,
	"clip_bin"  BLOB,
	PRIMARY KEY("id" AUTOINCREMENT)
);