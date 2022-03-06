table! {
    problems (id) {
        id -> Int4,
        title -> Varchar,
        grade -> Int4,
        rating -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        password_hash -> Text,
        created_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    problems,
    users,
);
