use super::schema::*;
use anyhow::{anyhow, Result};
use diesel::{insert_into, Insertable, Queryable, RunQueryDsl, SqliteConnection};
use uuid::Uuid;

use chrono::{
    prelude::{DateTime, Local},
    Duration, FixedOffset,
};
use rand::{distributions::Alphanumeric, Rng};

// TODO: read from configuration file
const EXPIRE_KEY_DURATION_DAYS: i64 = 2;

// Database Info
pub const DATABASE_VERSION: i32 = 1;

/// Create or insert new values.
pub trait Create {
    type Output;
    fn create(&self, conn: &SqliteConnection) -> Result<Self::Output>;
}

pub trait Statements {
    type Args;
    type SelectOutput;
    type FindOutput;
    fn select<F>(conn: &SqliteConnection, f: F) -> Self::SelectOutput
    where
        F: Fn(Self::Args) -> bool;
    fn find<F>(conn: &SqliteConnection, f: F) -> Self::FindOutput
    where
        F: Fn(Self::Args) -> bool;
}

#[derive(Queryable, Insertable, Clone, Debug, PartialEq)]
#[table_name = "connections"]
pub struct Connections {
    pub uuid: String,
    pub ip_addr: String,
    pub user_agent: String,
    pub connected_at: String,
}

impl Connections {
    pub fn new(ip_addr: &str, user_agent: &str) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            ip_addr: ip_addr.to_owned(),
            user_agent: user_agent.to_owned(),
            connected_at: Local::now().to_rfc2822(),
        }
    }

    pub fn uuid(&self) -> String {
        self.uuid.clone()
    }
    pub fn ip_addr(&self) -> String {
        self.ip_addr.clone()
    }
    pub fn user_agent(&self) -> String {
        self.user_agent.clone()
    }
    pub fn connected_at(&self) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc2822(&self.connected_at.clone()).unwrap()
    }
}

impl Create for Connections {
    type Output = Self;
    fn create(&self, conn: &SqliteConnection) -> Result<Self::Output> {
        if let Err(e) = insert_into(connections::table).values(self).execute(conn) {
            return Err(anyhow!(e));
        }
        Ok(self.clone())
    }
}

impl Statements for Connections {
    type Args = Self;
    type SelectOutput = Vec<Self>;
    type FindOutput = Option<Self>;
    fn select<F>(conn: &SqliteConnection, f: F) -> Self::SelectOutput
    where
        F: Fn(Self::Args) -> bool,
    {
        connections::table
            .load::<Self>(conn)
            .unwrap()
            .into_iter()
            .filter(|e| f(e.clone().into()))
            .collect::<Self::SelectOutput>()
    }

    fn find<F>(conn: &SqliteConnection, f: F) -> Self::FindOutput
    where
        F: Fn(Self::Args) -> bool,
    {
        connections::table
            .load::<Self>(conn)
            .unwrap()
            .into_iter()
            .find(|e| f(e.clone().into()))
    }
}

#[derive(Queryable, Insertable, Clone, Debug, PartialEq)]
#[table_name = "keys"]
pub struct Keys {
    pub id: i32,
    pub value: String,
    pub created_at: String,
    pub expires_at: String,
    pub conn_uuid: String,
}

impl Keys {
    pub fn generate_new_key(conn: &SqliteConnection, conn_uuid: &str) -> Self {
        let last_id = match keys::table.load::<Self>(conn) {
            Err(_) => 0,
            Ok(keys) => match keys.last() {
                Some(key) => key.id,
                None => 0,
            },
        };
        'outer: loop {
            let value = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .map(char::from)
                .collect();
            for key in keys::table.load::<Self>(conn).unwrap().iter() {
                if key.value.eq(&value) {
                    continue 'outer;
                }
            }

            return Self {
                id: last_id + 1,
                value,
                created_at: Local::now().to_rfc2822(),
                expires_at: match Local::now()
                    .checked_add_signed(Duration::days(EXPIRE_KEY_DURATION_DAYS))
                {
                    Some(dt) => dt,
                    None => Local::now(),
                }
                .to_rfc2822(),
                conn_uuid: conn_uuid.to_owned(),
            };
        }
    }

    pub fn valid(conn: &SqliteConnection, key_value: &str) -> bool {
        let keys = keys::table.load::<Self>(conn).unwrap();
        for key in keys {
            if key.value().eq(&key_value)
                && key.conn(conn).is_some()
                && Local::now() < key.expires_at()
            {
                return true;
            }
        }
        false
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
    pub fn expires_at(&self) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc2822(&self.expires_at.clone()).unwrap()
    }
    pub fn created_at(&self) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc2822(&self.created_at.clone()).unwrap()
    }

    pub fn conn(&self, conn: &SqliteConnection) -> Option<Connections> {
        for connection in connections::table.load::<Connections>(conn).unwrap() {
            if connection.uuid().eq(&self.conn_uuid) {
                return Some(connection);
            }
        }
        None
    }
}

