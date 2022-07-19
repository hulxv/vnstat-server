use app::MainDirectory;

mod model;
mod query;
pub mod schema;
mod tests;

pub use model::*;
use query::*;

use anyhow::{anyhow, Result};
use diesel::{
    dsl::sql_query,
    prelude::{Connection, SqliteConnection},
    RunQueryDsl,
};
use std::{
    fs::{create_dir_all, File},
    path::Path,
};

pub struct InitDatabase {
    pub conn: SqliteConnection,
}

/// Initialization of the authentication database, i.e. its creation and creation of its tables
impl InitDatabase {
    pub fn connect() -> Result<Self> {
        let path = match DatabaseFile::new()?.create_if_not_exists() {
            Err(e) => {
                return Err(anyhow!(std::io::Error::new(
                    e.downcast_ref::<std::io::Error>().unwrap().kind(),
                    format!("Cannot create database file: {e}")
                )))
            }
            Ok(f) => f.path(),
        };
        Ok(Self {
            conn: SqliteConnection::establish(&path)?,
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
    file_name: String,
    dir: String,
}
impl DatabaseFile {
    pub fn new() -> Result<Self> {
        Ok(Self {
            file_name: "auth.db".to_owned(),
            dir: [MainDirectory::get()?, "/database".to_owned()].concat(),
        })
    }

    pub fn create_if_not_exists(&self) -> Result<&Self> {
        if !Path::new(&self.dir).exists() {
            create_dir_all(&self.dir)?;
        }
        if !Path::new(&self.path()).exists() {
            File::create(&self.path())?;
        }
        Ok(self)
    }
    pub fn path(&self) -> String {
        [self.dir.clone(), "/".to_owned(), self.file_name.clone()].concat()
    }
}
