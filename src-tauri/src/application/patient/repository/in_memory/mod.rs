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

impl PatientRepository for InMemoryUserRepository {
    fn save(&mut self, patient: &Patient) -> Result<(), PatientError> {
        let patient = patient.clone();
        let existent_patient = self.data.iter_mut().find(|p| p.id == patient.id);

        if let Some(exists) = existent_patient {
            *exists = patient;
            return Ok(());
        }

        self.data.push(patient);
        Ok(())
    }

    fn delete(&mut self, id: &Uuid) -> Result<&Patient, PatientError> {
        let to_remove = self.data.iter_mut().find(|p| p.id == *id);
        if let Some(patient) = to_remove {
            patient.deleted_at = Some(Utc::now());
            return Ok(patient);
        }

        Err(PatientError::RepositoryError(
            "Patient not found with provided ID".to_string(),
        ))
    }

    fn find_by_id(&self, id: &Uuid) -> Option<&Patient> {
        self.data
            .iter()
            .find(|p| p.id == *id && p.deleted_at.is_none())
    }

    fn find_by_cpf(&self, cpf: &str) -> Option<&Patient> {
        self.data
            .iter()
            .find(|p| p.cpf == cpf && p.deleted_at.is_none())
    }

    fn find_by_name(&self, name: &str) -> Vec<&Patient> {
        self.data
            .iter()
            .filter(|p| {
                p.name.to_lowercase().contains(&name.to_lowercase()) && p.deleted_at.is_none()
            })
            .collect()
    }

    fn get_all(&self) -> Vec<&Patient> {
        self.data
            .iter()
            .filter(|p| p.deleted_at.is_none())
            .collect()
    }
}

#[cfg(test)]
mod test;
