CREATE TABLE IF NOT EXISTS user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS credential (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    client_id TEXT UNIQUE NOT NULL,
    client_secret TEXT NOT NULL,
    vendor TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS credential_client_id ON credential(client_id);
CREATE TABLE IF NOT EXISTS token (
    token TEXT UNIQUE NOT NULL,
    token_type INTEGER,
    is_active INTEGER,
    expired_at TIMESTAMP,
    user_id INTEGER,
    credential_id INTEGER,
    FOREIGN KEY(user_id) REFERENCES user(id),
    FOREIGN KEY(credential_id) REFERENCES credential(id)
);
CREATE INDEX IF NOT EXISTS idx_token ON token(token);
-- INSERT INTO user (account, password)
-- VALUES (
--     'tester_01',
--     'passwd'
--   );
-- INSERT INTO credential (
--     client_id,
--     client_secret,
--     vendor
--   )
-- VALUES (
--     'tester_01_client_id',
--     'tester_01_client_secret',
--     'tester_01_vendor'
--   );