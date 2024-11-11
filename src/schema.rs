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
        name -> Text,
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
        product -> Nullable<Text>,
        user -> Nullable<Text>,
        amount -> Nullable<Int4>,
    }
}

diesel::table! {
    users (guid) {
        guid -> Text,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        password -> Text,
    }
}

diesel::joinable!(products -> billing_types (billing_type));
diesel::joinable!(shopping_lists -> products (product));
diesel::joinable!(shopping_lists -> users (user));

diesel::allow_tables_to_appear_in_same_query!(
    billing_types,
    products,
    roles,
    shopping_carts,
    shopping_lists,
    users,
);
