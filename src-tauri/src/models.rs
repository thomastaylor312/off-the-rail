use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, AsChangeset, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::horses)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Horse {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::shows)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Show {
    pub id: i32,
    pub name: String,
    // NOTE: I can't seem to get DateTime<Utc> to work without a compile error and I can't seem to figure out why, so this will be good enough for now as I know I am going to be storing as UTC anyway
    pub start_date: NaiveDateTime,
    pub location: Option<String>,
    pub entry_deadline: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::riders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Rider {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub membership_date: Option<NaiveDateTime>,
    pub birthday: Option<NaiveDateTime>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub person_responsible: Option<String>,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::entries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Entry {
    pub id: i32,
    pub back_number: i32,
    pub horse_id: i32,
    pub show_id: i32,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::classes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Class {
    pub id: i32,
    pub name: String,
    pub division_id: i32,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::results)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ShowResult {
    pub id: i32,
    pub entry_id: i32,
    pub class_id: i32,
    pub placing: Option<i32>,
    pub payout: Option<f32>,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::scores)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Score {
    pub id: i32,
    pub result_id: i32,
    pub starting_score: i32,
    pub content_score: Option<i32>,
    pub penalty: Option<i32>,
    pub off_pattern: Option<bool>,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::divisions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Division {
    pub id: i32,
    pub name: String,
}
