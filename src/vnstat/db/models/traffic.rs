use diesel::sql_types::{Integer, Text};
use serde::Serialize;

#[derive(Debug, QueryableByName, Serialize, Clone)]
pub struct Traffic {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Integer"]
    pub interface: i32,
    #[sql_type = "Text"]
    pub date: String,
    #[sql_type = "Integer"]
    pub rx: i32,
    #[sql_type = "Integer"]
    pub tx: i32,
}
