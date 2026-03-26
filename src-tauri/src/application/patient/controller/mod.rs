use crate::{
    application::patient::{
        dto::create::{CreatePatient, CreatePatientInput},
        error::PatientError,
        service::PatientService,
    },
    domain::patient::Patient,
};

pub struct PatientController {
    service: PatientService,
}

impl PatientController {
    pub fn new(service: PatientService) -> Self {
        PatientController { service }
    }

    pub fn get_all(&self) -> Result<Vec<&Patient>, PatientError> {
        self.service.get_all()
    }
}
