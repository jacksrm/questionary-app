use chrono::{NaiveDate, Utc};
use uuid::Uuid;

use crate::{
    application::patient::{
        controller::PatientController,
        dto::create::CreatePatientInput,
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

fn build_get_all() -> PatientController {
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

        repo.save(&patient).unwrap();
    }

    let service = PatientService::new(repo);
    let controller = PatientController::new(service);
    controller
}

#[tauri::command]
pub fn get_all() -> Result<Vec<Patient>, ResponseError> {
    let controller = build_get_all();

    let patients = match controller.get_all() {
        Ok(patients) => patients.iter().map(|p| (*p).clone()).collect(),
        Err(e) => {
            return Err(ResponseError {
                content: vec![UIError::from(e)],
            })
        }
    };

    Ok(patients)
}

#[tauri::command]
fn create_patient(input: CreatePatientInput) -> Result<(), ResponseError> {
    Ok(())
}
