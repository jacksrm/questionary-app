use chrono::{NaiveDate, Utc};
use uuid::Uuid;

use crate::{
    application::patient::{
        controller::PatientController,
        dto::{
            create::CreatePatientInput,
            get::{GetPatientBy, GetPatientByInput},
        },
        error::{ResponseError, UIError},
        repository::{in_memory::InMemoryUserRepository, PatientRepository},
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
        let mut repo = Box::new(InMemoryUserRepository::new());

        for n in 0..100 {
            let patient = Patient {
                id: Uuid::from_u128(n),
                name: format!("Geraldo Mendonça{}", n),
                cpf: format!("123.456.789-{:02}", n),
                phone1: "(85) 99999-9999".to_string(),
                phone2: Some("(85) 99999-9999".to_string()),
                birth_date: NaiveDate::parse_from_str("1980-01-01", "%Y-%m-%d").unwrap(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            };
            repo.save(&patient).await.unwrap();
        }

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
