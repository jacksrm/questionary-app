use super::*;

impl PatientService {
    pub async fn delete(&mut self, dto: DeletePatient) -> Result<Patient, PatientError> {
        let DeletePatient(id) = dto;

        self.repo.delete(&id).await
    }
}
