use chrono::NaiveDate;

use crate::application::patient::{
    dto::update::UpdatePhone2Field, repository::sqlite::SqlitePatientRepository,
};

use super::*;

const PATIENT_NAME: &str = "Patient 0";
const PATIENT_CPF: &str = "444.896.358-69";
const PATIENT_CPF_NON_EXISTENT: &str = "123.456.789-09";
const PATIENT_PHONE1: &str = "(85) 90000-0000";
const PATIENT_BIRTH_DATE: &str = "2000-01-01";
const PATIENT_BIRTH_DATE_FMT: &str = "%Y-%m-%d";

const DB_VALID_PATIENT_COUNT: usize = 25;
const _DB_TOTAL_PATIENT_COUNT: usize = 50;

fn patient_id(n: u128) -> Uuid {
    Uuid::from_u128(n)
}

fn new_create_patient() -> CreatePatient {
    CreatePatient {
        name: "Patient 50".to_string(),
        cpf: PATIENT_CPF_NON_EXISTENT.to_string(),
        birth_date: NaiveDate::parse_from_str(PATIENT_BIRTH_DATE, PATIENT_BIRTH_DATE_FMT).unwrap(),
        phone1: PATIENT_PHONE1.to_string(),
        phone2: None,
    }
}

fn new_create_patient_existing_cpf() -> CreatePatient {
    CreatePatient {
        name: PATIENT_NAME.to_string(),
        cpf: PATIENT_CPF.to_string(),
        birth_date: NaiveDate::parse_from_str(PATIENT_BIRTH_DATE, PATIENT_BIRTH_DATE_FMT).unwrap(),
        phone1: PATIENT_PHONE1.to_string(),
        phone2: None,
    }
}

fn database_url() -> String {
    format!("sqlite://file:{}?mode=memory&cache=shared", Uuid::new_v4())
}

async fn service_factory() -> PatientService {
    let repo = Box::new(SqlitePatientRepository::new(&database_url()).await.unwrap());
    let service = PatientService::new(repo);

    service
}

mod create;
mod delete;
mod get;
mod get_all;
mod update;
