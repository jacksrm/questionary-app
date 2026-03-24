use crate::application::patient::error::PatientError;

use super::*;

impl PatientService {
    pub fn create(&mut self, dto: CreatePatient) -> Result<(), PatientError> {
        let CreatePatient {
            birth_date,
            cpf,
            name,
            phone1,
            phone2,
        } = dto;

        if self.repo.find_by_cpf(&cpf).is_some() {
            return Err(PatientError::CpfAlreadyInUse);
        }

        let patient = Patient {
            id: Uuid::new_v4(),
            birth_date,
            cpf,
            name,
            phone1,
            phone2,
        };

        self.repo.save(&patient)?;

        Ok(())
    }
}
