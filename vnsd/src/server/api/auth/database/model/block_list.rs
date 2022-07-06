use super::traits::*;
use crate::server::api::auth::database::{schema::block_list, Statements};
use anyhow::Result;
use chrono::Local;
use diesel::{insert_into, EqAll, QueryDsl, RunQueryDsl, SqliteConnection};
use serde_derive::Serialize;

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

#[derive(Queryable, Insertable, Clone, Debug, PartialEq, Serialize)]
#[table_name = "block_list"]
pub struct BlockList {
    pub id: i32,
    pub ip_addr: String,
    pub blocked_at: String,
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
                &format!("already blocked"),
            ));
        }
        Self::new(conn, ip_addr).create(conn).unwrap();
        Ok(())
    }
    pub fn unblock(conn: &SqliteConnection, addr: &str) -> Result<(), BlockError> {
        if Self::find(conn, |l| l.ip_addr == addr).is_none() {
            return Err(BlockError::new(
                BlockErrorKinds::AlreadyUnBlocked,
                &format!("already un-blocked"),
            ));
        }

        use crate::server::api::auth::database::schema::block_list::dsl::*;
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
        use crate::server::api::auth::database::schema::block_list::dsl::*;
        insert_into(block_list).values(self).execute(conn)?;

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
