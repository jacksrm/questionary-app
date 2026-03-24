use uuid::Uuid;

use crate::{application::patient::error::PatientError, domain::patient::Patient};

pub trait PatientRepository {
    fn save(&mut self, patient: &Patient) -> Result<(), PatientError>;
    fn delete(&mut self, id: &Uuid) -> Result<Patient, PatientError>;
    fn get_all(&self) -> Vec<&Patient>;
    fn find_by_id(&self, id: &Uuid) -> Option<&Patient>;
    fn find_by_cpf(&self, cpf: &str) -> Option<&Patient>;
    fn find_by_name(&self, name: &str) -> Vec<&Patient>;
}

pub mod in_memory;
