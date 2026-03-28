use super::*;

#[tokio::test]
async fn should_be_able_to_update_a_patient() {
    let phone2 = "(85) 99999-9999";
    let name = "Jacson Rodrigues";

    let service = service_factory().await;
    let to_update = service
        .repo
        .find_by_id(&patient_id(0))
        .await
        .unwrap()
        .unwrap()
        .clone();
    let to_update_dto = UpdatePatient {
        id: patient_id(0),
        name: Some(name.to_string()),
        birth_date: None,
        cpf: None,
        phone1: None,
        phone2: Some(UpdatePhone2Field::Value(phone2.to_string())),
    };

    let result = service.update(to_update_dto.clone()).await.unwrap();

    assert_eq!(name, result.name);
    assert_eq!(to_update.cpf, result.cpf);
    assert_eq!(to_update.birth_date, result.birth_date);
    assert_eq!(to_update.phone1, result.phone1);
    assert_eq!(Some(phone2.to_string()), result.phone2);
}

#[tokio::test]
async fn should_return_an_error_if_cpf_is_already_registered() {
    let service = service_factory().await;
    let to_update_dto = UpdatePatient {
        id: patient_id(2),
        name: None,
        birth_date: None,
        cpf: Some(PATIENT_CPF.to_string()),
        phone1: None,
        phone2: None,
    };

    let result = service.update(to_update_dto.clone()).await.unwrap_err();

    assert_eq!(result, PatientError::CpfAlreadyInUse);
}

#[tokio::test]
async fn should_clear_the_phone2_field_if_requested() {
    let service = service_factory().await;
    let to_update_dto = UpdatePatient {
        id: patient_id(0),
        name: None,
        birth_date: None,
        cpf: None,
        phone1: None,
        phone2: Some(UpdatePhone2Field::Clear),
    };

    let result = service.update(to_update_dto.clone()).await.unwrap();

    assert_eq!(result.phone2, None);
}
