use crate::server::api::auth::database::{schema::info, Create, Statements};
use anyhow::Result;

use diesel::{insert_into, Insertable, Queryable, RunQueryDsl, SqliteConnection};
use serde_derive::Serialize;

// Database Info
pub const DATABASE_VERSION: i32 = 1;

#[derive(Queryable, Insertable, Clone, Debug, PartialEq, Serialize)]
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
        insert_into(info::table).values(self).execute(conn)?;
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
