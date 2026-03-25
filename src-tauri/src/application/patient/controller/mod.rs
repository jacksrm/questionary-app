use crate::application::patient::{
    dto::create::{CreatePatient, CreatePatientInput},
    service::PatientService,
};

pub struct PatientController {
    service: PatientService,
}

impl PatientController {
    pub fn new(service: PatientService) -> Self {
        PatientController { service }
    }

    // pub fn create(&self, input: CreatePatientInput) -> Result<Patient, PatientError> {
    //     let dto = CreatePatient::new(input);
    //     self.service.create(dto)
    // }
}
