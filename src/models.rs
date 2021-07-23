use super::schema::things;

#[derive(Queryable)]
pub struct Thing {
    pub thing_id: i32,
    pub thing_name: String,
}

#[derive(Insertable)]
#[table_name="things"]
pub struct NewThing {
    pub thing_name: String,
}