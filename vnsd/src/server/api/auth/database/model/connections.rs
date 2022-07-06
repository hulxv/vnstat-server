use super::traits::*;
use crate::server::api::auth::database::{schema::connections, Statements};
use anyhow::Result;
use chrono::{DateTime, FixedOffset, Local};

use ::uuid::Uuid;
use diesel::{
    insert_into, prelude::*, Insertable, QueryDsl, Queryable, RunQueryDsl, SqliteConnection,
};
use serde_derive::Serialize;

#[derive(Queryable, Insertable, Clone, Debug, PartialEq, Serialize)]
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
        insert_into(connections::table).values(self).execute(conn)?;
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
