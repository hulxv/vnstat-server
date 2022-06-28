table! {
    use diesel::sql_types::{Date,Text,Integer};
    connections (uuid) {
        uuid -> Text,
        ip_addr -> Text,
        user_agent -> Text,
        connected_at -> Date,
    }
}

table! {
    use diesel::sql_types::{Date,Text,Integer};
    keys (id) {
        id -> Integer,
        value -> Text,
        created_at -> Date,
        expires_at -> Date,
        conn_uuid -> Text,
    }
}
