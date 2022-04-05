use diesel::sql_types::{Integer, Text};
use serde::Serialize;

#[derive(Debug, QueryableByName, Serialize, Clone)]
pub struct Info {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Text"]
    pub name: String,
    #[sql_type = "Text"]
    pub value: String,
}
