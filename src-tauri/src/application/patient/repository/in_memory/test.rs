use chrono::{NaiveDate, Utc};

use super::*;

const PATIENT_NAME: &str = "John Doe";
const PATIENT_CPF: &str = "123.456.789-00";
const PATIENT_PHONE1: &str = "(85) 98765-4321";
const PATIENT_BIRTH_DATE: &str = "1988-02-26";
const PATIENT_BIRTH_DATE_FMT: &str = "%Y-%m-%d";

fn patient_id(n: u128) -> Uuid {
    Uuid::from_u128(n)
}

fn create_db_and_setup() -> (InMemoryUserRepository, Patient) {
    let mut db = InMemoryUserRepository::new();
    let patient = Patient {
        id: patient_id(1),
        name: PATIENT_NAME.to_string(),
        cpf: PATIENT_CPF.to_string(),
        phone1: PATIENT_PHONE1.to_string(),
        phone2: None,
        birth_date: NaiveDate::parse_from_str(PATIENT_BIRTH_DATE, PATIENT_BIRTH_DATE_FMT).unwrap(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
    };

    db.data.push(patient.clone());

    (db, patient)
}

fn create_db_and_setup_deleted() -> (InMemoryUserRepository, Patient) {
    let mut db = InMemoryUserRepository::new();
    let patient = Patient {
        id: patient_id(1),
        name: PATIENT_NAME.to_string(),
        cpf: PATIENT_CPF.to_string(),
        phone1: PATIENT_PHONE1.to_string(),
        phone2: None,
        birth_date: NaiveDate::parse_from_str(PATIENT_BIRTH_DATE, PATIENT_BIRTH_DATE_FMT).unwrap(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: Utc::now().into(),
    };

    db.data.push(patient.clone());

    (db, patient)
}

fn create_db_and_setup_many() -> InMemoryUserRepository {
    let mut db = InMemoryUserRepository::new();
    for n in 0..50 {
        let patient = Patient {
            id: patient_id(n),
            name: format!("{}{}", PATIENT_NAME, n),
            cpf: PATIENT_CPF.to_string(),
            phone1: PATIENT_PHONE1.to_string(),
            phone2: None,
            birth_date: NaiveDate::parse_from_str(PATIENT_BIRTH_DATE, PATIENT_BIRTH_DATE_FMT)
                .unwrap(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        db.data.push(patient)
    }

    db
}

fn create_db_and_setup_many_deleted() -> InMemoryUserRepository {
    let mut db = InMemoryUserRepository::new();
    for n in 0..50 {
        let patient = Patient {
            id: patient_id(n),
            name: format!("{}{}", PATIENT_NAME, n),
            cpf: PATIENT_CPF.to_string(),
            phone1: PATIENT_PHONE1.to_string(),
            phone2: None,
            birth_date: NaiveDate::parse_from_str(PATIENT_BIRTH_DATE, PATIENT_BIRTH_DATE_FMT)
                .unwrap(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: if n % 2 == 0 { Some(Utc::now()) } else { None },
        };

        db.data.push(patient)
    }

    db
}

#[test]
fn should_add_a_patient_to_db() {
    let (mut db, _) = create_db_and_setup();
    let to_save = Patient {
        id: patient_id(2),
        name: "Fulano De Tal".to_string(),
        cpf: "789.456.123-01".to_string(),
        phone1: "(85) 99876-5432".to_string(),
        phone2: None,
        birth_date: NaiveDate::parse_from_str("1988-02-27", "%Y-%m-%d").unwrap(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
    };

    let result = db.save(&to_save).unwrap();

    assert_eq!(db.data.len(), 2);
    assert_eq!(result, ());
    assert_eq!(db.data[1].cpf, "789.456.123-01".to_string());
}

#[test]
fn should_delete_a_patient() {
    let (mut db, _) = create_db_and_setup();

    let result = db.delete(&patient_id(1)).unwrap();

    assert!(result.deleted_at.is_some());
    assert_eq!(db.data.len(), 1);
}

#[test]
fn should_retrieve_data_with_id() {
    let (db, _) = create_db_and_setup();
    let result = db.find_by_id(&patient_id(1));

    assert_ne!(result, None);
}

#[test]
fn should_not_retrieve_data_with_id_if_it_is_deleted() {
    let (mut db, _) = create_db_and_setup_deleted();
    let result = db.find_by_id(&patient_id(1));

    assert_eq!(result, None);
}

#[test]
fn should_retrieve_data_with_cpf() {
    let (db, _) = create_db_and_setup();
    let result = db.find_by_cpf(PATIENT_CPF);

    assert_ne!(result, None);
}

#[test]
fn should_not_retrieve_data_with_cpf_if_it_is_deleted() {
    let (mut db, _) = create_db_and_setup_deleted();
    let result = db.find_by_cpf(PATIENT_CPF);

    assert_eq!(result, None);
}

#[test]
fn should_retrieve_data_with_name() {
    let db = create_db_and_setup_many();
    let result = db.find_by_name("2");

    assert_ne!(result.len(), 15);
}

#[test]
fn should_not_retrieve_data_with_name_if_it_is_deleted() {
    let db = create_db_and_setup_many_deleted();
    let result = db.find_by_name("2");

    assert_eq!(result.len(), 5);
}

#[test]
fn should_retrieve_all_data() {
    let db = create_db_and_setup_many();
    let result = db.get_all();

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 50);
}

#[test]
fn should_not_retrieve_deleted_data() {
    let db = create_db_and_setup_many_deleted();
    let result = db.get_all();

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 25);
}
