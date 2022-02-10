use diesel::prelude::Queryable;

#[derive(Queryable, Debug)]
pub struct Info {
    pub id: i32,
    pub name: String,
    pub value: String,
}
