// @generated automatically by Diesel CLI.

diesel::table! {
    blog_tag_relations (blog_id, tag_id) {
        blog_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    blogs (id) {
        id -> Int4,
        #[max_length = 191]
        title -> Varchar,
        #[max_length = 191]
        slug -> Varchar,
        #[max_length = 191]
        description -> Varchar,
        body -> Text,
        #[max_length = 191]
        cover -> Nullable<Varchar>,
        #[max_length = 191]
        author -> Nullable<Varchar>,
        published -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

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
    note_tag_relations (note_id, tag_id) {
        note_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    notes (id) {
        id -> Int4,
        body -> Text,
        published -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    service_contexts (id) {
        id -> Int4,
        maintenance -> Bool,
    }
}

diesel::table! {
    snippet_tag_relations (snippet_id, tag_id) {
        snippet_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    snippets (id) {
        id -> Int4,
        #[max_length = 191]
        title -> Varchar,
        #[max_length = 191]
        slug -> Varchar,
        #[max_length = 191]
        description -> Varchar,
        body -> Text,
        published -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        #[max_length = 191]
        name -> Varchar,
        #[max_length = 191]
        slug -> Varchar,
        #[sql_name = "type"]
        #[max_length = 20]
        type_ -> Varchar,
        icon -> Nullable<Text>,
        icon_dark -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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
        #[max_length = 191]
        id -> Varchar,
        #[max_length = 191]
        name -> Nullable<Varchar>,
        #[max_length = 191]
        password -> Nullable<Varchar>,
        #[max_length = 191]
        email -> Nullable<Varchar>,
        email_verified -> Nullable<Timestamptz>,
        #[max_length = 191]
        image -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(blog_tag_relations -> blogs (blog_id));
diesel::joinable!(blog_tag_relations -> tags (tag_id));
diesel::joinable!(note_tag_relations -> notes (note_id));
diesel::joinable!(note_tag_relations -> tags (tag_id));
diesel::joinable!(snippet_tag_relations -> snippets (snippet_id));
diesel::joinable!(snippet_tag_relations -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    blog_tag_relations,
    blogs,
    files,
    note_tag_relations,
    notes,
    service_contexts,
    snippet_tag_relations,
    snippets,
    tags,
    todos,
    users,
);
