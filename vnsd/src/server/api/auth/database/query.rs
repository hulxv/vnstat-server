// ! Changeable

pub const CREATE_KEYS_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS keys (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        value TEXT,
        created_at DATE,
        expires_at DATE
    );
"#;

pub const CREATE_CONNECTIONS_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS connections (
        uuid TEXT PRIMARY KEY,
        ip_addr TEXT,
        user_agent TEXT,
        connected_at DATE,
        key_id INTEGER NOT NULL,
        CONSTRAINT fk_keys
            FOREIGN KEY (key_id)
            REFERENCES keys(id)
    );
"#;
