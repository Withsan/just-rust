drop table if exists user;
CREATE TABLE user (
	name TEXT PRIMARY KEY NOT NULL,
	password TEXT NOT NULL,
	solt BLOB NOT NULL,
	certificate BLOB NOT NULL,
	status INT8 NOT NULL,
	create_by TEXT NOT NULL,
	create_at timestamp NOT NULL
);

