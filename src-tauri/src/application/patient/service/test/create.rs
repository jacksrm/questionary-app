use super::*;

#[tokio::test]
async fn should_be_able_to_create_a_patient() {
    let mut service = service_factory_clean();
    let to_create = CreatePatient {
        name: PATIENT_NAME.to_string(),
        cpf: PATIENT_CPF.to_string(),
        birth_date: NaiveDate::parse_from_str(PATIENT_BIRTH_DATE, PATIENT_BIRTH_DATE_FMT).unwrap(),
        phone1: PATIENT_PHONE1.to_string(),
        phone2: None,
    };

    service.create(to_create).await.unwrap();

    assert_eq!(service.repo.get_all().await.unwrap().len(), 1);
}

#[tokio::test]
async fn should_return_a_error_if_cpf_already_exists() {
    let mut service = service_factory_single().await;
    let to_create = CreatePatient {
        name: PATIENT_NAME.to_string(),
        cpf: PATIENT_CPF.to_string(),
        birth_date: NaiveDate::parse_from_str(PATIENT_BIRTH_DATE, PATIENT_BIRTH_DATE_FMT).unwrap(),
        phone1: PATIENT_PHONE1.to_string(),
        phone2: None,
    };

    let result = service.create(to_create).await.unwrap_err();

    assert_eq!(result, PatientError::CpfAlreadyInUse);
}
