use crate::uid_from_uname;
use crate::schema::things;

#[derive(Serialize, Queryable)]
pub struct Thing {
    pub thing_id: i32,
    pub thing_name: String,
    pub user_id: i32
}

#[derive(Deserialize)]
pub struct IncomingThing {
    pub thing_name: String,
    pub user_name: String
}

#[derive(Insertable)]
#[table_name="things"]
pub struct NewThing {
    thing_name: String,
    user_id: i32
}

impl NewThing {
    pub fn new(it: IncomingThing) -> Self {
        NewThing {thing_name: it.thing_name, user_id: uid_from_uname(it.user_name)}
    }
}