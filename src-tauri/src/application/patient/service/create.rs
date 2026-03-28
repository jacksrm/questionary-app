use super::*;

impl PatientService {
    pub async fn create(&self, dto: CreatePatient) -> Result<(), PatientError> {
        let CreatePatient {
            birth_date,
            cpf,
            name,
            phone1,
            phone2,
        } = dto;

        if self.repo.find_by_cpf(&cpf).await?.is_some() {
            return Err(PatientError::CpfAlreadyInUse);
        }

        let patient = Patient {
            id: Uuid::new_v4(),
            birth_date,
            cpf,
            name,
            phone1,
            phone2,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        self.repo.save(&patient).await?;

        Ok(())
    }
}
