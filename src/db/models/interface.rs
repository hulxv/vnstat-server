use diesel::prelude::Queryable;

#[derive(Queryable, Debug)]
pub struct Info {
    pub id: i32,
    pub name: String,
    pub alias: String,
    pub active: i32,
    pub created: String,
    pub updated: String,
    pub rxcounter: i64,
    pub txcounter: i64,
    pub rxtotal: i64,
    pub txtotal: i64,
}
