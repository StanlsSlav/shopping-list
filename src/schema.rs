// @generated automatically by Diesel CLI.

diesel::table! {
    billing_types (guid) {
        guid -> Text,
        name -> Text,
    }
}

diesel::table! {
    products (guid) {
        guid -> Text,
        name -> Text,
        price -> Float4,
        billing_type -> Text,
        qr_code -> Text,
    }
}

diesel::table! {
    roles (guid) {
        guid -> Text,
    }
}

diesel::table! {
    shopping_carts (guid) {
        guid -> Text,
    }
}

diesel::table! {
    shopping_lists (guid) {
        guid -> Text,
    }
}

diesel::table! {
    users (guid) {
        guid -> Text,
    }
}

diesel::joinable!(products -> billing_types (billing_type));

diesel::allow_tables_to_appear_in_same_query!(
    billing_types,
    products,
    roles,
    shopping_carts,
    shopping_lists,
    users,
);
