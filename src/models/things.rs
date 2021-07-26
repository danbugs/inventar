use super::super::schema::things;

#[derive(Serialize, Queryable)]
pub struct Thing {
    pub thing_id: i32,
    pub thing_name: String,
}

#[derive(Deserialize, Insertable)]
#[table_name="things"]
pub struct NewThing {
    pub thing_name: String,
}