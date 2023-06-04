// @generated automatically by Diesel CLI.

diesel::table! {
    classes (id) {
        id -> Integer,
        name -> Text,
        division_id -> Integer,
    }
}

diesel::table! {
    divisions (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    entries (id) {
        id -> Integer,
        back_number -> Integer,
        horse_id -> Integer,
        show_id -> Integer,
    }
}

diesel::table! {
    horses (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    results (id) {
        id -> Integer,
        entry_id -> Integer,
        class_id -> Integer,
        placing -> Nullable<Integer>,
        payout -> Nullable<Float>,
        starting_score -> Integer,
    }
}

diesel::table! {
    riders (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        membership_date -> Nullable<Timestamp>,
        birthday -> Nullable<Timestamp>,
        phone -> Nullable<Text>,
        address -> Nullable<Text>,
        person_responsible -> Nullable<Text>,
    }
}

diesel::table! {
    scores (id) {
        id -> Integer,
        result_id -> Integer,
        content_score -> Nullable<Integer>,
        penalty -> Nullable<Integer>,
        off_pattern -> Nullable<Bool>,
    }
}

diesel::table! {
    shows (id) {
        id -> Integer,
        name -> Text,
        start_date -> Timestamp,
        location -> Nullable<Text>,
        entry_deadline -> Nullable<Timestamp>,
    }
}

diesel::joinable!(classes -> divisions (division_id));
diesel::joinable!(entries -> horses (horse_id));
diesel::joinable!(entries -> shows (show_id));
diesel::joinable!(results -> classes (class_id));
diesel::joinable!(results -> entries (entry_id));
diesel::joinable!(scores -> results (result_id));

diesel::allow_tables_to_appear_in_same_query!(
    classes,
    divisions,
    entries,
    horses,
    results,
    riders,
    scores,
    shows,
);
