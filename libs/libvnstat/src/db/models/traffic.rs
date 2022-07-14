use diesel::sql_types::{BigInt, Integer, Text};
use serde::Serialize;

#[derive(Debug, QueryableByName, Serialize, Clone)]
pub struct Traffic {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Integer"]
    pub interface: i32,
    #[sql_type = "Text"]
    pub date: String,
    #[sql_type = "BigInt"]
    pub rx: i64,
    #[sql_type = "BigInt"]
    pub tx: i64,
}
