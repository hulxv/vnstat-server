// TODO: refactoring this file

use super::*;
use anyhow::{anyhow, Result};
use diesel::{
    insert_into, prelude::*, Insertable, QueryDsl, Queryable, RunQueryDsl, SqliteConnection,
};
use uuid::Uuid;

use app::Configs;
use chrono::{
    prelude::{DateTime, Local},
    Duration, FixedOffset,
};
use rand::{distributions::Alphanumeric, Rng};

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

pub struct BlockError {
    pub kind: BlockErrorKinds,
    pub details: String,
}

impl BlockError {
    pub fn new(kind: BlockErrorKinds, details: &str) -> Self {
        Self {
            kind,
            details: details.to_owned(),
        }
    }
}

impl std::fmt::Display for BlockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

// TODO: add error for invalid ip addresss pattern
pub enum BlockErrorKinds {
    AlreadyBlocked,
    AlreadyUnBlocked,
}

#[derive(Queryable, Insertable, Clone, Debug, PartialEq)]
#[table_name = "block_list"]
pub struct BlockList {
    id: i32,
    ip_addr: String,
    blocked_at: String,
}

impl BlockList {
    pub fn new(conn: &SqliteConnection, ip_addr: &str) -> Self {
        // TODO: check from validate ip address pattern
        let last_id = match block_list::table.load::<Self>(conn) {
            Err(_) => 0,
            Ok(keys) => match keys.last() {
                Some(key) => key.id,
                None => 0,
            },
        };
        Self {
            id: last_id + 1,
            ip_addr: ip_addr.to_owned(),
            blocked_at: Local::now().to_rfc2822(),
        }
    }

    pub fn block(conn: &SqliteConnection, ip_addr: &str) -> Result<(), BlockError> {
        if Self::find(conn, |l| l.ip_addr == ip_addr).is_some() {
            return Err(BlockError::new(
                BlockErrorKinds::AlreadyBlocked,
                &format!("{ip_addr} already blocked"),
            ));
        }
        Self::new(conn, ip_addr).create(conn).unwrap();
        Ok(())
    }
    pub fn unblock(conn: &SqliteConnection, addr: &str) -> Result<(), BlockError> {
        use super::schema::block_list::dsl::*;
        if Self::find(conn, |l| l.ip_addr == addr).is_none() {
            return Err(BlockError::new(
                BlockErrorKinds::AlreadyUnBlocked,
                &format!("{addr} already un-blocked"),
            ));
        }
        diesel::delete(block_list.filter(ip_addr.eq_all(addr)))
            .execute(conn)
            .unwrap();
        Ok(())
    }
    pub fn is_blocked(conn: &SqliteConnection, ip_addr: &str) -> bool {
        Self::find(conn, |item| item.ip_addr == ip_addr).is_some()
    }
}

impl Create for BlockList {
    type Output = Self;

    fn create(&self, conn: &SqliteConnection) -> Result<Self::Output> {
        if let Err(e) = insert_into(block_list::table).values(self).execute(conn) {
            return Err(anyhow!(e));
        }
        Ok(self.clone())
    }
}

impl Statements for BlockList {
    type Args = Self;
    type SelectOutput = Vec<Self>;
    type FindOutput = Option<Self>;
    fn select<F>(conn: &SqliteConnection, f: F) -> Self::SelectOutput
    where
        F: Fn(Self::Args) -> bool,
    {
        block_list::table
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
        block_list::table
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
