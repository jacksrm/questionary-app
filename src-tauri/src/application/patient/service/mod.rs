use uuid::Uuid;

use crate::application::patient::dto::{
    create::CreatePatient, get::GetPatientBy, update::UpdatePatient,
};
use crate::application::patient::error::PatientError;
use crate::application::patient::repository::PatientRepository;
use crate::domain::patient::Patient;

pub struct PatientService {
    repo: Box<dyn PatientRepository>,
}

impl PatientService {
    pub fn new(repo: Box<dyn PatientRepository>) -> Self {
        PatientService { repo }
    }
}

pub mod create;
pub mod delete;
pub mod get;
pub mod get_all;
pub mod update;

#[cfg(test)]
mod test;
