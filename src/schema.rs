table! {
    things (thing_id) {
        thing_id -> Int4,
        thing_name -> Varchar,
        user_id -> Int4,
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

joinable!(things -> users (user_id));

allow_tables_to_appear_in_same_query!(
    things,
    users,
);
