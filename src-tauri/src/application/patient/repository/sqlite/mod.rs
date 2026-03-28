use chrono::Utc;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use uuid::Uuid;

use crate::{
    application::patient::{
        error::PatientError,
        repository::{sqlite::mapper::PatientRow, PatientRepository},
    },
    domain::patient::Patient,
};

pub struct SqlitePatientRepository {
    pool: SqlitePool,
}

impl SqlitePatientRepository {
    pub async fn new(database_url: &str) -> Result<Self, PatientError> {
        let options = SqlitePoolOptions::new().max_connections(5);
        let pool = options.connect(database_url).await.map_err(|e| {
            PatientError::RepositoryError(format!("Failed to connect to database: {}", e))
        })?;

        // Enable foreign keys (SQLite gotcha)
        sqlx::query("PRAGMA foreign_keys = ON;")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::migrate!().run(&pool).await.map_err(|e| {
            PatientError::RepositoryError(format!("Failed to run migrations: {}", e))
        })?;

        sqlx::query(include_str!(
            "../../../../../resources/db/seed_patients.sql"
        ))
        .execute(&pool)
        .await
        .map_err(|e| PatientError::RepositoryError(format!("Failed to insert seed data: {}", e)))?;

        Ok(Self { pool })
    }
}

#[async_trait::async_trait]
impl PatientRepository for SqlitePatientRepository {
    async fn save(&self, patient: &Patient) -> Result<(), PatientError> {
        let query = r#"
            SELECT COUNT(*) FROM patients WHERE id = ?;
        "#;

        let exists: (i64,) = sqlx::query_as(query)
            .bind(patient.id.to_string())
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                PatientError::RepositoryError(format!("Failed to check patient existence: {}", e))
            })?;

        let query = if exists.0 == 0 {
            r#"
                INSERT INTO patients (id, name, cpf, phone1, phone2, birth_date, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#
        } else {
            r#"
                UPDATE patients
                SET name = ?, cpf = ?, phone1 = ?, phone2 = ?, birth_date = ?, updated_at = ?
                WHERE id = ?
            "#
        };

        sqlx::query(query)
            .bind(patient.id.to_string())
            .bind(&patient.name)
            .bind(&patient.cpf)
            .bind(&patient.phone1)
            .bind(&patient.phone2)
            .bind(patient.birth_date.to_string())
            .bind(patient.created_at.to_rfc3339())
            .bind(patient.updated_at.to_rfc3339())
            .execute(&self.pool)
            .await
            .map_err(|e| PatientError::RepositoryError(format!("Failed to save patient: {}", e)))?;

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<Patient, PatientError> {
        let query = r#"
            UPDATE patients
            SET deleted_at = ?
            WHERE id = ? AND deleted_at IS NULL
            RETURNING id, name, cpf, phone1, phone2, birth_date, created_at, updated_at, deleted_at
        "#;

        let row: PatientRow = sqlx::query_as(query)
            .bind(Utc::now().to_rfc3339())
            .bind(id.to_string())
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                PatientError::RepositoryError(format!("Failed to delete patient: {}", e))
            })?;

        Patient::try_from(row).map_err(|e| PatientError::RepositoryError(format!("{:?}", e)))
    }

    async fn get_all(&self) -> Result<Vec<Patient>, PatientError> {
        let query = r#"
            SELECT id, name, cpf, phone1, phone2, birth_date, created_at, updated_at, deleted_at
            FROM patients
            WHERE deleted_at IS NULL
        "#;

        let rows: Vec<PatientRow> =
            sqlx::query_as(query)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| {
                    PatientError::RepositoryError(format!("Failed to fetch patients: {}", e))
                })?;

        let patients = rows
            .into_iter()
            .map(Patient::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(patients)
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Patient>, PatientError> {
        let query = r#"
            SELECT id, name, cpf, phone1, phone2, birth_date, created_at, updated_at, deleted_at
            FROM patients
            WHERE id = ? AND deleted_at IS NULL
        "#;

        let row: Option<PatientRow> = sqlx::query_as(query)
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                PatientError::RepositoryError(format!("Failed to fetch patient: {}", e))
            })?;
        Ok(row.map(Patient::try_from).transpose()?)
    }

    async fn find_by_cpf(&self, cpf: &str) -> Result<Option<Patient>, PatientError> {
        let query = r#"
            SELECT id, name, cpf, phone1, phone2, birth_date, created_at, updated_at, deleted_at
            FROM patients
            WHERE cpf = ? AND deleted_at IS NULL
        "#;

        let row: Option<PatientRow> = sqlx::query_as(query)
            .bind(cpf)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                PatientError::RepositoryError(format!("Failed to fetch patient: {}", e))
            })?;
        Ok(row.map(Patient::try_from).transpose()?)
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<Patient>, PatientError> {
        let query = r#"
            SELECT id, name, cpf, phone1, phone2, birth_date, created_at, updated_at, deleted_at
            FROM patients
            WHERE name LIKE ? AND deleted_at IS NULL
        "#;

        let rows: Vec<PatientRow> = sqlx::query_as(query)
            .bind(format!("%{}%", name))
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                PatientError::RepositoryError(format!("Failed to fetch patients: {}", e))
            })?;

        let patients = rows
            .into_iter()
            .map(Patient::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(patients)
    }
}

mod mapper;

#[cfg(test)]
mod test;
