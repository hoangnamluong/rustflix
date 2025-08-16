// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "maturity_rating"))]
    pub struct MaturityRating;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "streaming_protocol"))]
    pub struct StreamingProtocol;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_status"))]
    pub struct UserStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::StreamingProtocol;

    asset (id) {
        id -> Uuid,
        title_id -> Nullable<Uuid>,
        manifest_url -> StreamingProtocol,
        subtitle_locales -> Nullable<Int4>,
        audio_locales -> Nullable<Int4>,
    }
}

diesel::table! {
    casting (title_id, filmmaker_id, role_id) {
        title_id -> Uuid,
        filmmaker_id -> Uuid,
        role_id -> Uuid,
        #[max_length = 255]
        character_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    casting_role (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
    }
}

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
    filmmaker (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        bio -> Nullable<Text>,
        birth_date -> Nullable<Date>,
    }
}

diesel::table! {
    genre (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
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

diesel::table! {
    playback_progress (user_id, asset_id) {
        user_id -> Uuid,
        asset_id -> Uuid,
        position_ms -> Int4,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MaturityRating;

    profile (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        maturity_rating_max -> MaturityRating,
        #[max_length = 255]
        avatar_url -> Nullable<Varchar>,
        language -> Int4,
    }
}

diesel::table! {
    rating (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        title_id -> Nullable<Uuid>,
        score -> Int2,
        rated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MaturityRating;

    title (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        synopsis -> Text,
        release_year -> Int2,
        runtime_min -> Int2,
        age_rating -> MaturityRating,
        #[max_length = 255]
        poster_url -> Varchar,
        #[max_length = 255]
        hero_image_url -> Varchar,
        orginal_language_id -> Nullable<Int4>,
        is_active -> Bool,
    }
}

diesel::table! {
    title_genre (title_id, genre_id) {
        title_id -> Uuid,
        genre_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserStatus;

    users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        status -> UserStatus,
        created_at -> Timestamp,
        country_id -> Nullable<Int4>,
    }
}

diesel::table! {
    video_file (id) {
        id -> Uuid,
        asset_id -> Nullable<Uuid>,
        codec -> Text,
        container -> Text,
        width -> Int4,
        height -> Int4,
        bitrate_kbps -> Int4,
    }
}

diesel::table! {
    viewing_session (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        asset_id -> Nullable<Uuid>,
        started_at -> Nullable<Timestamp>,
        ended_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    watchlist_item (user_id, title_id) {
        user_id -> Uuid,
        title_id -> Uuid,
        added_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(asset -> title (title_id));
diesel::joinable!(casting -> casting_role (role_id));
diesel::joinable!(casting -> filmmaker (filmmaker_id));
diesel::joinable!(casting -> title (title_id));
diesel::joinable!(playback_progress -> asset (asset_id));
diesel::joinable!(playback_progress -> users (user_id));
diesel::joinable!(profile -> languages (language));
diesel::joinable!(profile -> users (user_id));
diesel::joinable!(rating -> title (title_id));
diesel::joinable!(rating -> users (user_id));
diesel::joinable!(title -> languages (orginal_language_id));
diesel::joinable!(title_genre -> genre (genre_id));
diesel::joinable!(title_genre -> title (title_id));
diesel::joinable!(users -> country (country_id));
diesel::joinable!(video_file -> asset (asset_id));
diesel::joinable!(viewing_session -> asset (asset_id));
diesel::joinable!(viewing_session -> users (user_id));
diesel::joinable!(watchlist_item -> title (title_id));
diesel::joinable!(watchlist_item -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    asset,
    casting,
    casting_role,
    country,
    filmmaker,
    genre,
    languages,
    playback_progress,
    profile,
    rating,
    title,
    title_genre,
    users,
    video_file,
    viewing_session,
    watchlist_item,
);
