// ! Changeable

pub const CREATE_KEYS_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS keys (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        value TEXT,
        expire_time DATE
    );
"#;

pub const CREATE_SESSIONS_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS sessions (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        last_login TEXT
    );
"#;

pub const CREATE_CONNECTIONS_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS connections (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        host TEXT,
        key_id INTEGER,
        session_id INTEGER,
        CONSTRAINT fk_keys
            FOREIGN KEY (key_id)
            REFERENCES keys(id),
        CONSTRAINT fk_sessions 
            FOREIGN KEY (session_id)
            REFERENCES sessions(id)
    );
"#;
