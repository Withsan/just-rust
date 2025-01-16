drop table if exists user;
CREATE TABLE user (
	id TEXT PRIMARY KEY NOT NULL ,
	name TEXT UNIQUE NOT NULL,
	password TEXT NOT NULL,
	solt BLOB NOT NULL,
	status INT8 NOT NULL,
	certificate BLOB NOT NULL,
	create_by TEXT NOT NULL,
	create_at TEXT NOT NULL
);

