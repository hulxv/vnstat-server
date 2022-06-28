// ! Changeable

pub const CREATE_KEYS_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS keys (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        value TEXT,
        created_at DATE,
        expires_at DATE,
        conn_uuid String NOT NULL,
        CONSTRAINT fk_conn
            FOREIGN KEY (conn_uuid)
            REFERENCES connections(uuid)
    );
"#;

pub const CREATE_CONNECTIONS_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS connections (
        uuid TEXT PRIMARY KEY,
        ip_addr TEXT,
        user_agent TEXT,
        connected_at DATE
    );
"#;
pub const CREATE_INFO_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS info (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        key TEXT,
        value TEXT
    );
"#;
