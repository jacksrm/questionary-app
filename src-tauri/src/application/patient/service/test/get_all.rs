use super::*;

#[tokio::test]
async fn should_return_all_patients_in_the_repository() {
    let service = service_factory().await;

    let result = service.get_all().await;

    assert_eq!(result.unwrap().len(), DB_VALID_PATIENT_COUNT);
}
