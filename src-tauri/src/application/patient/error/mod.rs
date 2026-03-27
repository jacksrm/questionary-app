use serde::Serialize;

#[derive(Debug, PartialEq)]
pub enum PatientError {
    CpfAlreadyInUse,
    NotFound,
    RepositoryError(String),
}

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    InvalidCpfField,
    InvalidNameField,
    InvalidPhone1Field,
    InvalidPhone2Field,
    InvalidBirthDateField,
    InvalidIdField,
}

#[derive(Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UIError {
    InvalidCpfField,
    InvalidNameField,
    InvalidPhone1Field,
    InvalidPhone2Field,
    InvalidBirthDateField,
    InvalidIdField,
    CpfAlreadyInUse,
    RepositoryError(String),
    PatientNotFound,
}

impl From<ValidationError> for UIError {
    fn from(error: ValidationError) -> Self {
        match error {
            ValidationError::InvalidCpfField => UIError::InvalidCpfField,
            ValidationError::InvalidNameField => UIError::InvalidNameField,
            ValidationError::InvalidPhone1Field => UIError::InvalidPhone1Field,
            ValidationError::InvalidPhone2Field => UIError::InvalidPhone2Field,
            ValidationError::InvalidBirthDateField => UIError::InvalidBirthDateField,
            ValidationError::InvalidIdField => UIError::InvalidIdField,
        }
    }
}

impl From<PatientError> for UIError {
    fn from(error: PatientError) -> Self {
        match error {
            PatientError::CpfAlreadyInUse => UIError::CpfAlreadyInUse,
            PatientError::RepositoryError(msg) => UIError::RepositoryError(msg),
            PatientError::NotFound => UIError::PatientNotFound,
        }
    }
}

#[derive(Serialize)]
pub struct ResponseError {
    pub content: Vec<UIError>,
}

impl ResponseError {
    pub fn new(errors: Vec<UIError>) -> Self {
        ResponseError { content: errors }
    }
}
