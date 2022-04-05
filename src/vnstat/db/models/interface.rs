use diesel::sql_types::{Date, Integer, Nullable, Text};
use serde::Serialize;

#[derive(Debug, QueryableByName, Serialize, Clone)]
pub struct Interface {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Text"]
    pub name: String,
    #[sql_type = "Integer"]
    pub active: i32,
    #[sql_type = "Date"]
    pub created: String,
    #[sql_type = "Date"]
    pub updated: String,
    #[sql_type = "Integer"]
    pub rxcounter: i32,
    #[sql_type = "Integer"]
    pub txcounter: i32,
    #[sql_type = "Integer"]
    pub rxtotal: i32,
    #[sql_type = "Integer"]
    pub txtotal: i32,
}
