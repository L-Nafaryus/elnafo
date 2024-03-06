// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        login -> Text,
        hashed_password -> Text,
        name -> Text,
        email -> Text,
        is_admin -> Bool,
    }
}
