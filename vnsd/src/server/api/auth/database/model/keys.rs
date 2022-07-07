use super::{traits::*, Connections};
use crate::server::api::auth::database::schema::{connections, keys};
use anyhow::Result;
use app::Configs;
use chrono::{DateTime, Duration, FixedOffset, Local};

use diesel::{insert_into, Insertable, Queryable, RunQueryDsl, SqliteConnection};
use rand::{distributions::Alphanumeric, Rng};
use serde_derive::Serialize;

#[derive(Queryable, Insertable, Clone, Debug, PartialEq, Serialize)]
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
                expires_at: match Local::now().checked_add_signed(Duration::days(
                    Configs::init().unwrap().auth().key_expire_duration(),
                )) {
                    Some(dt) => dt,
                    None => Local::now(),
                }
                .to_rfc2822(),
                conn_uuid: conn_uuid.to_owned(),
            };
        }
    }

    pub fn is_valid(conn: &SqliteConnection, key_value: &str) -> bool {
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
        insert_into(keys::table).values(self).execute(conn)?;
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