impl std::default::Default for Keys {
    fn default() -> Self {
        Self {
            id: 1,
            value: "".to_owned(),
            created_at: Local::now().to_rfc2822(),
            expires_at: Local::now().to_rfc2822(),
            conn_uuid: "UNKNOWN".to_owned(),
        }
    }
}

impl Create for Keys {
    type Output = Self;
    fn create(&self, conn: &SqliteConnection) -> Result<Self::Output> {
        if let Err(e) = insert_into(keys::table).values(self).execute(conn) {
            return Err(anyhow!(e));
        }
        Ok(self.clone())
    }
}

impl Statements for Keys {
    type Args = Self;
    type SelectOutput = Vec<Self>;
    type FindOutput = Option<Self>;
    fn select<F>(conn: &SqliteConnection, f: F) -> Self::SelectOutput
    where
        F: Fn(Self::Args) -> bool,
    {
        keys::table
            .load::<Self>(conn)
            .unwrap()
            .into_iter()
            .filter(|e| f(e.clone().into()))
            .collect::<Self::SelectOutput>()
    }

    fn find<F>(conn: &SqliteConnection, f: F) -> Self::FindOutput
    where
        F: Fn(Self::Args) -> bool,
    {
        keys::table
            .load::<Self>(conn)
            .unwrap()
            .into_iter()
            .find(|e| f(e.clone().into()))
    }
}

#[derive(Queryable, Insertable, Clone, Debug, PartialEq)]
#[table_name = "info"]
pub struct Info {
    id: i32,
    key: String,
    value: String,
}

impl Info {
    pub fn new(k: &str, v: &str, conn: &SqliteConnection) -> Self {
        let last_id = match info::table.load::<Self>(conn) {
            Err(_) => 0,
            Ok(i) => match i.last() {
                Some(ii) => ii.id,
                None => 0,
            },
        };
        Self {
            id: last_id + 1,
            key: k.to_owned(),
            value: v.to_owned(),
        }
    }
    pub fn key(&self) -> String {
        self.key.clone()
    }
    pub fn value(&self) -> String {
        self.value.clone()
    }
    pub fn setup(conn: &SqliteConnection) {
        let expected_info = [("db_version", DATABASE_VERSION.to_string())];

        let exist_info = info::table.load::<Info>(conn).unwrap();
        for i in expected_info.iter() {
            if exist_info.iter().find(|e| e.key().eq(i.0)).is_none() {
                Info::new(i.0, &i.1, conn).create(conn).unwrap();
            }
        }
    }
}

impl Create for Info {
    type Output = Self;
    fn create(&self, conn: &SqliteConnection) -> Result<Self::Output> {
        if let Err(e) = insert_into(info::table).values(self).execute(conn) {
            return Err(anyhow!(e));
        }
        Ok(self.clone())
    }
}

impl Statements for Info {
    type Args = Self;
    type SelectOutput = Vec<Self>;
    type FindOutput = Option<Self>;
    fn select<F>(conn: &SqliteConnection, f: F) -> Self::SelectOutput
    where
        F: Fn(Self::Args) -> bool,
    {
        info::table
            .load::<Self>(conn)
            .unwrap()
            .into_iter()
            .filter(|e| f(e.clone().into()))
            .collect::<Self::SelectOutput>()
    }

