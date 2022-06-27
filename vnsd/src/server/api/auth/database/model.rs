use super::schema::{connections, keys};
use anyhow::{anyhow, Result};
use diesel::{insert_into, Insertable, Queryable, RunQueryDsl, SqliteConnection};
use uuid::Uuid;

use chrono::{
    prelude::{DateTime, Local, TimeZone},
    Duration, FixedOffset,
};
use rand::{distributions::Alphanumeric, Rng};

const EXPIRE_KEY_DURATION_DAYS: i64 = 2;

/// Implement create, read, update and delete for table.
pub trait CRUD {
    /// Create or insert new values.
    fn create(&self, conn: &SqliteConnection) -> Result<()>;
    /// Read previous values.
    fn read(&self, conn: &SqliteConnection) -> Result<()>;
    /// Update previous values.
    fn update<T>(&self, conn: &SqliteConnection, id: T) -> Result<()>;
    /// Delete previous values.
    fn delete<T>(&self, conn: &SqliteConnection, id: T) -> Result<()>;
}

#[derive(Queryable, Insertable, Clone, Debug, PartialEq)]
#[table_name = "connections"]
pub struct Connections {
    pub uuid: String,
    pub ip_addr: String,
    pub user_agent: String,
    pub connected_at: String,
    pub key_id: i32,
}

impl Connections {
    pub fn new(conn: &SqliteConnection, ip_addr: &str, user_agent: &str) -> Self {
        let key = Keys::generate(conn);
        key.create(conn).unwrap();
        Self {
            uuid: Uuid::new_v4().to_string(),
            ip_addr: ip_addr.to_owned(),
            user_agent: user_agent.to_owned(),
            key_id: key.id(),
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

impl CRUD for Connections {
    fn create(&self, conn: &SqliteConnection) -> Result<()> {
        if let Err(e) = insert_into(connections::table).values(self).execute(conn) {
            return Err(anyhow!(e));
        }
        Ok(())
    }
    fn read(&self, conn: &SqliteConnection) -> Result<()> {
        todo!()
    }
    fn update<T>(&self, conn: &SqliteConnection, id: T) -> Result<()> {
        todo!()
    }
    fn delete<T>(&self, conn: &SqliteConnection, id: T) -> Result<()> {
        todo!()
    }
}

#[derive(Queryable, Insertable, Clone, Debug, PartialEq)]
#[table_name = "keys"]
pub struct Keys {
    pub id: i32,
    pub value: String,
    pub created_at: String,
    pub expires_at: String,
}

impl Keys {
    pub fn generate(conn: &SqliteConnection) -> Self {
        let last_id = match keys::table.load::<Self>(conn) {
            Err(_) => 0,
            Ok(keys) => match keys.last() {
                Some(key) => key.id(),
                None => 0,
            },
        };
        let mut value = String::with_capacity(32);
        'outer: loop {
            value = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .map(char::from)
                .collect();
            for key in keys::table.load::<Self>(conn).unwrap().iter() {
                if key.value.eq(&value) {
                    continue 'outer;
                }
            }

            break;
        }

        Self {
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
        }
    }

    pub fn id(&self) -> i32 {
        self.id
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
}

impl std::default::Default for Keys {
    fn default() -> Self {
        Self {
            id: 1,
            value: "".to_owned(),
            created_at: Local::now().to_string(),
            expires_at: "".to_owned(),
        }
    }
}

impl CRUD for Keys {
    fn create(&self, conn: &SqliteConnection) -> Result<()> {
        if let Err(e) = insert_into(keys::table).values(self).execute(conn) {
            return Err(anyhow!(e));
        }
        Ok(())
    }
    fn read(&self, conn: &SqliteConnection) -> Result<()> {
        todo!()
    }
    fn update<T>(&self, conn: &SqliteConnection, id: T) -> Result<()> {
        todo!()
    }
    fn delete<T>(&self, conn: &SqliteConnection, id: T) -> Result<()> {
        todo!()
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
            Keys::generate(db.conn().clone()).expires_at().to_rfc2822()
        )
    }

    #[test]
    async fn when_key_was_created() {
        let db = InitDatabase::connect().unwrap();
        let key = Keys::generate(db.conn());
        println!("{:#?}", key);

        db.init().unwrap();
        assert_eq!(Local::now().to_rfc2822(), key.created_at().to_rfc2822())
    }
    #[test]
    async fn create_new_key() {
        let db = InitDatabase::connect().unwrap();
        db.init().unwrap();

        let key = Keys::generate(db.conn());
        key.create(db.conn()).unwrap();

        assert!(keys::table.load::<Keys>(db.conn()).unwrap().contains(&key))
    }

    #[test]
    async fn create_new_connection() {
        let (ip_addr, user_agent) = ("0.0.0.0", "USER-AGENT");
        let db = InitDatabase::connect().unwrap();
        db.init().unwrap();
        let connection = Connections::new(db.conn(), ip_addr, user_agent);
        connection.create(db.conn()).unwrap();

        assert!(connections::table
            .load::<Connections>(db.conn())
            .unwrap()
            .contains(&connection))
    }
}
