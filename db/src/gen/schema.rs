// @generated automatically by Diesel CLI.

diesel::table! {
    beverage (id) {
        id -> Text,
        description -> Text,
        capacity -> Float,
        amount -> Float,
    }
}

diesel::table! {
    beverage_meta (id) {
        id -> Text,
        category -> Nullable<Text>,
        image_uri -> Nullable<Text>,
        alc_percent -> Nullable<Double>,
        packaging -> Nullable<Integer>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    beverage,
    beverage_meta,
);
