// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        code -> Varchar,
        brand -> Varchar,
        category -> Varchar,
        unit -> Varchar,
        price -> Numeric,
        stock -> Int4,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 50]
        role -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    products,
    users,
);
