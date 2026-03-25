use crate::application::patient::dto::delete::DeletePatient;

use super::*;

#[test]
fn should_delete_a_patient_from_repository() {
    let mut service = service_factory_many();

    let result = service.delete(DeletePatient(patient_id(1))).unwrap();

    assert_eq!(result.id, patient_id(1));
    assert!(result.deleted_at.is_some());
}

#[test]
fn should_return_an_error_if_patient_does_not_exist() {
    let mut service = service_factory_single();
    let result = service.delete(DeletePatient(patient_id(999)));

    assert!(result.is_err());
}
