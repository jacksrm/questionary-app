use crate::application::patient::dto::create::CreatePatientInput;

mod controller;
mod dto;
mod error;
mod repository;
mod service;
mod validation;

#[tauri::command]
pub fn do_something_stupid(input: CreatePatientInput) {
    println!("Doing something stupid with input: {:?}", input);
}
