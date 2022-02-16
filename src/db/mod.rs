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
pub mod models;

const DEFAULT_DATABASE_PATH: &str = "/var/lib/vnstat/vnstat.db";
pub struct Database {
    pub path: String,
    pub conn: Option<SqliteConnection>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self, Error> {
        match Path::new(path).exists() {
            false => Err(Error::new(
                NotFound,
                format!("Database file [{}] is not found", path),
            )),
            true => Ok(Self {
                path: path.to_owned(),
                conn: None,
            }),
        }
    }
    pub fn default() -> Result<Self, Error> {
        if !Path::new(DEFAULT_DATABASE_PATH).exists() {
            return Err(Error::new(NotFound,format!("Default database file ({}) is not found, You should create sqlite db file in default vnStat database path.", DEFAULT_DATABASE_PATH)));
        }
        match Path::new(DEFAULT_DATABASE_PATH).exists() {
            false => Err(Error::new(
                NotFound,
                format!("Database file [{}] is not found", DEFAULT_DATABASE_PATH),
            )),
            true => Ok(Self {
                path: DEFAULT_DATABASE_PATH.to_owned(),
                conn: None,
            }),
        }
    }

    pub fn connect(&mut self) -> Result<&mut Self, Error> {
        match SqliteConnection::establish(self.path.as_str()) {
            Err(err) => Err(Error::new(Interrupted, err)),
            Ok(conn) => {
                self.conn = Some(conn);
                Ok(self)
            }
        }
    }

    pub fn select_table<T>(&mut self, table: String) -> Result<Vec<T>, String>
    where
        T: diesel::deserialize::QueryableByName<diesel::sqlite::Sqlite>,
    {
        match self.conn.is_some() {
            true => match sql_query(format!("SELECT * from {}", table))
                .load(&*self.conn.as_ref().unwrap())
            {
                Err(err) => Err(format!("{}", err)),
                Ok(result) => Ok(result),
            },
            false => Err(format!("[{}] Database wasn't connected", Interrupted)),
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
fn new_database_with_default_path() {
    // println!("{:?}",)
    assert_eq!(
        Database::default().unwrap().path,
        DEFAULT_DATABASE_PATH.to_owned()
    )
}

#[test]
fn new_database_with_exists_path() {
    let path = "./test.db";
    if !Path::new(path).exists() {
        fs::File::create(path).unwrap();
    }
    assert_eq!(Database::new(path).unwrap().path, path.to_owned());
}

#[test]
fn new_database_with_unexists_path() {
    let path = "test.db";
    if Path::new(path).exists() {
        fs::remove_file(path).unwrap();
    }
    assert_eq!(Database::new(path).map_err(|e| e.kind()), Err(NotFound));
}

#[test]
fn database_connection_with_default_path() {
    assert!(Database::default().is_ok())
}

#[test]

fn database_connection_with_exists_path() {
    let path = "test.db";
    if !Path::new(path).exists() {
        fs::File::create(path).unwrap();
    }
    assert!(Database::new(path).is_ok());
}

#[test]
fn select_data_from_table() {
    for (i, value) in Database::default()
        .unwrap()
        .connect()
        .unwrap()
        .select_table::<Interface>("interface".to_owned())
        .unwrap()
        .iter()
        .enumerate()
    {
        println!("{value:?}");
    }
    assert!(true);
}