    fn find<F>(conn: &SqliteConnection, f: F) -> Self::FindOutput
    where
        F: Fn(Self::Args) -> bool,
    {
        info::table
            .load::<Self>(conn)
            .unwrap()
            .into_iter()
            .find(|e| f(e.clone().into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::auth::database::InitDatabase;
    // use ::core::prelude::v1::test;
    use chrono::prelude::Local;

    #[test]
    async fn when_key_will_expires() {
        let db = InitDatabase::connect().unwrap();

        db.init().unwrap();
        assert_eq!(
            Local::now()
                .checked_add_signed(Duration::days(EXPIRE_KEY_DURATION_DAYS))
                .unwrap()
                .to_rfc2822(),
            Keys::generate_new_key(db.conn().clone(), "")
                .expires_at()
                .to_rfc2822()
        )
    }

    #[test]
    async fn when_key_was_created() {
        let db = InitDatabase::connect().unwrap();
        let key = Keys::generate_new_key(db.conn(), "");
        println!("{:#?}", key);

        db.init().unwrap();
        assert_eq!(Local::now().to_rfc2822(), key.created_at().to_rfc2822())
    }
    #[test]
    async fn create_new_key() {
        let db = InitDatabase::connect().unwrap();
        db.init().unwrap();

        // let (ip_addr, user_agent) = ("0.0.0.0", "USER-AGENT");

        // let connection = Connections::new(db.conn(), ip_addr, user_agent);
        // key.create(db.conn()).unwrap();

        let key = Keys::generate_new_key(db.conn(), "");
        key.create(db.conn()).unwrap();

        assert!(keys::table.load::<Keys>(db.conn()).unwrap().contains(&key))
    }

    #[test]
    async fn create_new_connection() {
        let (ip_addr, user_agent) = ("0.0.0.0", "USER-AGENT");
        let db = InitDatabase::connect().unwrap();
        db.init().unwrap();
        let connection = Connections::new(ip_addr, user_agent);
        connection.create(db.conn()).unwrap();

        assert!(connections::table
            .load::<Connections>(db.conn())
            .unwrap()
            .contains(&connection))
    }
    #[test]
    async fn create_new_connection_with_its_key() {
        let db = InitDatabase::connect().unwrap();
        db.init().unwrap();

        let (ip_addr, user_agent) = ("0.0.0.0", "USER-AGENT");

        let connection = Connections::new(ip_addr, user_agent);
        connection.create(db.conn()).unwrap();

        let key = Keys::generate_new_key(db.conn(), &connection.uuid());
        key.create(db.conn()).unwrap();

        assert!(keys::table.load::<Keys>(db.conn()).unwrap().contains(&key));
        assert!(connections::table
            .load::<Connections>(db.conn())
            .unwrap()
            .contains(&connection));

        for key in keys::table.load::<Keys>(db.conn()).unwrap() {
            if key.conn(db.conn()).is_some() {
                return;
            }
        }
        assert!(false)
    }

    #[test]
    async fn validate_key() {
        let db = InitDatabase::connect().unwrap();
        db.init().unwrap();

        let connection = Connections::new("0.0.0.0", "USER-AGENT");
        connection.create(db.conn()).unwrap();

        let valid_key = Keys::generate_new_key(db.conn(), &connection.uuid());
        valid_key.create(db.conn()).unwrap();

        let invalid_keys = vec![
            // A key doesn't exist
            Keys::default(),
            // An exists key but connection uuid doesn't exist
            Keys::generate_new_key(db.conn(), "UNKNOWN")
                .create(db.conn())
                .unwrap(),
            // An exists key and connection uuid is exists but the key is expires
            Keys {
                id: match keys::table.load::<Keys>(db.conn()) {
                    Err(_) => 0,
                    Ok(keys) => match keys.last() {
                        Some(key) => key.id,
                        None => 0,
                    },
                } + 1,
                value: "".to_owned(),
                expires_at: Local::now().to_rfc2822(),
                created_at: Local::now().to_rfc2822(),
                conn_uuid: connection.uuid(),
            }
            .create(db.conn())
            .unwrap(),
        ];

        assert_eq!(Keys::valid(db.conn(), &valid_key.value()), true);

        for k in invalid_keys.iter() {
            assert_eq!(Keys::valid(db.conn(), &k.value()), false);
        }
    }

    #[test]
    async fn select_columns_from_connections_table() {
        let db = InitDatabase::connect().unwrap();
        db.init().unwrap();

        let connections = vec![
            Connections::new("0.0.0.0", "user_agent")
                .create(db.conn())
                .unwrap(),
            Connections::new("0.0.0.1", "user_agent")
                .create(db.conn())
                .unwrap(),
            Connections::new("0.0.0.1", "user_agent")
                .create(db.conn())
                .unwrap(),
            Connections::new("0.0.0.0", "user_agent")
                .create(db.conn())
                .unwrap(),
        ];

        let expected = vec![
            connections.first().unwrap().uuid(),
            connections.last().unwrap().uuid(),
        ];

        for uuid in expected {
            assert!(Connections::select(db.conn(), |c| c.ip_addr() == "0.0.0.0")
                .iter()
                .map(|c| c.uuid())
                .collect::<Vec<String>>()
                .contains(&uuid));
        }
    }

    #[test]
    async fn select_columns_from_keys_table() {
        let db = InitDatabase::connect().unwrap();
        db.init().unwrap();
        let _1st_conn_uuid = Connections::new("0.0.0.0", "user_agent")
            .create(db.conn())
            .unwrap()
            .uuid();
        let _2nd_conn_uuid = Connections::new("0.0.0.0", "user_agent")
            .create(db.conn())
            .unwrap()
            .uuid();
        let keys = vec![
            Keys::generate_new_key(db.conn(), &_1st_conn_uuid)
                .create(db.conn())
                .unwrap()
                .value(),
            Keys::generate_new_key(db.conn(), &_2nd_conn_uuid)
                .create(db.conn())
                .unwrap()
                .value(),
            Keys::generate_new_key(db.conn(), &_1st_conn_uuid)
                .create(db.conn())
                .unwrap()
                .value(),
            Keys::generate_new_key(db.conn(), &_2nd_conn_uuid)
                .create(db.conn())
                .unwrap()
                .value(),
        ];

        let _1st_expected = vec![keys[0].clone(), keys[2].clone()];
        println!("1st: {_1st_expected:#?}");
        for key_value in Keys::select(db.conn(), |k| {
            if let Some(conn) = k.conn(db.conn()) {
                return conn.uuid() == _1st_conn_uuid;
            }
            false
        })
        .iter()
        .map(|k| k.value())
        {
            println!("{key_value}");
            assert!(_1st_expected.contains(&key_value));
        }

        let _2nd_expected = vec![keys[1].clone(), keys[3].clone()];

        println!("2nd: {_2nd_expected:#?}");
        for key_value in Keys::select(db.conn(), |k| {
            if let Some(conn) = k.conn(db.conn()) {
                return conn.uuid() == _2nd_conn_uuid;
            }
            false
        })
        .iter()
        .map(|k| k.value())
        {
            println!("{key_value}");
            assert!(_2nd_expected.contains(&key_value));
        }
    }

    #[test]
    async fn select_columns_from_info_table() {
        let db = InitDatabase::connect().unwrap();
        db.init().unwrap();
        let expected = vec![format!("{}", DATABASE_VERSION)];
        for key_value in Info::select(db.conn(), |i| i.key() == "db_version")
            .iter()
            .map(|i| i.value())
        {
            assert!(expected.contains(&key_value));
        }
    }
    #[test]
    async fn find_column_in_connections_table() {
        let db = InitDatabase::connect().unwrap();
        db.init().unwrap();
        let connections = vec![
            Connections::new("0.0.0.0", "user_agent")
                .create(db.conn())
                .unwrap()
                .uuid(),
            Connections::new("1.1.1.1", "user_agent")
                .create(db.conn())
                .unwrap()
                .uuid(),
            Connections::new("0.0.0.1", "user_agent")
                .create(db.conn())
                .unwrap()
                .uuid(),
            Connections::new("0.0.0.0", "user_agent")
                .create(db.conn())
                .unwrap()
                .uuid(),
        ];

        let expected = connections[1].clone();

        assert_eq!(
            expected,
            Connections::find(db.conn(), |c| c.ip_addr() == "1.1.1.1"
                && c.uuid() == expected)
            .unwrap()
            .uuid()
        );
    }
}
