use super::*;

#[tokio::test]
async fn should_be_able_to_create_a_patient() {
    let service = service_factory().await;
    let to_create = new_create_patient();

    service.create(to_create).await.unwrap();

    assert_eq!(
        service.repo.get_all().await.unwrap().len(),
        DB_VALID_PATIENT_COUNT + 1
    );
}

#[tokio::test]
async fn should_return_a_error_if_cpf_already_exists() {
    let service = service_factory().await;
    let to_create = new_create_patient_existing_cpf();

    let result = service.create(to_create).await.unwrap_err();

    assert_eq!(result, PatientError::CpfAlreadyInUse);
}
