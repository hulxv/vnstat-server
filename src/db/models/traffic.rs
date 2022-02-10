use diesel::prelude::Queryable;

#[derive(Queryable, Debug)]
pub struct Traffic {
    pub id: i32,
    pub interface: i32,
    pub date: String,
    pub rx: i64,
    pub tx: i64,
}
