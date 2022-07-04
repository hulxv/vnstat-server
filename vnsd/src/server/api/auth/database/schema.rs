table! {
    use diesel::sql_types::{Date,  Text};
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
table! {
    use diesel::sql_types::{Text,Integer};
    info (id) {
        id -> Integer,
        key -> Text,
        value -> Text,
    }
}
table! {
    use diesel::sql_types::{Text,Date,Integer};
    block_list {
        id -> Integer,
        ip_addr -> Text,
        blocked_at -> Date,
    }
}
