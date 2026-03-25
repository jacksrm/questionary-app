use crate::application::patient::dto::delete::DeletePatient;

use super::*;

impl PatientService {
    pub fn delete(&mut self, dto: DeletePatient) -> Result<&Patient, PatientError> {
        let DeletePatient(id) = dto;

        self.repo.delete(&id)
    }
}
