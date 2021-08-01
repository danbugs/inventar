table! {
    categories (category_id) {
        category_id -> Int4,
        category_name -> Varchar,
        category_color -> Nullable<Varchar>,
        user_id -> Int4,
    }
}

table! {
    things (thing_id) {
        thing_id -> Int4,
        thing_name -> Varchar,
        user_id -> Int4,
        category_id -> Nullable<Int4>,
        thing_description -> Nullable<Varchar>,
        thing_img -> Nullable<Varchar>,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        user_name -> Varchar,
        user_email -> Varchar,
        user_pwd_hash -> Bytea,
        user_salt -> Varchar,
        verified_email -> Bool,
        verification_code -> Varchar,
    }
}

joinable!(things -> categories (category_id));

allow_tables_to_appear_in_same_query!(
    categories,
    things,
    users,
);
