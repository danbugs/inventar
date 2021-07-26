table! {
    things (thing_id) {
        thing_id -> Int4,
        thing_name -> Varchar,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        user_name -> Varchar,
        user_email -> Varchar,
        user_pwd_hash -> Bytea,
        user_salt -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    things,
    users,
);
