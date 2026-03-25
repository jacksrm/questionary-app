use uuid::Uuid;

use crate::application::patient::dto::{create::CreatePatient, UpdatePatient};
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
pub mod get;
pub mod get_all;
pub mod update;

#[cfg(test)]
mod test;
