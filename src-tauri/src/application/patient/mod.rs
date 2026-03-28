use uuid::Uuid;

use crate::{
    application::patient::{
        controller::PatientController,
        dto::{
            create::CreatePatientInput, delete::DeletePatientInput, get::GetPatientByInput,
            update::UpdatePatientInput,
        },
        error::ResponseError,
        repository::sqlite::SqlitePatientRepository,
        service::PatientService,
    },
    domain::patient::Patient,
};

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

pub struct PatientModule {
    controller: PatientController,
}

impl PatientModule {
    pub async fn new() -> Self {
        let database_url = format!("sqlite://file:{}?mode=memory&cache=shared", Uuid::new_v4());
        let repo = Box::new(SqlitePatientRepository::new(&database_url).await.unwrap());

        let service = PatientService::new(repo);
        let controller = PatientController::new(service);
        PatientModule { controller }
    }
}

#[tauri::command]
pub async fn get_all(
    state: tauri::State<'_, PatientModule>,
) -> Result<Vec<Patient>, ResponseError> {
    state.controller.get_all().await
}

#[tauri::command]
pub async fn get_patient(
    input: GetPatientByInput,
    state: tauri::State<'_, PatientModule>,
) -> Result<Option<Patient>, ResponseError> {
    state.controller.get(input).await
}

#[tauri::command]
pub async fn delete(
    input: DeletePatientInput,
    state: tauri::State<'_, PatientModule>,
) -> Result<Patient, ResponseError> {
    state.controller.delete(input).await
}

#[tauri::command]
pub async fn create(
    input: CreatePatientInput,
    state: tauri::State<'_, PatientModule>,
) -> Result<(), ResponseError> {
    state.controller.create(input).await
}

#[tauri::command]
pub async fn update(
    input: UpdatePatientInput,
    state: tauri::State<'_, PatientModule>,
) -> Result<Patient, ResponseError> {
    state.controller.update(input).await
}
