use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use tauri::State;

use crate::models::Show;
use crate::schema::shows::dsl::*;
use crate::DbConnection;

#[tauri::command]
pub fn list_shows(state: State<'_, DbConnection>) -> Result<Vec<Show>, String> {
    let mut conn = state.conn.get().map_err(|e| e.to_string())?;
    shows
        .select(Show::as_select())
        .load(&mut conn)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_show(state: State<'_, DbConnection>, show_id: i32) -> Result<Show, String> {
    let mut conn = state.conn.get().map_err(|e| e.to_string())?;
    shows
        .find(show_id)
        .select(Show::as_select())
        .first(&mut conn)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_show(state: State<'_, DbConnection>, show: Show) -> Result<(), String> {
    let mut conn = state.conn.get().map_err(|e| e.to_string())?;
    diesel::update(shows.find(show.id))
        .set(show)
        .execute(&mut conn)
        .map_err(|e| e.to_string())
        .map(|_| ())
}

#[tauri::command]
pub fn create_show(state: State<'_, DbConnection>, show_name: &str) -> Result<(), String> {
    let mut conn = state.conn.get().map_err(|e| e.to_string())?;
    diesel::insert_into(shows)
        .values(&name.eq(show_name))
        .execute(&mut conn)
        .map_err(|e| e.to_string())
        .map(|_| ())
}

#[tauri::command]
pub fn delete_show(state: State<'_, DbConnection>, show_id: i32) -> Result<(), String> {
    let mut conn = state.conn.get().map_err(|e| e.to_string())?;
    diesel::delete(shows.find(show_id))
        .execute(&mut conn)
        .map_err(|e| e.to_string())
        .map(|_| ())
}
