use super::*;

impl PatientService {
    pub async fn get_all(&self) -> Result<Vec<Patient>, PatientError> {
        let patients = self.repo.get_all().await?;

        Ok(patients)
    }
}
