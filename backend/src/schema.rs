// @generated automatically by Diesel CLI.

diesel::table! {
    post_statuses (id) {
        id -> Int4,
        #[max_length = 63]
        name -> Varchar,
    }
}

diesel::table! {
    post_tags (post_id, tag_id) {
        post_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        status -> Nullable<Int4>,
        content -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        #[max_length = 63]
        name -> Varchar,
    }
}

diesel::joinable!(post_tags -> posts (post_id));
diesel::joinable!(post_tags -> tags (tag_id));
diesel::joinable!(posts -> post_statuses (status));

diesel::allow_tables_to_appear_in_same_query!(
    post_statuses,
    post_tags,
    posts,
    tags,
);
