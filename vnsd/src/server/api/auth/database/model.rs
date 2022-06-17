use diesel::{Insertable, Queryable};

pub struct Connections {
    pub uuid: u64,
    pub host: String,
    pub key_id: u64,
    pub session_id: u64,
}

pub struct Keys {
    pub id: u64,
    pub value: String,
    pub expire_time: u64,
}

pub struct Sessions {
    pub id: u64,
    pub last_login: String,
}
