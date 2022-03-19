pub mod models;

use anyhow::{anyhow, Result};
use core::fmt;
use std::{
    fmt::Debug,
    io::{
        Error,
        ErrorKind::{Interrupted, InvalidData, NotFound},
    },
};
use std::{fs, io, path::Path};

use diesel::{
    dsl::sql_query,
    prelude::{Connection, SqliteConnection},
    query_builder::SqlQuery,
    RunQueryDsl,
};
use models::{info::Info, interface::Interface, traffic::Traffic};
const DEFAULT_DATABASE_PATH: &str = "/var/lib/vnstat/vnstat.db";
pub struct Database {
    pub path: String,
    pub conn: Option<SqliteConnection>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        match Path::new(path).exists() {
            false => Err(anyhow!(Error::new(
                NotFound,
                format!("Database file [{}] is not found", path),
            ))),

            true => Ok(Self {
                path: path.to_owned(),
                conn: None,
            }),
        }
    }
    pub fn default() -> Result<Self> {
        if !Path::new(DEFAULT_DATABASE_PATH).exists() {
            return Err(anyhow!(Error::new(NotFound,format!("Default database file ({}) is not found, You should create sqlite db file in default vnStat database path.", DEFAULT_DATABASE_PATH))));
        }
        match Path::new(DEFAULT_DATABASE_PATH).exists() {
            false => Err(anyhow!(Error::new(
                NotFound,
                format!("Database file [{}] is not found", DEFAULT_DATABASE_PATH),
            ))),
            true => Ok(Self {
                path: DEFAULT_DATABASE_PATH.to_owned(),
                conn: None,
            }),
        }
    }

    pub fn connect(&mut self) -> Result<&mut Self> {
        match SqliteConnection::establish(self.path.as_str()) {
            Err(err) => Err(anyhow!(err)),
            Ok(conn) => {
                self.conn = Some(conn);
                Ok(self)
            }
        }
    }

    pub fn select_table<T>(&mut self, table: String) -> Result<Vec<T>>
    where
        T: diesel::deserialize::QueryableByName<diesel::sqlite::Sqlite>,
    {
        match self.conn.is_some() {
            true => match sql_query(format!("SELECT * from {}", table))
                .load(&*self.conn.as_ref().unwrap())
            {
                Err(err) => Err(anyhow!(err)),
                Ok(result) => Ok(result),
            },
            false => Err(anyhow!(Error::new(
                Interrupted,
                "Database wasn't connected",
            ))),
        }
    }
}

impl fmt::Debug for Database {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path.to_string())
    }
}

impl PartialEq for Database {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

#[cfg(test)]
#[test]
fn new_database_with_default_path() -> Result<()> {
    assert_eq!(Database::default()?.path, DEFAULT_DATABASE_PATH.to_owned());
    Ok(())
}

#[test]
fn new_database_with_exists_path() -> Result<()> {
    let path = "./test.db";
    if !Path::new(path).exists() {
        fs::File::create(path)?;
    }
    assert_eq!(Database::new(path)?.path, path.to_owned());
    Ok(())
}

#[test]
fn new_database_with_unexists_path() -> Result<()> {
    let path = "test.db";
    if Path::new(path).exists() {
        fs::remove_file(path).unwrap();
    }
    assert!(Database::new(path).is_err());
    Ok(())
}

#[test]
fn database_connection_with_default_path() {
    assert!(Database::default().is_ok())
}

#[test]

fn database_connection_with_exists_path() -> Result<()> {
    let path = "test.db";
    if !Path::new(path).exists() {
        fs::File::create(path)?;
    }
    assert!(Database::new(path).is_ok());
    Ok(())
}

#[test]
fn select_data_from_table() -> Result<()> {
    for (i, value) in Database::default()?
        .connect()?
        .select_table::<Interface>("interface".to_owned())?
        .iter()
        .enumerate()
    {
        println!("{value:?}");
    }
    assert!(true);
    Ok(())
}
