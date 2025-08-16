// @generated automatically by Diesel CLI.

diesel::table! {
    country (id) {
        id -> Int4,
        #[max_length = 2]
        iso -> Bpchar,
        #[max_length = 80]
        name -> Varchar,
        #[max_length = 80]
        nicename -> Varchar,
        #[max_length = 3]
        iso3 -> Nullable<Bpchar>,
        numcode -> Nullable<Int2>,
        phonecode -> Int4,
    }
}

diesel::table! {
    languages (id) {
        id -> Int4,
        #[max_length = 2]
        code -> Varchar,
        #[max_length = 100]
        language -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    country,
    languages,
);
