table! {
    problems (id) {
        id -> Int4,
        title -> Varchar,
        grade -> Int2,
        rating -> Int2,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
