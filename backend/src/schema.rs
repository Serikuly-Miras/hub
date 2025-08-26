// @generated automatically by Diesel CLI.

diesel::table! {
    blog_statuses (id) {
        id -> Int4,
        #[max_length = 63]
        name -> Varchar,
    }
}

diesel::table! {
    blog_tags (blog_id, tag_id) {
        blog_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    blogs (id) {
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

diesel::joinable!(blog_tags -> blogs (blog_id));
diesel::joinable!(blog_tags -> tags (tag_id));
diesel::joinable!(blogs -> blog_statuses (status));

diesel::allow_tables_to_appear_in_same_query!(
    blog_statuses,
    blog_tags,
    blogs,
    tags,
);
