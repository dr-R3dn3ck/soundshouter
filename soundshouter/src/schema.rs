// @generated automatically by Diesel CLI.

diesel::table! {
    category (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    sound (id) {
        id -> Integer,
        name -> Text,
        path -> Text,
        duration -> Float,
        play_count -> Integer,
        category_id -> Nullable<Integer>,
        subcategory_id -> Nullable<Integer>,
    }
}

diesel::table! {
    subcategory (id) {
        id -> Integer,
        category_id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(sound -> category (category_id));
diesel::joinable!(sound -> subcategory (subcategory_id));
diesel::joinable!(subcategory -> category (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    category,
    sound,
    subcategory,
);
