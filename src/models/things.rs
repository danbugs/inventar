use crate::{uid_from_uname};
use crate::schema::things;

#[derive(Serialize, Queryable)]
pub struct Thing {
    pub thing_id: i32,
    pub thing_name: String,
    pub user_id: i32,
    pub category_id: Option<i32>,
    pub thing_description: Option<String>
}

#[derive(Deserialize)]
pub struct IncomingThing {
    pub thing_name: String,
    pub user_name: String,
    pub category_id: Option<i32>,
    pub thing_description: Option<String>
}

#[derive(Insertable)]
#[table_name="things"]
pub struct NewThing {
    thing_name: String,
    user_id: i32,
    category_id: Option<i32>,
    thing_description: Option<String>
}

impl NewThing {
    pub fn new(it: IncomingThing) -> Self {
        NewThing {thing_name: it.thing_name, user_id: uid_from_uname(it.user_name), category_id: it.category_id, thing_description: it.thing_description}
    }
}