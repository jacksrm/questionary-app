#[derive(Debug, PartialEq)]
pub enum PatientError {
    PatientNotFound,
    CpfAlreadyInUse,
    RepositoryError(String),
}
