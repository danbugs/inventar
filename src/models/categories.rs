use crate::{schema::categories, uid_from_uname};

#[derive(Serialize, Queryable)]
pub struct Category {
    pub category_id: i32,
    pub category_name: String,
    pub category_color: Option<String>,
    pub user_id: i32
}

#[derive(Deserialize)]
pub struct IncomingCategory {
    pub category_name: String,
    pub category_color: Option<String>,
    pub user_name: String
}

#[derive(Insertable)]
#[table_name="categories"]
pub struct NewCategory {
    category_name: String,
    category_color: Option<String>,
    user_id: i32
}

impl NewCategory {
    pub fn new(ic: IncomingCategory) -> Self {
        NewCategory {category_name: ic.category_name, category_color: ic.category_color, user_id: uid_from_uname(ic.user_name)}
    }
} 