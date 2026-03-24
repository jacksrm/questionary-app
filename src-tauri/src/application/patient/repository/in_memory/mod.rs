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

    fn delete(&mut self, id: &Uuid) -> Result<Patient, PatientError> {
        let to_remove = self.data.iter().position(|p| p.id == *id);
        if to_remove.is_none() {
            return Err(PatientError::RepositoryError(
                "Couldn't remove the Patient with the specified ID".to_string(),
            ));
        }
        Ok(self.data.remove(to_remove.unwrap()))
    }

    fn find_by_id(&self, id: &Uuid) -> Option<&Patient> {
        self.data.iter().find(|p| p.id == *id)
    }

    fn find_by_cpf(&self, cpf: &str) -> Option<&Patient> {
        self.data.iter().find(|p| p.cpf == cpf)
    }

    fn find_by_name(&self, name: &str) -> Vec<&Patient> {
        self.data
            .iter()
            .filter(|p| p.name.to_lowercase().contains(&name.to_lowercase()))
            .collect()
    }

    fn get_all(&self) -> Vec<&Patient> {
        self.data.iter().collect()
    }
}

#[cfg(test)]
mod test;
