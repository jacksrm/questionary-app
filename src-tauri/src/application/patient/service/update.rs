use crate::application::patient::error::PatientError;

use super::*;

impl PatientService {
    pub fn update(&mut self, dto: UpdatePatient) -> Result<Patient, PatientError> {
        let UpdatePatient {
            birth_date,
            cpf,
            id,
            name,
            phone1,
            phone2,
        } = dto;

        if let Some(ref v) = cpf {
            if self.repo.find_by_cpf(&v).is_some_and(|p| p.id != id) {
                return Err(PatientError::CpfAlreadyInUse);
            }
        }

        let mut to_update = self.repo.find_by_id(&id).unwrap().clone();

        if let Some(v) = cpf {
            to_update.cpf = v;
        }

        if let Some(v) = birth_date {
            to_update.birth_date = v;
        }

        if let Some(v) = name {
            to_update.name = v;
        }

        if let Some(v) = phone1 {
            to_update.phone1 = v;
        }

        if let Some(v) = phone2 {
            to_update.phone2 = v;
        }

        self.repo.save(&to_update)?;

        Ok(to_update)
    }
}
