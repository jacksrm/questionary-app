use uuid::Uuid;

use crate::{application::patient::error::PatientError, domain::patient::Patient};

#[async_trait::async_trait]
pub trait PatientRepository: Send + Sync {
    async fn save(&self, patient: &Patient) -> Result<(), PatientError>;
    async fn delete(&self, id: &Uuid) -> Result<Patient, PatientError>;
    async fn get_all(&self) -> Result<Vec<Patient>, PatientError>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Patient>, PatientError>;
    async fn find_by_cpf(&self, cpf: &str) -> Result<Option<Patient>, PatientError>;
    async fn find_by_name(&self, name: &str) -> Result<Vec<Patient>, PatientError>;
}

pub mod sqlite;
