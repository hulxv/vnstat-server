mod model;
mod query;
pub mod schema;
mod tests;

pub use model::*;
use query::*;

use anyhow::Result;
use diesel::{
    dsl::sql_query,
    prelude::{Connection, SqliteConnection},
    RunQueryDsl,
};
use dirs::config_dir;
use std::{
    fs::{create_dir_all, File},
    path::Path,
};

// #[derive(Clone)]
pub struct InitDatabase {
    pub conn: SqliteConnection,
}

/// Initialization of the authentication database, i.e. its creation and creation of its tables
impl InitDatabase {
    pub fn connect() -> Result<Self> {
        Ok(Self {
            conn: SqliteConnection::establish(
                &DatabaseFile::new()
                    .unwrap()
                    .create_if_not_exists()
                    .unwrap()
                    .path(),
            )?,
        })
    }

    pub fn init(&self) -> Result<()> {
        sql_query(CREATE_INFO_QUERY).execute(&self.conn)?;

        Info::setup(&self.conn);
        let db_version = Info::find(&self.conn, |i| i.key() == "db_version")
            .unwrap()
            .value();

        if DATABASE_VERSION > db_version.parse()? {
            let tables = vec!["connections", "keys", "info"];

            for t in tables.iter() {
                sql_query(&format!("DROP TABLE IF EXISTS {t}")).execute(&self.conn)?;
            }
            sql_query(CREATE_INFO_QUERY).execute(&self.conn)?;
            Info::setup(&self.conn);
        }

        for q in [
            CREATE_KEYS_QUERY,
            CREATE_CONNECTIONS_QUERY,
            CREATE_BLOCK_LIST_QUERY,
        ]
        .iter()
        {
            sql_query(*q).execute(&self.conn)?;
        }
        Ok(())
    }

    pub fn conn(&self) -> &SqliteConnection {
        &self.conn
    }
}

pub struct DatabaseFile {
    path: String,
}
impl DatabaseFile {
    pub fn new() -> Result<Self> {
        let database_dir = [
            match std::env::var("SUDO_USER") {
                Ok(_) => "/etc".to_owned(),
                Err(_) => config_dir()
                    .unwrap()
                    .into_os_string()
                    .into_string()
                    .unwrap(),
            },
            "/vns/database".to_owned(),
        ]
        .concat();
        if !Path::new(&database_dir).exists() {
            create_dir_all(database_dir.clone()).unwrap();
        }
        Ok(Self {
            path: [database_dir, "/auth.db".to_owned()].concat(),
        })
    }

    pub fn create_if_not_exists(&self) -> Result<&Self> {
        if !Path::new(&self.path).exists() {
            File::create(&self.path)?;
        }
        Ok(self)
    }
    pub fn path(&self) -> String {
        self.path.clone()
    }
}
