use super::schema::things;

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Thing {
    pub thing_id: i32,
    pub thing_name: String,
}

#[derive(Insertable, Deserialize)]
#[table_name="things"]
pub struct NewThing {
    pub thing_name: String,
}