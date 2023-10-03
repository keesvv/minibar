// @generated automatically by Diesel CLI.

diesel::table! {
    Beverage (id) {
        id -> Integer,
        description -> Binary,
        capacity -> Double,
        amount -> Double,
    }
}

diesel::table! {
    Beverage_Metadata (id) {
        id -> Integer,
        category -> Nullable<Text>,
        image_uri -> Nullable<Text>,
        alc_percent -> Nullable<Double>,
        packaging -> Nullable<Integer>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    Beverage,
    Beverage_Metadata,
);
