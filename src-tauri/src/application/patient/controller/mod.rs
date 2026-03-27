use crate::{
    application::patient::{
        dto::{
            create::{CreatePatient, CreatePatientInput},
            get::{GetPatientBy, GetPatientByInput},
        },
        error::{PatientError, ResponseError, UIError},
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

    pub async fn get_all(&self) -> Result<Vec<Patient>, ResponseError> {
        self.service
            .get_all()
            .await
            .map_err(|err| ResponseError::new(vec![err.into()]))
    }

    pub async fn get(&self, input: GetPatientByInput) -> Result<Option<Patient>, ResponseError> {
        let validated = GetPatientBy::new(input).map_err(|e| ResponseError::new(vec![e.into()]))?;
        self.service
            .get(validated)
            .await
            .map_err(|err| ResponseError::new(vec![err.into()]))
    }
}
