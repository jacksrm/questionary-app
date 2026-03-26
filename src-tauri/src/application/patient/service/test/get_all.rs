use super::*;

#[test]
fn should_return_all_patients_in_the_repository() {
    let service = service_factory_many();

    let result = service.get_all();

    assert_eq!(result.unwrap().len(), 100);
}
