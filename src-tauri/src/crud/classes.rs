use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use tauri::State;

use crate::models::Class;
use crate::schema::classes::dsl::*;
use crate::DbConnection;

#[tauri::command]
pub fn list_classes(state: State<'_, DbConnection>) -> Result<Vec<Class>, String> {
    let mut conn = state.conn.get().map_err(|e| e.to_string())?;
    classes
        .select(Class::as_select())
        .load(&mut conn)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_class(state: State<'_, DbConnection>, class: Class) -> Result<(), String> {
    let mut conn = state.conn.get().map_err(|e| e.to_string())?;
    diesel::update(classes.find(class.id))
        .set(class)
        .execute(&mut conn)
        .map_err(|e| e.to_string())
        .map(|_| ())
}

#[tauri::command]
pub fn create_class(state: State<'_, DbConnection>, class_name: &str) -> Result<(), String> {
    let mut conn = state.conn.get().map_err(|e| e.to_string())?;
    diesel::insert_into(classes)
        .values(&name.eq(class_name))
        .execute(&mut conn)
        .map_err(|e| e.to_string())
        .map(|_| ())
}

#[tauri::command]
pub fn delete_class(state: State<'_, DbConnection>, class_id: i32) -> Result<(), String> {
    let mut conn = state.conn.get().map_err(|e| e.to_string())?;
    diesel::delete(classes.find(class_id))
        .execute(&mut conn)
        .map_err(|e| e.to_string())
        .map(|_| ())
}

#[tauri::command]
pub fn list_classes_for_division(
    state: State<'_, DbConnection>,
    division: i32,
) -> Result<Vec<Class>, String> {
    let mut conn = state.conn.get().map_err(|e| e.to_string())?;
    classes
        .filter(division_id.eq(division))
        .select(Class::as_select())
        .load(&mut conn)
        .map_err(|e| e.to_string())
}
