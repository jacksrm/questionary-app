use crate::application::patient::dto::delete::DeletePatient;

use super::*;

#[tokio::test]
async fn should_delete_a_patient_from_repository() {
    let service = service_factory().await;

    let result = service.delete(DeletePatient(patient_id(0))).await.unwrap();

    assert_eq!(result.id, patient_id(0));
    assert!(result.deleted_at.is_some());
}

#[tokio::test]
async fn should_return_an_error_if_patient_does_not_exist() {
    let service = service_factory().await;
    let result = service.delete(DeletePatient(patient_id(999))).await;

    assert!(result.is_err());
}
