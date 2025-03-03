-- Add migration script here
DROP TABLE IF EXISTS user;
CREATE TABLE user (
	name TEXT PRIMARY KEY NOT NULL,
	password TEXT NOT NULL,
	solt BLOB NOT NULL,
	certificate BLOB NOT NULL,
	status INT8 NOT NULL,
	create_by TEXT NOT NULL,
	create_at timestamp NOT NULL
);
insert into user values('wyl','1',1,1,1,'init','20250121012509');

DROP TABLE IF EXISTS role;
CREATE TABLE role  (
    id INTEGER PRIMARY KEY  NOT NULL,
    name TEXT  NOT NULL 
);

DROP TABLE IF EXISTS user_role;
CREATE TABLE user_role  (
    user_name TEXT PRIMARY KEY NOT NULL,
    role_id INTEGER  NOT NULL,
    UNIQUE(user_name,role_id)
)


 
