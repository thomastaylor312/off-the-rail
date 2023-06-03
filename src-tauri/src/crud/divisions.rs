use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use tauri::State;

use crate::models::Division;
use crate::schema::divisions::dsl::*;
use crate::DbConnection;

#[tauri::command]
pub fn list_divisions(state: State<'_, DbConnection>) -> Result<Vec<Division>, String> {
    let mut conn = state.conn.get().map_err(|e| e.to_string())?;
    divisions
        .select(Division::as_select())
        .load(&mut conn)
        .map_err(|e| e.to_string())
}

// Create update command for the divisions table
#[tauri::command]
pub fn update_division(state: State<'_, DbConnection>, division: Division) -> Result<(), String> {
    let mut conn = state.conn.get().map_err(|e| e.to_string())?;
    diesel::update(divisions.find(division.id))
        .set(division)
        .execute(&mut conn)
        .map_err(|e| e.to_string())
        .map(|_| ())
}

// Create create command for the divisions table
#[tauri::command]
pub fn create_division(state: State<'_, DbConnection>, division_name: &str) -> Result<(), String> {
    let mut conn = state.conn.get().map_err(|e| e.to_string())?;
    diesel::insert_into(divisions)
        .values(&name.eq(division_name))
        .execute(&mut conn)
        .map_err(|e| e.to_string())
        .map(|_| ())
}

// Create delete command for the divisions table
#[tauri::command]
pub fn delete_division(state: State<'_, DbConnection>, division_id: i32) -> Result<(), String> {
    let mut conn = state.conn.get().map_err(|e| e.to_string())?;
    diesel::delete(divisions.find(division_id))
        .execute(&mut conn)
        .map_err(|e| e.to_string())
        .map(|_| ())
}
