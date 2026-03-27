use super::*;

impl PatientService {
    pub async fn get(&self, by: GetPatientBy) -> Result<Option<Patient>, PatientError> {
        match by {
            GetPatientBy::Id(id) => self.repo.find_by_id(&id).await,
            GetPatientBy::Cpf(cpf) => self.repo.find_by_cpf(&cpf).await,
        }
    }
}
