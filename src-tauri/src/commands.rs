use cotp::get_elements_with_password;
use tauri::State;

use crate::state::AppState;


// TODO We should return a more detailed error back to the frontend :)
#[tauri::command]
pub fn unlock_db(password: &str, state: State<AppState>) -> bool {
    let maybe_db = get_elements_with_password(password.to_string());
    match maybe_db {
        Ok((database, key, salt)) => {
            *state.database.lock().unwrap() = Some(database);
            *state.key.lock().unwrap() = Some(key);
            *state.salt.lock().unwrap() = Some(salt);
            return true;
        },
        Err(e) => {
            println!("{e}");
            return false;
        }
    }
}
