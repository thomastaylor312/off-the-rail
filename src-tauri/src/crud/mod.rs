macro_rules! crud {
    ($t:ty, $new_type:ty, $singular_table:ident, $plural_table:ident, $schema_path:path) => {
        paste::paste! {
            use ::diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
            use ::tauri::State;

            use $crate::DbConnection;
            use $schema_path::*;

            #[::tauri::command]
            pub fn [<list_$plural_table>](state: State<'_, DbConnection>) -> Result<Vec<$t>, String> {
                let mut conn = state.conn.get().map_err(|e| e.to_string())?;
                $plural_table
                    .select($t::as_select())
                    .load(&mut conn)
                    .map_err(|e| e.to_string())
            }

            #[::tauri::command]
            pub fn [<get_$singular_table>](
                state: State<'_, DbConnection>,
                [<$singular_table _id>]: i32,
            ) -> Result<$t, String> {
                let mut conn = state.conn.get().map_err(|e| e.to_string())?;
                $plural_table
                    .find([<$singular_table _id>])
                    .select($t::as_select())
                    .first(&mut conn)
                    .map_err(|e| e.to_string())
            }

            #[::tauri::command]
            pub fn [<update_$singular_table>](
                state: State<'_, DbConnection>,
                $singular_table: $t,
            ) -> Result<(), String> {
                let mut conn = state.conn.get().map_err(|e| e.to_string())?;
                diesel::update($plural_table.find($singular_table.id))
                    .set($singular_table)
                    .execute(&mut conn)
                    .map_err(|e| e.to_string())
                    .map(|_| ())
            }

            #[::tauri::command]
            pub fn [<create_$singular_table>](
                state: State<'_, DbConnection>,
                $singular_table: $new_type,
            ) -> Result<(), String> {
                let mut conn = state.conn.get().map_err(|e| e.to_string())?;
                diesel::insert_into($plural_table)
                    .values($singular_table)
                    .execute(&mut conn)
                    .map_err(|e| e.to_string())
                    .map(|_| ())
            }

            #[::tauri::command]
            pub fn [<delete_$singular_table>](
                state: State<'_, DbConnection>,
                [<$singular_table _id>]: i32,
            ) -> Result<(), String> {
                let mut conn = state.conn.get().map_err(|e| e.to_string())?;
                diesel::delete($plural_table.find([<$singular_table _id>]))
                    .execute(&mut conn)
                    .map_err(|e| e.to_string())
                    .map(|_| ())
            }
        }
    };
}

pub mod classes {
    use diesel::ExpressionMethods;

    use crate::models::{Class, NewClass};

    crud!(Class, NewClass, class, classes, crate::schema::classes::dsl);

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
}

pub mod divisions {
    crud!(
        crate::models::Division,
        crate::models::NewDivision,
        division,
        divisions,
        crate::schema::divisions::dsl
    );
}

pub mod shows {
    crud!(
        crate::models::Show,
        crate::models::NewShow,
        show,
        shows,
        crate::schema::shows::dsl
    );
}

pub mod riders {
    crud!(
        crate::models::Rider,
        crate::models::NewRider,
        rider,
        riders,
        crate::schema::riders::dsl
    );
}

pub mod horses {
    crud!(
        crate::models::Horse,
        crate::models::NewHorse,
        horse,
        horses,
        crate::schema::horses::dsl
    );
}

pub mod entries {
    crud!(
        crate::models::Entry,
        crate::models::NewEntry,
        entry,
        entries,
        crate::schema::entries::dsl
    );
}

pub mod results {
    crud!(
        crate::models::ShowResult,
        crate::models::NewShowResult,
        result,
        results,
        crate::schema::results::dsl
    );
}

pub mod scores {
    crud!(
        crate::models::Score,
        crate::models::NewScore,
        score,
        scores,
        crate::schema::scores::dsl
    );
}
