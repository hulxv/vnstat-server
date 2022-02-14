use diesel::sql_types::{Text,Integer};

#[derive(Debug,QueryableByName)]
pub struct Info {
    #[sql_type = "Integer"] pub id: i32,
    #[sql_type = "Text"]    pub name: String,
    #[sql_type = "Text"]    pub value: String,
}
