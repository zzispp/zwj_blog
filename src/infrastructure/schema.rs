// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Int4,
        #[max_length = 32]
        file_hash -> Varchar,
        file_path -> Varchar,
        upload_time -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    service_contexts (id) {
        id -> Int4,
        maintenance -> Bool,
    }
}

diesel::table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        completed -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        full_name -> Nullable<Varchar>,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    files,
    service_contexts,
    todos,
    users,
);
