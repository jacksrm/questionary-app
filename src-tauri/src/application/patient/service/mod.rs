use uuid::Uuid;

use crate::application::patient::repository::PatientRepository;
use crate::domain::patient::Patient;
use crate::dto::patient::{CreatePatient, UpdatePatient};

pub struct PatientService {
    repo: Box<dyn PatientRepository>,
}

impl PatientService {
    pub fn new(repo: Box<dyn PatientRepository>) -> Self {
        PatientService { repo }
    }
}

pub mod create;
pub mod update;

#[cfg(test)]
mod test;
