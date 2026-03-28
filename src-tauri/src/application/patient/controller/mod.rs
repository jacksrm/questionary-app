use crate::{
    application::patient::{
        dto::{
            create::{CreatePatient, CreatePatientInput},
            delete::{DeletePatient, DeletePatientInput},
            get::{GetPatientBy, GetPatientByInput},
            update::{UpdatePatient, UpdatePatientInput},
        },
        error::{ResponseError, UIError},
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

    pub async fn delete(&self, input: DeletePatientInput) -> Result<Patient, ResponseError> {
        let dto = DeletePatient::new(input).map_err(|e| ResponseError {
            content: vec![UIError::from(e)],
        })?;

        self.service.delete(dto).await.map_err(|e| ResponseError {
            content: vec![UIError::from(e)],
        })
    }

    pub async fn create(&self, input: CreatePatientInput) -> Result<(), ResponseError> {
        let dto = CreatePatient::new(input).map_err(|e| ResponseError {
            content: e.into_iter().map(|err| err.into()).collect(),
        })?;

        self.service.create(dto).await.map_err(|e| ResponseError {
            content: vec![UIError::from(e)],
        })
    }

    pub async fn update(&self, input: UpdatePatientInput) -> Result<Patient, ResponseError> {
        let dto = UpdatePatient::new(input).map_err(|e| ResponseError {
            content: e.into_iter().map(|err| err.into()).collect(),
        })?;

        self.service.update(dto).await.map_err(|e| ResponseError {
            content: vec![UIError::from(e)],
        })
    }
}
