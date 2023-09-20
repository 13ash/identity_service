// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 122]
        hash -> Varchar,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        last_name -> Varchar,
        created_at -> Timestamp,
    }
}
