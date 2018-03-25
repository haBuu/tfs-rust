table! {
    contents (id) {
        id -> Integer,
        page_id -> Integer,
        user_id -> Integer,
        content -> Text,
        version -> Integer,
        created -> Datetime,
        comment -> Nullable<Text>,
    }
}

table! {
    pages (id) {
        id -> Integer,
        name -> Varchar,
        user_id -> Integer,
        rank -> Integer,
        top_level -> Bool,
        removed -> Bool,
        created -> Datetime,
        modified -> Nullable<Datetime>,
    }
}

table! {
    posts (id) {
        id -> Integer,
        created -> Datetime,
        user_id -> Integer,
        modified -> Nullable<Datetime>,
        user_modified_id -> Nullable<Integer>,
        title -> Text,
        content -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        password -> Varchar,
        admin -> Bool,
        superadmin -> Bool,
        created -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(
    contents,
    pages,
    posts,
    users,
);
