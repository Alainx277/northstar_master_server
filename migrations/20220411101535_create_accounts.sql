CREATE TABLE accounts (
    id INTEGER PRIMARY KEY NOT NULL,
    username TEXT,
    token BLOB,
    token_created DATETIME,
    current_server BLOB,
    last_auth_ip TEXT,
    persistent_data BLOB
);
