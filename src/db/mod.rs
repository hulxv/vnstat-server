use std::io::{
    Error,
    ErrorKind::{Interrupted, NotFound},
};
use std::{fs, io, path::Path};

use diesel::{prelude::Connection, sqlite::SqliteConnection};

pub mod models;
pub mod schema;

const DEFAULT_DATABASE_PATH: &str = "/var/lib/vnstat/vnstat.db";

#[derive(Debug, Clone, PartialEq)]
struct Database {
    path: String,
}

impl Database {
    fn new(path: &str) -> Result<Self, Error> {
        match Path::new(path).exists() {
            false => Err(Error::new(
                NotFound,
                format!("Database file [{}] is not found", path),
            )),
            true => Ok(Self {
                path: path.to_owned(),
            }),
        }
    }
    fn default() -> Result<Self, Error> {
        match Path::new(DEFAULT_DATABASE_PATH).exists() {
            false => Err(Error::new(
                NotFound,
                format!("Database file [{}] is not found", DEFAULT_DATABASE_PATH),
            )),
            true => Ok(Self {
                path: DEFAULT_DATABASE_PATH.to_owned(),
            }),
        }
    }

    fn connection(&self) -> Result<SqliteConnection, Error> {
        match SqliteConnection::establish(self.path.as_str()) {
            Err(err) => Err(Error::new(Interrupted, err)),
            Ok(conn) => Ok(conn),
        }
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
    let path = "test.db";
    fs::File::create(path).unwrap();
    assert_eq!(Database::new(path).unwrap().path, path.to_owned());
    fs::remove_file(path).unwrap();
}

#[test]
fn new_database_with_unexists_path() {
    let path = "test.db";
    assert_eq!(Database::new(path).map_err(|e| e.kind()), Err(NotFound));
}

#[test]
fn database_connection_with_default_path() {
    assert!(Database::default().is_ok())
}

#[test]

fn database_connection_with_exists_path() {
    let path = "test.db";
    fs::File::create(path).unwrap();
    assert!(Database::new(path).is_ok());
    fs::remove_file(path).unwrap();
}
