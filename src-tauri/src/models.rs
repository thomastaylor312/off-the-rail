use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

macro_rules! create_model {
    ($t:ty, $table_path:path, {
        $( $attr_name:ident : $attr_type:ty ),*
    }) => {
        paste::paste! {
            #[derive(Queryable, Selectable, AsChangeset, Identifiable, Serialize, Deserialize)]
            #[diesel(table_name = $table_path)]
            #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
            pub struct $t {
                pub id: i32,
                $( pub $attr_name : $attr_type ),*
            }

            #[derive(Insertable, Serialize, Deserialize)]
            #[diesel(table_name = $table_path)]
            #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
            pub struct [<New $t>] {
                $( pub $attr_name : $attr_type ),*
            }
        }
    };
}

create_model!(Horse, crate::schema::horses, { name: String });

create_model!(Show, crate::schema::shows, {
    name: String,
    // NOTE: I can't seem to get DateTime<Utc> to work without a compile error and I can't seem to figure out why, so this will be good enough for now as I know I am going to be storing as UTC anyway
    start_date: NaiveDateTime,
    location: Option<String>,
    entry_deadline: Option<NaiveDateTime>
});

create_model!(Rider, crate::schema::riders, {
    name: String,
    email: String,
    membership_date: Option<NaiveDateTime>,
    birthday: Option<NaiveDateTime>,
    phone: Option<String>,
    address: Option<String>,
    person_responsible: Option<String>
});

create_model!(Entry, crate::schema::entries, {
    back_number: i32,
    horse_id: i32,
    show_id: i32
});

create_model!(Class, crate::schema::classes, {
    name: String,
    division_id: i32
});

create_model!(ShowResult, crate::schema::results, {
    entry_id: i32,
    class_id: i32,
    starting_score: i32,
    placing: Option<i32>,
    payout: Option<f32>
});

create_model!(Score, crate::schema::scores, {
    result_id: i32,
    content_score: Option<i32>,
    penalty: Option<i32>,
    off_pattern: Option<bool>
});

create_model!(Division, crate::schema::divisions, { name: String });
