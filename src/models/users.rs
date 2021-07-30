use super::super::schema::users;
use argon2rs::argon2i_simple;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[derive(Queryable, Serialize)]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub user_email: String,
    pub user_pwd_hash: Vec<u8>,
    pub user_salt: String
}

impl User {
    pub fn equal(self, lu: LoginUser) -> bool {
        let candidate_hash = argon2i_simple(&lu.user_pwd, &self.user_salt).to_vec();
        candidate_hash == self.user_pwd_hash
    }
}

#[derive(Deserialize)]
pub struct RegisteringUser {
    pub user_name: String,
    pub user_email: String,
    pub user_pwd: String
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub user_name: String,
    pub user_pwd: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub user_name: String,
    pub user_email: String,
    user_pwd_hash: Vec<u8>,
    user_salt: String
}

impl NewUser {
    pub fn new(nu: RegisteringUser) -> Self {
        let salt: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

        let pwd = argon2i_simple(&nu.user_pwd, &salt).to_vec();
        NewUser {user_name: nu.user_name, user_email: nu.user_email, user_pwd_hash: pwd, user_salt: salt}
    }
}

