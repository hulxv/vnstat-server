use super::traits::*;
use crate::server::api::auth::database::{schema::block_list, Statements};
use anyhow::Result;
use chrono::Local;
use diesel::{insert_into, EqAll, QueryDsl, RunQueryDsl, SqliteConnection};
use regex::Regex;
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

#[derive(PartialEq, Debug)]
pub enum BlockErrorKinds {
    AlreadyBlocked,
    AlreadyUnBlocked,
    InvliadIPv4Pattern,
}

#[derive(Queryable, Insertable, Clone, Debug, PartialEq, Serialize)]
#[table_name = "block_list"]
pub struct BlockList {
    pub id: i32,
    pub ip_addr: String,
    pub blocked_at: String,
}

impl BlockList {
    pub fn new(conn: &SqliteConnection, addr: &str) -> Result<Self, BlockError> {
        let pattern = Regex::new(r"^((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();
        if !pattern.is_match(addr) {
            return Err(BlockError::new(
                BlockErrorKinds::InvliadIPv4Pattern,
                "Invalid IPv4 address pattern",
            ));
        }
        let last_id = match block_list::table.load::<Self>(conn) {
            Err(_) => 0,
            Ok(keys) => match keys.last() {
                Some(key) => key.id,
                None => 0,
            },
        };
        Ok(Self {
            id: last_id + 1,
            ip_addr: addr.to_owned(),
            blocked_at: Local::now().to_rfc2822(),
        })
    }

    pub fn block(conn: &SqliteConnection, addr: &str) -> Result<(), BlockError> {
        if Self::find(conn, |l| l.ip_addr == addr).is_some() {
            return Err(BlockError::new(
                BlockErrorKinds::AlreadyBlocked,
                &format!("IP address already blocked"),
            ));
        }
        Self::new(conn, addr)?.create(conn).unwrap();
        Ok(())
    }
    pub fn unblock(conn: &SqliteConnection, addr: &str) -> Result<(), BlockError> {
        let pattern = Regex::new(r"^((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();
        if !pattern.is_match(addr) {
            return Err(BlockError::new(
                BlockErrorKinds::InvliadIPv4Pattern,
                "Invalid IPv4 address pattern",
            ));
        }

        if Self::find(conn, |l| l.ip_addr == addr).is_none() {
            return Err(BlockError::new(
                BlockErrorKinds::AlreadyUnBlocked,
                &format!("IP address already un-blocked"),
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

mod tests {
    use super::*;
    use crate::api::auth::*;
    #[test]
    async fn block_nvalid_ip_address() {
        let db = InitDatabase::connect().unwrap();
        db.init().unwrap();
        let invalid_addresses = vec![
            "256.1.1.1",
            "2222.1.1.1",
            "1..1.1.1",
            "1g.1.1.1",
            "ff.dd.sds.sfs",
            "hello.world.!.com",
            "1.1.1",
            "127.1.0.?",
            "1.1.1.1.1",
            "1.1.1.1.1",
            "1.1.1.1.",
            ".1.1.1.1",
        ];
        for addr in invalid_addresses {
            assert_eq!(
                BlockErrorKinds::InvliadIPv4Pattern,
                BlockList::block(db.conn(), addr).unwrap_err().kind
            );
        }
    }
    #[test]
    async fn unblock_nvalid_ip_address() {
        let db = InitDatabase::connect().unwrap();
        db.init().unwrap();
        let invalid_addresses = vec![
            "256.1.1.1",
            "2222.1.1.1",
            "1..1.1.1",
            "1g.1.1.1",
            "ff.dd.sds.sfs",
            "hello.world.!.com",
            "1.1.1",
            "127.1.0.?",
            "1.1.1.1.1",
            "1.1.1.1.",
            ".1.1.1.1",
        ];
        for addr in invalid_addresses {
            assert_eq!(
                BlockErrorKinds::InvliadIPv4Pattern,
                BlockList::block(db.conn(), addr).unwrap_err().kind
            );
        }
    }
}
