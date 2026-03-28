use tauri::Manager;

use crate::application::patient::{self, PatientModule};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let module = tauri::async_runtime::block_on(PatientModule::new());
            app.manage(module);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            patient::do_something_stupid,
            patient::get_all,
            patient::get_patient,
            patient::delete,
            patient::create,
            patient::update,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod application;
mod domain;
