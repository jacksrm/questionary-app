use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use crate::{
    application::patient::{error::PatientError, repository::PatientRepository},
    domain::patient::Patient,
};

pub struct InMemoryUserRepository {
    data: Vec<Patient>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        InMemoryUserRepository { data: vec![] }
    }
}

#[async_trait]
impl PatientRepository for InMemoryUserRepository {
    async fn save(&mut self, patient: &Patient) -> Result<(), PatientError> {
        let patient = patient.clone();
        let existent_patient = self.data.iter_mut().find(|p| p.id == patient.id);

        if let Some(exists) = existent_patient {
            *exists = patient;
            return Ok(());
        }

        self.data.push(patient);
        Ok(())
    }

    async fn delete(&mut self, id: &Uuid) -> Result<Patient, PatientError> {
        let to_remove = self.data.iter_mut().find(|p| p.id == *id);
        if let Some(patient) = to_remove {
            patient.deleted_at = Some(Utc::now());
            return Ok(patient.clone());
        }

        Err(PatientError::RepositoryError(
            "Patient not found with provided ID".to_string(),
        ))
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Patient>, PatientError> {
        Ok(self
            .data
            .iter()
            .find(|p| p.id == *id && p.deleted_at.is_none())
            .cloned())
    }

    async fn find_by_cpf(&self, cpf: &str) -> Result<Option<Patient>, PatientError> {
        Ok(self
            .data
            .iter()
            .find(|p| p.cpf == cpf && p.deleted_at.is_none())
            .cloned())
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<Patient>, PatientError> {
        Ok(self
            .data
            .iter()
            .filter(|p| p.name == name && p.deleted_at.is_none())
            .map(|p| p.clone())
            .collect())
    }

    async fn get_all(&self) -> Result<Vec<Patient>, PatientError> {
        Ok(self
            .data
            .iter()
            .filter(|p| p.deleted_at.is_none())
            .map(|p| p.clone())
            .collect())
    }
}

#[cfg(test)]
mod test;
