use chrono::NaiveDate;
use sqlx::migrate::Migrator;

use super::*;

static MIGRATOR: Migrator = sqlx::migrate!();

fn test_db_url() -> String {
    format!("sqlite://file:{}?mode=memory&cache=shared", Uuid::new_v4())
}

async fn setup_test_db() -> SqlitePool {
    let db_url = test_db_url();

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect");

    // Enable foreign keys (SQLite gotcha)
    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await
        .unwrap();

    // Run migrations
    MIGRATOR.run(&pool).await.expect("Migration failed");

    // Insert seed data from ./seed_patients.sql
    sqlx::query(include_str!("./seed_patients.sql"))
        .execute(&pool)
        .await
        .expect("Failed to insert seed data");

    pool
}

#[tokio::test]
async fn should_be_able_to_get_all_users() {
    let repo = SqlitePatientRepository {
        pool: setup_test_db().await,
    };

    let patients = repo.get_all().await.unwrap();

    assert_eq!(patients.len(), 25);
}

#[tokio::test]
async fn should_be_able_to_find_user_by_id() {
    let repo = SqlitePatientRepository {
        pool: setup_test_db().await,
    };

    let patient = repo.find_by_id(&Uuid::from_u128(0)).await.unwrap().unwrap();

    assert_eq!(patient.id, Uuid::from_u128(0));
    assert_eq!(patient.name, "Patient 0");
    assert_eq!(patient.cpf, "444.896.358-69");
    assert_eq!(patient.phone1, "(85) 90000-0000");
    assert_eq!(patient.phone2, None);
    assert_eq!(
        patient.birth_date,
        NaiveDate::parse_from_str("2000-01-01", "%Y-%m-%d").unwrap()
    );
}

#[tokio::test]
async fn should_be_able_to_find_user_by_cpf() {
    let repo = SqlitePatientRepository {
        pool: setup_test_db().await,
    };

    let patient = repo.find_by_cpf("444.896.358-69").await.unwrap().unwrap();

    assert_eq!(patient.id, Uuid::from_u128(0));
    assert_eq!(patient.name, "Patient 0");
    assert_eq!(patient.cpf, "444.896.358-69");
    assert_eq!(patient.phone1, "(85) 90000-0000");
    assert_eq!(patient.phone2, None);
    assert_eq!(
        patient.birth_date,
        NaiveDate::parse_from_str("2000-01-01", "%Y-%m-%d").unwrap()
    );
}

#[tokio::test]
async fn should_be_able_to_find_user_by_name() {
    let repo = SqlitePatientRepository {
        pool: setup_test_db().await,
    };

    let patients = repo.find_by_name("Patient 1").await.unwrap();

    assert_eq!(patients.len(), 5);

    for patient in patients {
        assert!(patient.name.contains("Patient 1"));
    }
}

#[tokio::test]
async fn should_not_return_any_data_if_deleted_at_is_set() {
    let repo = SqlitePatientRepository {
        pool: setup_test_db().await,
    };

    let by_id = repo.find_by_id(&Uuid::from_u128(9)).await.unwrap();
    let by_cpf = repo.find_by_cpf("492.599.571-50").await.unwrap();
    let by_name = repo.find_by_name("Patient 15").await.unwrap();

    assert!(by_id.is_none());
    assert!(by_cpf.is_none());
    assert!(by_name.is_empty());
}

#[tokio::test]
async fn should_delete_a_patient() {
    let sql = r#"
        SELECT id, name, cpf, phone1, phone2, birth_date, created_at, updated_at, deleted_at 
        FROM patients 
        WHERE id = ?
    "#;

    let repo = SqlitePatientRepository {
        pool: setup_test_db().await,
    };

    let _ = repo.delete(&Uuid::from_u128(0)).await.unwrap();
    let row: PatientRow = sqlx::query_as(sql)
        .bind(Uuid::from_u128(0).to_string())
        .fetch_one(&repo.pool)
        .await
        .unwrap();

    let patient = Patient::try_from(row).unwrap();

    assert!(patient.deleted_at.is_some());
}

#[tokio::test]
async fn should_be_able_to_create_a_patient() {
    let repo = SqlitePatientRepository {
        pool: setup_test_db().await,
    };

    let new_patient = Patient {
        id: Uuid::new_v4(),
        name: "New Patient".to_string(),
        cpf: "123.456.789-00".to_string(),
        phone1: "(85) 91234-5678".to_string(),
        phone2: None,
        birth_date: NaiveDate::parse_from_str("1990-01-01", "%Y-%m-%d").unwrap(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
    };

    repo.save(&new_patient).await.unwrap();

    let fetched = repo.find_by_id(&new_patient.id).await.unwrap().unwrap();

    assert_eq!(fetched.id, new_patient.id);
    assert_eq!(fetched.name, new_patient.name);
    assert_eq!(fetched.cpf, new_patient.cpf);
    assert_eq!(fetched.phone1, new_patient.phone1);
    assert_eq!(fetched.phone2, new_patient.phone2);
    assert_eq!(fetched.birth_date, new_patient.birth_date);
}
