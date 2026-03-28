use super::*;

#[tokio::test]
async fn should_return_a_patient_by_id() {
    let service = service_factory().await;
    let patient_id = patient_id(0);

    let patient = service.get(GetPatientBy::Id(patient_id)).await.unwrap();

    assert!(patient.is_some());
    assert_eq!(patient.unwrap().id, patient_id);
}

#[tokio::test]
async fn should_return_a_patient_by_cpf() {
    let service = service_factory().await;
    let patient_cpf = PATIENT_CPF.to_string();

    let patient = service
        .get(GetPatientBy::Cpf(patient_cpf.clone()))
        .await
        .unwrap();

    assert!(patient.is_some());
    assert_eq!(patient.unwrap().cpf, patient_cpf);
}

#[tokio::test]
async fn should_return_none_if_patient_not_found_by_id_or_cpf() {
    let service = service_factory().await;
    let non_existent_id = patient_id(999);
    let non_existent_cpf = PATIENT_CPF_NON_EXISTENT.to_string();

    let patient_by_id = service
        .get(GetPatientBy::Id(non_existent_id))
        .await
        .unwrap();
    let patient_by_cpf = service
        .get(GetPatientBy::Cpf(non_existent_cpf))
        .await
        .unwrap();

    assert!(patient_by_id.is_none());
    assert!(patient_by_cpf.is_none());
}
