use chrono::NaiveDate;

use crate::application::patient::{dto::create::CreatePatient, error::PatientError};

use super::*;

#[test]
fn should_be_able_to_create_a_patient() {
    let mut service = service_factory_clean();
    let to_create = CreatePatient {
        name: PATIENT_NAME.to_string(),
        cpf: PATIENT_CPF.to_string(),
        birth_date: NaiveDate::parse_from_str(PATIENT_BIRTH_DATE, PATIENT_BIRTH_DATE_FMT).unwrap(),
        phone1: PATIENT_PHONE1.to_string(),
        phone2: None,
    };

    service.create(to_create).unwrap();

    assert_eq!(service.repo.get_all().len(), 1);
}

#[test]
fn should_return_a_error_if_cpf_already_exists() {
    let mut service = service_factory_single();
    let to_create = CreatePatient {
        name: PATIENT_NAME.to_string(),
        cpf: PATIENT_CPF.to_string(),
        birth_date: NaiveDate::parse_from_str(PATIENT_BIRTH_DATE, PATIENT_BIRTH_DATE_FMT).unwrap(),
        phone1: PATIENT_PHONE1.to_string(),
        phone2: None,
    };

    let result = service.create(to_create).unwrap_err();

    assert_eq!(result, PatientError::CpfAlreadyInUse);
}
