use crate::application::patient::error::PatientError;

use super::*;

#[test]
fn should_be_able_to_update_a_patient() {
    let mut service = service_factory_single();
    let to_update = service.repo.find_by_id(&patient_id(1)).unwrap().clone();
    let to_update_dto = UpdatePatient {
        id: patient_id(1),
        name: Some("Jacson Rodrigues".to_string()),
        birth_date: None,
        cpf: None,
        phone1: None,
        phone2: None,
    };

    let result = service.update(to_update_dto.clone()).unwrap();

    assert_eq!(to_update_dto.name.unwrap(), result.name);
    assert_eq!(to_update.cpf, result.cpf);
    assert_eq!(to_update.birth_date, result.birth_date);
    assert_eq!(to_update.phone1, result.phone1);
    assert_eq!(to_update.phone2, result.phone2);
}

#[test]
fn should_return_an_error_if_cpf_is_already_registered() {
    let mut service = service_factory_many();
    let to_update_dto = UpdatePatient {
        id: patient_id(1),
        name: None,
        birth_date: None,
        cpf: Some(patient_cpf(5)),
        phone1: None,
        phone2: None,
    };

    let result = service.update(to_update_dto.clone()).unwrap_err();

    assert_eq!(result, PatientError::CpfAlreadyInUse);
}
