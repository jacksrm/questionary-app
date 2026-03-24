use chrono::NaiveDate;

use crate::application::patient::repository::in_memory::InMemoryUserRepository;

use super::*;

const PATIENT_NAME: &str = "John Doe";
const PATIENT_CPF: &str = "123.456.789-00";
const PATIENT_PHONE1: &str = "(85) 98765-4321";
const PATIENT_BIRTH_DATE: &str = "1988-02-26";
const PATIENT_BIRTH_DATE_FMT: &str = "%Y-%m-%d";

fn patient_id(n: u128) -> Uuid {
    Uuid::from_u128(n)
}

fn patient_cpf(end: u128) -> String {
    format!("123.456.789-{:02}", end)
}

fn service_factory_clean() -> PatientService {
    let repo = Box::new(InMemoryUserRepository::new());
    let service = PatientService::new(repo);

    service
}

fn service_factory_single() -> PatientService {
    let mut repo = Box::new(InMemoryUserRepository::new());
    let patient = Patient {
        id: patient_id(1),
        name: PATIENT_NAME.to_string(),
        cpf: PATIENT_CPF.to_string(),
        phone1: PATIENT_PHONE1.to_string(),
        phone2: None,
        birth_date: NaiveDate::parse_from_str(PATIENT_BIRTH_DATE, PATIENT_BIRTH_DATE_FMT).unwrap(),
    };

    repo.save(&patient).unwrap();

    let service = PatientService::new(repo);
    service
}

fn service_factory_many() -> PatientService {
    let mut repo = Box::new(InMemoryUserRepository::new());
    for n in 0..100 {
        let patient = Patient {
            id: patient_id(n),
            name: format!("{}{}", PATIENT_NAME, n),
            cpf: patient_cpf(n),
            phone1: PATIENT_PHONE1.to_string(),
            phone2: None,
            birth_date: NaiveDate::parse_from_str(PATIENT_BIRTH_DATE, PATIENT_BIRTH_DATE_FMT)
                .unwrap(),
        };

        repo.save(&patient).unwrap();
    }

    let service = PatientService::new(repo);

    service
}

mod create;
mod update;
