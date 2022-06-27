mod model;
mod query;
mod schema;

use query::*;

use anyhow::{anyhow, Result};
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
pub struct InitDatabase {
    pub conn: SqliteConnection,
}

/// Initialization of the authentication database, i.e. its creation and creation of its tables
impl InitDatabase {
    pub fn connect() -> Result<Self> {
        match SqliteConnection::establish(
            &DatabaseFile::new()
                .unwrap()
                .create_if_not_exists()
                .unwrap()
                .path(),
        ) {
            Err(err) => Err(anyhow!(err)),
            Ok(conn) => Ok(Self { conn }),
        }
    }

    pub fn init(&self) -> Result<()> {
        for q in [CREATE_KEYS_QUERY, CREATE_CONNECTIONS_QUERY].iter() {
            if let Err(e) = sql_query(*q).execute(&self.conn) {
                return Err(anyhow!(e));
            }
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
            config_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap(),
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
            if let Err(e) = File::create(&self.path) {
                return Err(anyhow!(e));
            }
        }
        Ok(self)
    }
    pub fn path(&self) -> String {
        self.path.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::core::prelude::v1::test;
    use dirs::config_dir;
    use std::fs::remove_file;

    #[test]
    fn get_database_file_path() {
        let path = DatabaseFile::new().unwrap().path();
        println!("database path: {path}");
        assert_eq!(
            path,
            [
                config_dir().unwrap().to_str().unwrap(),
                "/vns/database/auth.db"
            ]
            .concat()
        )
    }
    #[test]
    fn create_database_if_not_exists() {
        DatabaseFile::new().unwrap().create_if_not_exists().unwrap();

        let path = DatabaseFile::new().unwrap().path();
        assert!(Path::new(&path).exists());
        remove_file(path).unwrap()
    }

    #[test]
    fn initlize_database() {
        use diesel::{sql_types::Text, QueryableByName};
        #[derive(Debug, QueryableByName, Clone, PartialEq)]
        struct Table {
            #[sql_type = "Text"]
            pub name: String,
        }

        let db = InitDatabase::connect().unwrap();

        db.init().unwrap();
        let tables = sql_query("SELECT name FROM sqlite_master WHERE type='table'")
            .load::<Table>(&db.conn)
            .unwrap();
        let excepted_tables = vec!["connections", "keys"];

        println!("{tables:#?}");

        for table in excepted_tables.into_iter() {
            assert!(tables.contains(&Table {
                name: table.to_owned()
            }))
        }
    }
}
