#[derive(Debug, PartialEq)]
pub enum PatientError {
    CpfAlreadyInUse,
    RepositoryError(String),
}

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    InvalidCpfField,
    InvalidNameField,
    InvalidPhone1Field,
    InvalidPhone2Field,
    InvalidBirthDateField,
}
