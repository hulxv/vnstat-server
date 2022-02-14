use diesel::sql_types::{Date,Text,Integer,Nullable};

#[derive(Debug,QueryableByName)]
pub struct Interface {
    #[sql_type = "Integer"] pub id: i32,
    #[sql_type = "Text"]    pub name: String,
    #[sql_type = "Integer"] pub active: i32,
    #[sql_type = "Date"]    pub created: String,
    #[sql_type = "Date"]    pub updated: String,
    #[sql_type = "Integer"] pub rxcounter: i32,
    #[sql_type = "Integer"] pub txcounter: i32,
    #[sql_type = "Integer"] pub rxtotal: i32,
    #[sql_type = "Integer"] pub txtotal: i32,
}


// id
// name
// alias
// active
// created
// updated
// rxcounter
// txcounter
// rxtotal
// txtotal