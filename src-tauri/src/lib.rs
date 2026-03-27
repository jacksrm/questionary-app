use std::sync::Mutex;

use tauri::Manager;

use crate::application::patient::{self, PatientModule};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn sum(a: usize, b: usize) -> usize {
    a + b
}

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
            greet,
            sum,
            patient::do_something_stupid,
            patient::get_all,
            patient::get_patient
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod application;
mod domain;
